use tower_lsp::lsp_types::*;

pub fn check_unclosed_delimiters(content: &str) -> Vec<Diagnostic> {
    let mut diagnostics: Vec<Diagnostic> = Vec::new();
    let mut stack: Vec<(&str, &str, usize, usize)> = Vec::new();
    let delimiters = [("(", ")"), ("{", "}"), ("[", "]")];
    let mut line_offset: usize = 0;

    content.lines().enumerate().for_each(|(line_idx, line)| {
        line.chars().enumerate().for_each(|(char_idx, ch)| {
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
        });
        line_offset += line.len() + 1;
    });

    while let Some((open, _close, line_idx, char_idx)) = stack.pop() {
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
