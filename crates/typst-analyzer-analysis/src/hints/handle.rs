use regex::Regex;
use tower_lsp::lsp_types::*;

pub fn calculate_inlay_hints(doc: &str) -> Result<Vec<InlayHint>, anyhow::Error> {
    let mut hints: Vec<InlayHint> = Vec::new();

    // Regex to match any word within angle brackets and @word
    let angle_brackets_re: Regex = Regex::new(r"<(\w+)>")?;
    let at_word_re: Regex = Regex::new(r"@(\w+)")?;

    doc.lines().enumerate().for_each(|(line_idx, line)| {
        // Match words within angle brackets
        angle_brackets_re.captures_iter(line).for_each(|cap| {
            if let Some(matched_word) = cap.get(1) {
                if let Some(cap_get) = cap.get(0) {
                    let start = cap_get.start();
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
        });

        // Match @word patterns
        at_word_re.captures_iter(line).for_each(|cap| {
            if let Some(matched_word) = cap.get(1) {
                if let Some(cap_get) = cap.get(0) {
                    let start = cap_get.start();
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
        });
    });
    Ok(hints)
}
