//! The backend module contains the Backend struct that holds the client, the document map and the
//! AST map. It also contains the implementation of the LanguageServer trait for the Backend
//! struct.

use std::sync::Arc;

use dashmap::DashMap;
use serde_json::Value;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer};
use typst_analyzer_analysis::{calculate_inlay_hints, check_unclosed_delimiters};
use typst_syntax::{FileId, Source, VirtualPath};

use crate::code_actions::handle::TypstCodeActions;
use crate::completion::TypstCompletion;
use crate::definition::HandleDefinitions;
use crate::error_ctx::TypError;
use crate::hover::HandleHover;
use crate::symbols::{Symbol, SymbolTable};
use crate::typ_logger;

/// The backend struct that holds the client, the document map and the AST map
#[derive(Debug)]
pub struct Backend {
    pub client: Client,
    // Maps a document URI to its text content
    pub doc_map: DashMap<String, String>,
    // Maping a document URI to its parsed AST of type SyntaxNode is what is have seen in other
    // lsps but i am maping to to type Source cuz
    // 1. i can easly parse it to AST
    // 2, it contains additional metadata (we need ast and span id in some cases)
    pub ast_map: DashMap<String, Source>,
    // Maps symbol names to Symbol metadata
    pub symbol_table: Arc<DashMap<String, Symbol>>,
}

/// Helper function to convert a Position to an offset in the text
///
/// Offset is the number of characters from the start of the text
pub fn position_to_offset(text: &str, position: Position) -> Option<usize> {
    let mut offset = 0;
    // Iterate over the lines and characters in the text
    for (line_idx, line) in text.lines().enumerate() {
        if line_idx == position.line as usize {
            // If the line index matches the position line, return the offset
            return Some(offset + position.character as usize);
        }
        offset += line.len() + 1; // +1 for the newline character
    }
    None
}

impl Backend {
    /// funciton to handle did change requests
    pub async fn handle_did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri.to_string();
        let populate_symbol_table_re = self.populate_symbol_table(params.clone());
        match populate_symbol_table_re {
            Ok(_) => {}
            Err(err) => typ_logger!("{}", TypError::NonCriticalError(err.to_string().as_str())),
        }
        // Check if the document exists in the document map with key (uri) and collect the document if exists
        if let Some(mut doc) = self.doc_map.get_mut(&uri) {
            for change in params.content_changes {
                // Get the document content
                let mut doc_ctx = doc.value().clone();
                // Get the range of the change
                if let Some(range) = change.range {
                    // Get the start and end positions of the range
                    let start = range.start;
                    let end = range.end;
                    // Convert the position to an offset
                    let start_idx = position_to_offset(&doc_ctx, start).unwrap_or(0);
                    let end_idx = position_to_offset(&doc_ctx, end).unwrap_or(doc_ctx.len());
                    // Replace the text in the range with the new text
                    doc_ctx.replace_range(start_idx..end_idx, &change.text);
                } else {
                    // If range is None, replace the whole text
                    doc_ctx = change.text.clone();
                }
                // Update the document with the new content
                *doc.value_mut() = doc_ctx.clone();
                // Parse the document content to generate the AST
                if let Some(path_from_uri) = &uri.strip_prefix("file://") {
                    let ast =
                        Source::new(FileId::new(None, VirtualPath::new(path_from_uri)), doc_ctx);
                    // Update the AST map with the new AST
                    self.ast_map.insert(uri.clone(), ast);
                }
            }
            // Check for unclosed delimiters
            let mut diagnostics = check_unclosed_delimiters(&doc);
            if let Ok(mut syntax_err) = self.syntax_error(params.text_document.uri.clone()) {
            diagnostics.append(&mut syntax_err);
            }
            // Publish the diagnostics to the client
            self.client
                .publish_diagnostics(params.text_document.uri.clone(), diagnostics, None)
                .await;
        }
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
pub enum OneOfThis<A, B> {
    Left(A),
    Right(B),
}

impl Backend {}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    /// Initialize the language server
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            server_info: None,
            capabilities: ServerCapabilities {
                inlay_hint_provider: Some(OneOf::Left(true)),
                definition_provider: Some(OneOf::Left(true)),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                code_action_provider: Some(CodeActionProviderCapability::Options(
                    CodeActionOptions {
                        code_action_kinds: Some(vec![
                            CodeActionKind::QUICKFIX,
                            CodeActionKind::SOURCE,
                        ]),
                        work_done_progress_options: WorkDoneProgressOptions {
                            work_done_progress: Some(true),
                        },
                        resolve_provider: Some(true),
                    },
                )),
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    // Incremental sync
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
                folding_range_provider: Some(FoldingRangeProviderCapability::Simple(true)),
                workspace: Some(WorkspaceServerCapabilities {
                    workspace_folders: Some(WorkspaceFoldersServerCapabilities {
                        supported: Some(true),
                        change_notifications: Some(OneOf::Left(true)),
                    }),
                    file_operations: Some(WorkspaceFileOperationsServerCapabilities {
                        will_create: Some(FileOperationRegistrationOptions {
                            filters: vec![FileOperationFilter {
                                scheme: Some("url".to_owned()),
                                pattern: FileOperationPattern {
                                    glob: "**".to_owned(),
                                    matches: Some(FileOperationPatternKind::File),
                                    options: Some(FileOperationPatternOptions {
                                        ignore_case: Some(false),
                                    }),
                                },
                            }],
                        }),
                        did_create: Some(FileOperationRegistrationOptions {
                            filters: vec![FileOperationFilter {
                                scheme: Some("url".to_owned()),
                                pattern: FileOperationPattern {
                                    glob: "**".to_owned(),
                                    matches: Some(FileOperationPatternKind::File),
                                    options: Some(FileOperationPatternOptions {
                                        ignore_case: Some(false),
                                    }),
                                },
                            }],
                        }),
                        ..Default::default()
                    }),
                }),
                ..ServerCapabilities::default()
            },
        })
    }

    /// Notify the language server that it has been initialized
    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "Language Server initialized!")
            .await;
    }

    async fn folding_range(
        &self,
        _params: FoldingRangeParams,
    ) -> Result<Option<Vec<FoldingRange>>> {
        Ok(None)
    }

    async fn goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>> {
        let definitions_result = self.provide_definitions(params);
        match definitions_result {
            Ok(definition) => Ok(Some(definition)),
            Err(_) => Ok(None),
        }
    }

    /// Handle completion requests
    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        let cmp_result = TypstCompletion::handle_completions(__self, params.text_document_position);
        match cmp_result {
            Ok(cmp) => {
                return Ok(Some(CompletionResponse::Array(cmp)));
            }
            Err(err) => {
                typ_logger!("{}", err);
                return Ok(None);
            }
        }
    }

    /// Handle hover requests
    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let definitions_result = self.provide_hover_ctx(params);
        match definitions_result {
            Ok(definition) => Ok(Some(definition)),
            Err(_) => Ok(None),
        }
    }

    /// Handle inlay hint requests
    async fn inlay_hint(&self, params: InlayHintParams) -> Result<Option<Vec<InlayHint>>> {
        let uri = params.text_document.uri.to_string();
        if let Some(doc) = self.doc_map.get(&uri) {
            return match calculate_inlay_hints(&doc) {
                Ok(hints) => Ok(Some(hints)),
                Err(_) => Ok(None),
            };
        }
        Ok(None)
    }

    /// Handle code action requests
    async fn code_action(&self, params: CodeActionParams) -> Result<Option<CodeActionResponse>> {
        let uri = params.text_document.uri.to_string();
        self.client
            .log_message(MessageType::INFO, "Code action requested")
            .await;

        if let Some(doc) = self.doc_map.get(&uri) {
            let content = doc.value();
            let range = params.range;

            let actions =
                self.generate_code_actions(content, range, params.text_document.uri.clone());
            let ctx_restlt =
                self.calculate_code_actions_for_bib(content, range, params.text_document.uri);

            match ctx_restlt {
                Ok(mut ctx) => match actions {
                    Ok(mut actions) => {
                        actions.append(&mut ctx);
                        if !actions.is_empty() {
                            self.client
                                .log_message(MessageType::INFO, "Code actions generated")
                                .await;
                            return Ok(Some(actions));
                        }
                    }
                    Err(err) => typ_logger!("{}", err),
                },
                Err(err) => typ_logger!("{}", err),
            }
        }
        Ok(None)
    }

    /// Handle did open requests
    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let text_document = params.text_document;
        self.doc_map
            .insert(text_document.uri.to_string(), text_document.text.clone());

        if let Some(doc) = self.doc_map.get_mut(&text_document.uri.to_string()) {
            // Update the AST map with the new AST
            if let Some(path_from_uri) = &text_document.uri.to_string().strip_prefix("file://") {
                let ast = Source::new(
                    FileId::new(None, VirtualPath::new(path_from_uri)),
                    doc.value().to_string(),
                );
                // Update the AST map with the new AST
                self.ast_map.insert(text_document.uri.to_string(), ast);
            }
        }
        self.client
            .log_message(
                MessageType::INFO,
                format!("Opened file: {}", text_document.uri),
            )
            .await;
    }

    /// Handle did change requests
    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        self.handle_did_change(params).await;
        self.client
            .log_message(MessageType::INFO, "File changed!")
            .await;
    }

    /// Handle did save requests
    async fn did_save(&self, _: DidSaveTextDocumentParams) {
        self.client
            .log_message(MessageType::INFO, "File saved!")
            .await;
    }

    /// Handle did close requests
    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        let uri = params.text_document.uri.to_string();
        self.doc_map.remove(&uri);
        self.client
            .log_message(MessageType::INFO, format!("Closed file: {}", uri))
            .await;
    }

    /// Handle shutdown requests
    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    /// Handle did change workspace folders requests
    async fn did_change_workspace_folders(&self, _: DidChangeWorkspaceFoldersParams) {
        self.client
            .log_message(MessageType::INFO, "Workspace folders changed!")
            .await;
    }

    /// Handle did change configuration requests
    async fn did_change_configuration(&self, _: DidChangeConfigurationParams) {
        self.client
            .log_message(MessageType::INFO, "Configuration changed!")
            .await;
    }

    /// Handle did change watched files requests
    async fn did_change_watched_files(&self, _: DidChangeWatchedFilesParams) {
        self.client
            .log_message(MessageType::INFO, "Watched files have changed!")
            .await;
    }

    /// Handle execute command requests
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
