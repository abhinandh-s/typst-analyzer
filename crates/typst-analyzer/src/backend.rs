use dashmap::DashMap;
use serde_json::Value;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer};
use typst_analyzer_analysis::{calculate_inlay_hints, check_unclosed_delimiters};
use typst_syntax::Source;

use crate::code_actions::handle::TypstCodeActions;
use crate::completion::handle::TypstCompletion;

#[derive(Debug)]
pub struct Backend {
    pub client: Client,
    pub doc_map: DashMap<String, String>,
    pub ast_map: DashMap<String, Source>,
}

pub fn position_to_offset(text: &str, position: Position) -> Option<usize> {
    let mut offset = 0;
    for (line_idx, line) in text.lines().enumerate() {
        if line_idx == position.line as usize {
            return Some(offset + position.character as usize);
        }
        offset += line.len() + 1; // +1 for the newline character
    }
    None
}

impl Backend {}

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

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        let cmp =
            TypstCompletion::handle_completions(__self, params.text_document_position);
        Ok(Some(CompletionResponse::Array(
            cmp, /*handle_completions() */
        )))
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
        if let Some(doc) = self.doc_map.get(&uri) {
            let hints = calculate_inlay_hints(&doc);
            return Ok(Some(hints));
        }
        Ok(None)
    }

    async fn code_action(&self, params: CodeActionParams) -> Result<Option<CodeActionResponse>> {
        let uri = params.text_document.uri.to_string();
        self.client
            .log_message(MessageType::INFO, "Code action requested")
            .await;

        if let Some(doc) = self.doc_map.get(&uri) {
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

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let text_document = params.text_document;
        self.doc_map
            .insert(text_document.uri.to_string(), text_document.text.clone());
        let source = Source::detached(text_document.text.clone());
        self.ast_map.insert(text_document.uri.to_string(), source);
        self.client
            .log_message(
                MessageType::INFO,
                format!("Opened file: {}", text_document.uri),
            )
            .await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri.to_string();
        if let Some(mut doc) = self.doc_map.get_mut(&uri) {
            for change in params.content_changes {
                // Apply changes incrementally
                let mut current_text = doc.value().clone();

                // Apply the text edit
                if let Some(range) = change.range {
                    let start = range.start;
                    let end = range.end;

                    let start_idx = position_to_offset(&current_text, start).unwrap_or(0);
                    let end_idx =
                        position_to_offset(&current_text, end).unwrap_or(current_text.len());

                    current_text.replace_range(start_idx..end_idx, &change.text);
                } else {
                    // If range is None, replace the whole text
                    current_text = change.text.clone();
                }

                *doc.value_mut() = current_text;
            }
            if let Some(mut source) = self.ast_map.get_mut(&uri) {
                source.replace(doc.value());
            }
            // Check for unclosed delimiters
            let diagnostics = check_unclosed_delimiters(&doc);
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
        self.doc_map.remove(&uri);
        self.client
            .log_message(MessageType::INFO, format!("Closed file: {}", uri))
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
}
