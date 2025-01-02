use std::env::current_dir;
use std::fs::{self, write};

use dirs::config_dir;
use serde::Deserialize;
use tower_lsp::lsp_types::{CompletionItem, CompletionItemKind};

use crate::typ_logger;

use super::generic::TypCmpItem;

pub fn cmp_items() -> Vec<CompletionItem> {
    let mut items = Vec::new();
    items.append(&mut constructors());
    items
}

struct SnippetMaker {
    label: String,
    details: String,
    insert_text: String,
}

fn snips_time() -> Vec<SnippetMaker> {
    let mut snip = Vec::new();
    let current_time = chrono::Local::now();
    let (year, month, day, hour, min, sec) = (
        current_time.format("%Y").to_string(),
        current_time.format("%m").to_string(),
        current_time.format("%d").to_string(),
        current_time.format("%H").to_string(),
        current_time.format("%M").to_string(),
        current_time.format("%S").to_string(),
    );
    snip.push(SnippetMaker {
        label: "date".to_owned(),
        details: "Insert current date".to_owned(),
        insert_text: format!(
            "#datetime(\n  year: {},\n  month: {},\n  day: {},\n).display()",
            year, month, day
        ),
    });
    snip.push(SnippetMaker {
        label: "time".to_owned(),
        details: "Insert current time".to_owned(),
        insert_text: format!(
            "#datetime(\n  hour: {},\n  minute: {},\n  second: {},\n).display()",
            hour, min, sec
        ),
    });
    snip.push(SnippetMaker {
        label: "datetime".to_owned(),
        details: "Insert current date and time".to_owned(),
        insert_text: format!(
            "#datetime(\n  year: {},\n  month: {},\n  day: {},\n  hour: {},\n  minute: {},\n  second: {},\n).display()",
            year, month, day,
            hour, min, sec
        ),
    });
    snip
}

fn constructors() -> Vec<CompletionItem> {
    let mut items = Vec::new();
    // vec of tuples with the constructor name, the label details and insert text

    let mut constructor = snips_time();
    constructor.append(&mut snips_set_items());
    match snips_from_yaml() {
        Some(mut user_snips) => {
            constructor.append(&mut user_snips);
        }
        None => typ_logger!("can't find snippets"),
    }

    for ctx in constructor {
        let item = TypCmpItem {
            label: ctx.label.to_owned(),
            label_details: "snippet",
            kind: CompletionItemKind::SNIPPET,
            documentation: ctx.details,
            insert_text: ctx.insert_text,
        };
        items.push(item);
    }
    TypCmpItem::get_cmp(items)
}

fn snips_set_items() -> Vec<SnippetMaker> {
    vec![
    SnippetMaker {
        label: "set_par".to_owned(),
        details: "Insert current date".to_owned(),
        insert_text: "#set par(\n  leading: 0.65em,\n  first-line-indent: 1em,\n  spacing: 1.2em,\n  justify: true,\n)".to_owned(),
    },
    SnippetMaker {
        label: "set_par_line".to_owned(),
        details: "Insert current date".to_owned(),
        insert_text: "#set par.line(numbering: \"1\")".to_owned(),
    }]
}

#[derive(Deserialize, Debug)]
struct UserSnippet {
    label: String,
    details: String,
    insert_text: String,
}

#[derive(Deserialize, Debug)]
struct SnippetFile {
    snippets: Vec<UserSnippet>,
}

fn load_user_snippets() -> Option<Vec<UserSnippet>> {
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
    let mut serialized_snippets = Vec::new();
    if let Some(user_snippets) = load_user_snippets() {
        for snip in user_snippets {
            serialized_snippets.push(SnippetMaker {
                label: snip.label,
                details: snip.details,
                insert_text: snip.insert_text,
            });
        }
        return Some(serialized_snippets);
    }
    None
}
