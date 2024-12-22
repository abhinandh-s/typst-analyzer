use std::collections::HashMap;

use dashmap::DashMap;
use regex::Regex;
use serde_json::Value;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer};

use crate::completion::handle::handle_completions;

#[derive(Debug)]
pub struct Backend {
    pub client: Client,
    pub document: DashMap<String, String>,
}

pub trait TypstInlayHints {
    fn calculate_inlay_hints(&self, doc: &str) -> Vec<InlayHint>;
}

pub trait TypstDiagnostic {
    fn check_unclosed_delimiters(&self, content: &str) -> Vec<Diagnostic>;
}

pub trait TypstCodeActions {
    fn calculate_code_actions(
        &self,
        content: &str,
        range: Range,
        uri: Url,
    ) -> Vec<CodeActionOrCommand>;
}

impl TypstInlayHints for Backend {
    fn calculate_inlay_hints(&self, doc: &str) -> Vec<InlayHint> {
        let mut hints = Vec::new();

        // Regex to match any word within angle brackets and @word
        let angle_brackets_re = Regex::new(r"<(\w+)>").unwrap();
        let at_word_re = Regex::new(r"@(\w+)").unwrap();

        for (line_idx, line) in doc.lines().enumerate() {
            // Match words within angle brackets
            for cap in angle_brackets_re.captures_iter(line) {
                if let Some(matched_word) = cap.get(1) {
                    let start = cap.get(0).unwrap().start();
                    hints.push(InlayHint {
                        position: Position {
                            line: line_idx as u32,
                            character: start as u32 + 1,
                        },
                        label: InlayHintLabel::String("label".to_owned()),
                        kind: Some(InlayHintKind::TYPE),
                        text_edits: None,
                        tooltip: Some(InlayHintTooltip::String(format!(
                            "Suggested label for <{}>",
                            matched_word.as_str()
                        ))),
                        padding_left: Some(true),
                        padding_right: Some(true),
                        data: None,
                    });
                }
            }

            // Match @word patterns
            for cap in at_word_re.captures_iter(line) {
                if let Some(matched_word) = cap.get(1) {
                    let start = cap.get(0).unwrap().start();
                    hints.push(InlayHint {
                        position: Position {
                            line: line_idx as u32,
                            character: start as u32 + 1,
                        },
                        label: InlayHintLabel::String("reference".to_owned()),
                        kind: Some(InlayHintKind::TYPE),
                        text_edits: None,
                        tooltip: Some(InlayHintTooltip::String(format!(
                            "Reference for @{}",
                            matched_word.as_str()
                        ))),
                        padding_left: Some(true),
                        padding_right: Some(true),
                        data: None,
                    });
                }
            }
        }
        hints
    }
}

impl Backend {
    fn position_to_offset(&self, text: &str, position: Position) -> Option<usize> {
        let mut offset = 0;
        for (line_idx, line) in text.lines().enumerate() {
            if line_idx == position.line as usize {
                return Some(offset + position.character as usize);
            }
            offset += line.len() + 1; // +1 for the newline character
        }
        None
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            server_info: None,
            capabilities: ServerCapabilities {
                inlay_hint_provider: Some(OneOf::Left(true)),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                code_action_provider: Some(CodeActionProviderCapability::Simple(true)),
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::INCREMENTAL,
                )),
                completion_provider: Some(CompletionOptions {
                    resolve_provider: Some(false),
                    trigger_characters: Some(vec![".".to_owned()]),
                    work_done_progress_options: Default::default(),
                    all_commit_characters: None,
                    ..Default::default()
                }),
                execute_command_provider: Some(ExecuteCommandOptions {
                    commands: vec!["dummy.do_something".to_owned()],
                    work_done_progress_options: Default::default(),
                }),
                workspace: Some(WorkspaceServerCapabilities {
                    workspace_folders: Some(WorkspaceFoldersServerCapabilities {
                        supported: Some(true),
                        change_notifications: Some(OneOf::Left(true)),
                    }),
                    file_operations: None,
                }),
                ..ServerCapabilities::default()
            },
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "Language Server initialized!")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_change_workspace_folders(&self, _: DidChangeWorkspaceFoldersParams) {
        self.client
            .log_message(MessageType::INFO, "Workspace folders changed!")
            .await;
    }

    async fn did_change_configuration(&self, _: DidChangeConfigurationParams) {
        self.client
            .log_message(MessageType::INFO, "Configuration changed!")
            .await;
    }

    async fn did_change_watched_files(&self, _: DidChangeWatchedFilesParams) {
        self.client
            .log_message(MessageType::INFO, "Watched files have changed!")
            .await;
    }

    async fn execute_command(&self, _: ExecuteCommandParams) -> Result<Option<Value>> {
        self.client
            .log_message(MessageType::INFO, "Command executed!")
            .await;

        match self.client.apply_edit(WorkspaceEdit::default()).await {
            Ok(res) if res.applied => {
                self.client
                    .log_message(MessageType::INFO, "Edit applied")
                    .await
            }
            Ok(_) => {
                self.client
                    .log_message(MessageType::INFO, "Edit rejected")
                    .await
            }
            Err(err) => self.client.log_message(MessageType::ERROR, err).await,
        }

        Ok(None)
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let text_document = params.text_document;
        self.document
            .insert(text_document.uri.to_string(), text_document.text);
        self.client
            .log_message(
                MessageType::INFO,
                format!("Opened file: {}", text_document.uri),
            )
            .await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri.to_string();
        if let Some(mut doc) = self.document.get_mut(&uri) {
            for change in params.content_changes {
                // Apply changes incrementally
                let mut current_text = doc.value().clone();

                // Apply the text edit
                if let Some(range) = change.range {
                    let start = range.start;
                    let end = range.end;

                    let start_idx = self.position_to_offset(&current_text, start).unwrap_or(0);
                    let end_idx = self
                        .position_to_offset(&current_text, end)
                        .unwrap_or(current_text.len());

                    current_text.replace_range(start_idx..end_idx, &change.text);
                } else {
                    // If range is None, replace the whole text
                    current_text = change.text.clone();
                }

                *doc.value_mut() = current_text;
            }
            // Check for unclosed delimiters
            let diagnostics = self.check_unclosed_delimiters(&doc);
            self.client
                .publish_diagnostics(params.text_document.uri.clone(), diagnostics, None)
                .await;

            self.client
                .log_message(MessageType::INFO, "File changed!")
                .await;
        }
    }

    async fn did_save(&self, _: DidSaveTextDocumentParams) {
        self.client
            .log_message(MessageType::INFO, "File saved!")
            .await;
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        let uri = params.text_document.uri.to_string();
        self.document.remove(&uri);
        self.client
            .log_message(MessageType::INFO, format!("Closed file: {}", uri))
            .await;
    }

    async fn completion(&self, _: CompletionParams) -> Result<Option<CompletionResponse>> {
        Ok(Some(CompletionResponse::Array(handle_completions())))
    }

    async fn hover(&self, _params: HoverParams) -> Result<Option<Hover>> {
        Ok(Some(Hover {
            contents: HoverContents::Markup(MarkupContent {
                kind: MarkupKind::Markdown,
                value: "# will this get displayed\nyes it will".to_owned(),
            }),
            range: None,
        }))
    }

    async fn inlay_hint(&self, params: InlayHintParams) -> Result<Option<Vec<InlayHint>>> {
        let uri = params.text_document.uri.to_string();
        if let Some(doc) = self.document.get(&uri) {
            let hints = self.calculate_inlay_hints(&doc);
            return Ok(Some(hints));
        }
        Ok(None)
    }

    /*    // code action for each params for table function
    #table(
        columns: auto,
        rows: auto,
        gutter: auto,
        column-gutter: auto,
        row-gutter: auto,
        fill: none,
        align: auto,
        stroke: none,
        inset: relative,
    )
        */
    async fn code_action(&self, params: CodeActionParams) -> Result<Option<CodeActionResponse>> {
        let uri = params.text_document.uri.to_string();
        self.client
            .log_message(MessageType::INFO, "Code action requested")
            .await;

        if let Some(doc) = self.document.get(&uri) {
            let content = doc.value();
            let range = params.range;

            let actions = self.calculate_code_actions(content, range, params.text_document.uri);

            if !actions.is_empty() {
                self.client
                    .log_message(MessageType::INFO, "Code actions generated")
                    .await;
                return Ok(Some(actions));
            }
        }
        Ok(None)
    }
}

impl TypstCodeActions for Backend {
    fn calculate_code_actions(
        &self,
        content: &str,
        range: Range,
        uri: Url,
    ) -> Vec<CodeActionOrCommand> {
        let mut actions = Vec::new();

        // Check if the text "VS Code" is within the range
        let vs_code_re = Regex::new(r"VS Code").unwrap();

        for (line_idx, line) in content.lines().enumerate() {
            if let Some(vs_code_match) = vs_code_re.find(line) {
                let start = vs_code_match.start();
                let end = vs_code_match.end();

                // Ensure the match is within the specified range
                if line_idx == range.start.line as usize && line_idx == range.end.line as usize {
                    let edit = TextEdit {
                        range: Range {
                            start: Position {
                                line: line_idx as u32,
                                character: start as u32,
                            },
                            end: Position {
                                line: line_idx as u32,
                                character: end as u32,
                            },
                        },
                        new_text: "Neovim".to_owned(),
                    };

                    let workspace_edit = WorkspaceEdit {
                        changes: Some(HashMap::from([(uri.clone(), vec![edit])])),
                        document_changes: None,
                        change_annotations: None,
                    };

                    let code_action = CodeAction {
                        title: "Replace 'VS Code' with 'Neovim'".to_owned(),
                        kind: Some(CodeActionKind::QUICKFIX),
                        diagnostics: None,
                        edit: Some(workspace_edit),
                        command: None,
                        is_preferred: Some(true),
                        disabled: None,
                        data: None,
                    };

                    // Wrap CodeAction in CodeActionOrCommand
                    actions.push(CodeActionOrCommand::CodeAction(code_action));
                }
            }
        }
        actions
    }
}

impl TypstDiagnostic for Backend {
    fn check_unclosed_delimiters(&self, content: &str) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        let mut stack = Vec::new();
        let delimiters = [("(", ")"), ("{", "}"), ("[", "]")];
        let mut line_offset = 0;

        for (line_idx, line) in content.lines().enumerate() {
            for (char_idx, ch) in line.chars().enumerate() {
                for &(open, close) in &delimiters {
                    if ch.to_string() == open {
                        stack.push((open, close, line_idx, char_idx));
                    } else if ch.to_string() == close {
                        if let Some((_, _, _, _)) = stack.pop() {
                            // Pair found, no action needed
                        } else {
                            // Unmatched closing delimiter
                            diagnostics.push(Diagnostic {
                                range: Range {
                                    start: Position {
                                        line: line_idx as u32,
                                        character: char_idx as u32,
                                    },
                                    end: Position {
                                        line: line_idx as u32,
                                        character: char_idx as u32 + 1,
                                    },
                                },
                                severity: Some(DiagnosticSeverity::ERROR),
                                code: None,
                                code_description: None,
                                source: Some("unclosed_delimiter".to_owned()),
                                message: format!("Unmatched closing delimiter '{}'", close),
                                related_information: None,
                                tags: None,
                                data: None,
                            });
                        }
                    }
                }
            }
            line_offset += line.len() + 1;
        }

        while let Some((open, close, line_idx, char_idx)) = stack.pop() {
            diagnostics.push(Diagnostic {
                range: Range {
                    start: Position {
                        line: line_idx as u32,
                        character: char_idx as u32,
                    },
                    end: Position {
                        line: line_idx as u32,
                        character: char_idx as u32 + 1,
                    },
                },
                severity: Some(DiagnosticSeverity::ERROR),
                code: None,
                code_description: None,
                source: Some("unclosed_delimiter".to_owned()),
                message: format!("Unclosed delimiter '{}'", open),
                related_information: None,
                tags: None,
                data: None,
            });
        }

        diagnostics
    }
}
