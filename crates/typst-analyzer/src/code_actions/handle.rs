use std::collections::HashMap;

use anyhow::Error;
use regex::Regex;
use tower_lsp::lsp_types::*;
use typst_analyzer_analysis::dict::*;

use crate::backend::Backend;

pub(crate) trait TypstCodeActions {
    fn get_table_parameters(&self) -> HashMap<String, String>;
    fn parse_funtion_params(&self, content: &str) -> Result<Vec<String>, Error>;
    fn calculate_code_actions_for_vs_code(
        &self,
        content: &str,
        range: Range,
        uri: Url,
    ) -> Result<Vec<CodeActionOrCommand>, Error>;
    fn generate_code_actions(
        &self,
        content: &str,
        range: Range,
        uri: Url,
    ) -> Result<Vec<CodeActionOrCommand>, Error>;
    fn calculate_code_actions_for_bib(
        &self,
        content: &str,
        range: Range,
        uri: Url,
    ) -> Result<Vec<CodeActionOrCommand>, Error>;
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

impl TypstCodeActions for Backend {
    // FEAT: add a dummy heading and add a dummy item
    fn get_table_parameters(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        params.insert(COLUMNS.0.to_owned(), COLUMNS.1.to_owned());
        params.insert(ROWS.0.to_owned(), ROWS.1.to_owned());
        params.insert(GUTTER.0.to_owned(), GUTTER.1.to_owned());
        params.insert(COLUMN_GUTTER.0.to_owned(), COLUMN_GUTTER.1.to_owned());
        params.insert(ROW_GUTTER.0.to_owned(), ROW_GUTTER.1.to_owned());
        params.insert(FILL.0.to_owned(), FILL.1.to_owned());
        params.insert(ALIGN.0.to_owned(), ALIGN.1.to_owned());
        params.insert(STROKE.0.to_owned(), STROKE.1.to_owned());
        params.insert(INSET.0.to_owned(), INSET.1.to_owned());
        params
    }

    /// Parses the content inside `#table(...)` and extracts the parameters already defined.
    ///
    /// returns vector of existing params
    fn parse_funtion_params(&self, content: &str) -> Result<Vec<String>, Error> {
        // Regular expression to find parameters (e.g., `param:`).
        let re = Regex::new(r"(\w+(-\w+)?)\s*:")?; // -- FIX: wont work as expected
        let mut existing_params = Vec::new();

        for cap in re.captures_iter(content) {
            if let Some(param) = cap.get(1) {
                existing_params.push(param.as_str().to_owned());
            }
        }
        Ok(existing_params)
    }

    fn calculate_code_actions_for_vs_code(
        &self,
        content: &str,
        range: Range,
        uri: Url,
    ) -> Result<Vec<CodeActionOrCommand>, Error> {
        let mut actions = Vec::new();

        // Check if the text "VS Code" is within the range
        let vs_code_re = Regex::new(r"VS Code")?;

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
        // -- TEST: this will work but the created file is not showing up in file explorer maybe
        // this might not be the actual method;
        // if let Ok(uri_two) = Url::parse("file:///home/abhi/docs/just/example_two.typ") {
        //     let edit_two = TextEdit {
        //         range: Range {
        //             start: Position {
        //                 line: 0,
        //                 character: 0,
        //             },
        //             end: Position {
        //                 line: 0,
        //                 character: 0,
        //             },
        //         },
        //         new_text: "Neovim".to_owned(),
        //     };
        //     let workspace_edit_two = WorkspaceEdit {
        //         changes: Some(HashMap::from([(uri_two.clone(), vec![edit_two])])),
        //         document_changes: None,
        //         change_annotations: None,
        //     };
        //     let code_action_two = CodeAction {
        //         title: "Create new file".to_owned(),
        //         kind: Some(CodeActionKind::QUICKFIX),
        //         diagnostics: None,
        //         edit: Some(workspace_edit_two),
        //         command: None,
        //         is_preferred: Some(true),
        //         disabled: None,
        //         data: None,
        //     };
        //     actions.push(CodeActionOrCommand::CodeAction(code_action_two));
        // }
        Ok(actions)
    }

    fn generate_code_actions(
        &self,
        content: &str,
        range: Range,
        uri: Url,
    ) -> Result<Vec<CodeActionOrCommand>, Error> {
        typst_analyzer_analysis::bibliography::parse_bib()?;
        let mut actions = Vec::new();
        let mut multiline_table = String::new();
        let mut in_table_block = false;
        let mut table_start_line = 0;

        for (line_idx, line) in content.lines().enumerate() {
            if line.contains("#table(") {
                in_table_block = true;
                table_start_line = line_idx;
            }

            if in_table_block {
                multiline_table.push_str(line);
                multiline_table.push('\n');

                if line.contains(")") {
                    in_table_block = false;

                    // Extract existing parameters inside `#table(...)`.
                    let existing_params: Vec<String> =
                        self.parse_funtion_params(&multiline_table)?;

                    // Get all default parameters.
                    let all_params: HashMap<String, String> = self.get_table_parameters();

                    // Generate a separate code action for each missing parameter.
                    for (param, default_value) in all_params {
                        if !existing_params.contains(&param) {
                            let title = format!("Add optional parameter: {}", param);

                            // Create a new parameter string.
                            let new_param = format!("\n  {}: {},\n", param, default_value);
                            // Prepare the text edit to add the missing parameter.
                            let edit = TextEdit {
                                range: Range {
                                    start: Position {
                                        // line: table_start_line as u32 + 1,
                                        // character: 2,
                                        line: table_start_line as u32,
                                        character: line.find("#table(").unwrap_or(5) as u32 + 7, // Position after `#table(`.
                                    },
                                    end: Position {
                                        // line: table_start_line as u32 + 1,
                                        // character: 2,
                                        line: table_start_line as u32,
                                        character: line.find("#table(").unwrap_or(0) as u32 + 7,
                                    },
                                },
                                new_text: new_param,
                            };

                            // Define the workspace edit for the code action.
                            let workspace_edit = WorkspaceEdit {
                                changes: Some(HashMap::from([(uri.clone(), vec![edit])])),
                                document_changes: None,
                                change_annotations: None,
                            };

                            // Create the code action for adding the missing parameter.
                            let code_action = CodeAction {
                                title,
                                kind: Some(CodeActionKind::QUICKFIX),
                                diagnostics: None,
                                edit: Some(workspace_edit),
                                command: None,
                                is_preferred: Some(true),
                                disabled: None,
                                data: None,
                            };

                            // Add the code action to the list.
                            actions.push(CodeActionOrCommand::CodeAction(code_action));
                        }
                    }

                    // Reset the multiline table content for the next block.
                    multiline_table.clear();
                }
            }
        }

        let mut vs_code_action = self.calculate_code_actions_for_vs_code(content, range, uri)?;
        actions.append(&mut vs_code_action);

        Ok(actions)
    }

    fn calculate_code_actions_for_bib(
        &self,
        content: &str,
        range: Range,
        uri: Url,
    ) -> Result<Vec<CodeActionOrCommand>, Error> {
        let mut actions = Vec::new();

        // Check if the text "VS Code" is within the range
        let bib_re = Regex::new(r#"([^"]*)"#)?;

        for (line_idx, line) in content.lines().enumerate() {
            if let Some(vs_code_match) = bib_re.find(line) {
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
                        new_text: "".to_owned(),
                    };

                    let workspace_edit = WorkspaceEdit {
                        changes: Some(HashMap::from([(uri.clone(), vec![edit])])),
                        document_changes: None,
                        change_annotations: None,
                    };

                    let code_action = CodeAction {
                        title: "File not found, Create one".to_owned(),
                        kind: Some(CodeActionKind::QUICKFIX),
                        diagnostics: Some(vec![Diagnostic {
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
                            severity: Some(DiagnosticSeverity::ERROR),
                            source: Some("File not found, Create one".to_owned()),
                            message: "File not found, Create one".to_owned(),
                            ..Default::default()
                        }]),
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

        Ok(actions)
    }
}

// unused
impl Backend {
    // Register the custom command to the client
    pub async fn register_custom_command(&self) -> Result<(), Error> {
        // Send a registration message to the client
        // Register the command for use
        // (The registration message is handled by the LSP client)
        Ok(())
    }

    // Handle the execution of the custom command
    pub async fn execute_custom_command(&self, params: ExecuteCommandParams) -> Result<(), Error> {
        if params.command == "customCommand" {
            println!("Running custom function...");
            // Replace this with actual custom logic, e.g., custom function call
        }
        Ok(())
    }
}
