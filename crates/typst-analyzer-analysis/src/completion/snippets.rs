use std::env::current_dir;
use std::fs::{self, write};

use dirs::config_dir;
use serde::Deserialize;
use tower_lsp::lsp_types::CompletionItem;

use crate::typ_logger;

use super::core::{ToTypCmpItem, TypCmpItem};

/// Collect and provide all the snippets for the language server
pub fn collect() -> Vec<CompletionItem> {
    let mut snippets = snips_time();
    snippets.append(&mut snips_set_items());
    if let Some(snips) = snips_from_yaml().as_mut() {
        snippets.append(snips)
    }
    TypCmpItem::convert(snippets.to_typ_cmp_item())
}

/// the main type to create snippets
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SnippetMaker {
    pub label: String,
    pub documentation: String,
    pub insert_text: String,
}

impl SnippetMaker {
    pub fn new(label: String, details: String, insert_text: String) -> Self {
        Self {
            label,
            documentation: details,
            insert_text,
        }
    }
}

/// type to deserialize the user defined snippets
#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
struct UserSnippet {
    label: String,
    details: String,
    insert_text: String,
}

/// Deserialize the user defined snippets from the config file
#[derive(Deserialize, Debug)]
struct SnippetFile {
    snippets: Vec<UserSnippet>,
}

/// Load user defined snippets from the config directory or from the current directory
fn load_user_snippets() -> Option<Vec<UserSnippet>> {
    // -- TODO: write this into config file using serde serialization
    let yml_ctx = "snippets:\n  - label: \"custom_date\"\n    details: \"Insert a custom date snippet\"\n    insert_text: \"#date(year: 2025, month: 1, day: 1)\"\n  - label: \"custom_time\"\n    details: \"Insert a custom time snippet\"\n    insert_text: \"#time(hour: 12, minute: 30, second: 45)\""
        .to_owned();
    if let Some(config_dir) = config_dir() {
        let typst_analyzer_config_dir = config_dir.join("typst-analyzer");
        if !typst_analyzer_config_dir.exists() {
            match fs::create_dir_all(typst_analyzer_config_dir.clone()) {
                Ok(_) => typ_logger!("info: created config dir."),
                Err(err) => typ_logger!("error: {}", err),
            }
        }
        let global_snippets = typst_analyzer_config_dir.join("snippets.yml");
        if !global_snippets.exists() {
            let re = write(&global_snippets, yml_ctx);
            match re {
                Ok(_) => typ_logger!("info: written snippets config file."),
                Err(err) => typ_logger!("error: {}", err),
            }
        }
        let snippets = if let Ok(c_dir) = current_dir() {
            let local_snippets = c_dir.join("snippets.yml");
            if local_snippets.exists() {
                local_snippets
            } else {
                global_snippets
            }
        } else {
            global_snippets
        };
        if let Ok(content) = std::fs::read_to_string(snippets) {
            let snippet_file: Result<SnippetFile, serde_yml::Error> = serde_yml::from_str(&content);
            if let Ok(snippets) = snippet_file {
                let items = snippets.snippets;
                return Some(items);
            }
        }
    }
    None
}

/// serialize the user defined snippets in to SnippetMaker
fn snips_from_yaml() -> Option<Vec<SnippetMaker>> {
    load_user_snippets().map(|user_snippets| {
        user_snippets
            .into_iter()
            .map(|snip| SnippetMaker {
                label: snip.label,
                documentation: snip.details,
                insert_text: snip.insert_text,
            })
            .collect()
    })
}

fn snips_time() -> Vec<SnippetMaker> {
    let current_time = chrono::Local::now();
    let (year, month, day, hour, min, sec) = (
        current_time.format("%Y").to_string(),
        current_time.format("%m").to_string(),
        current_time.format("%d").to_string(),
        current_time.format("%H").to_string(),
        current_time.format("%M").to_string(),
        current_time.format("%S").to_string(),
    );
    vec![
        SnippetMaker {
            label: "date".to_owned(),
            documentation: "Insert current date".to_owned(),
            insert_text: format!(
                "#datetime(\n  year: {},\n  month: {},\n  day: {},\n).display()",
                year, month, day
            ),
        },
        SnippetMaker {
            label: "time".to_owned(),
            documentation: "Insert current time".to_owned(),
            insert_text: format!(
                "#datetime(\n  hour: {},\n  minute: {},\n  second: {},\n).display()",
                hour, min, sec
            ),
        },
        SnippetMaker {
            label: "datetime".to_owned(),
            documentation: "Insert current date and time".to_owned(),
            insert_text: format!(
                "#datetime(\n  year: {},\n  month: {},\n  day: {},\n  hour: {},\n  minute: {},\n  second: {},\n).display()",
                year, month, day,
                hour, min, sec
            ),
        }
    ]
}

fn snips_set_items() -> Vec<SnippetMaker> {
    vec![
        SnippetMaker {
            label: "set_par".to_owned(),
            documentation: "Insert current date".to_owned(),
            insert_text: "#set par(\n  leading: 0.65em,\n  first-line-indent: 1em,\n  spacing: 1.2em,\n  justify: true,\n)".to_owned(),
        },
        SnippetMaker {
            label: "set_par_line".to_owned(),
            documentation: "Insert current date".to_owned(),
            insert_text: "#set par.line(numbering: \"1\")".to_owned(),
        }]
}
