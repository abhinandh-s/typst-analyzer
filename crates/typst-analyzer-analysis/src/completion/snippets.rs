use tower_lsp::lsp_types::{CompletionItem, CompletionItemKind};

use super::generic::TypCmpItem;

pub fn cmp_items() -> Vec<CompletionItem> {
    let mut items = Vec::new();
    items.append(&mut constructors());
    items
}

struct SnippetMaker<'a> {
    label: &'a str,
    details: &'a str,
    insert_text: String,
}

fn snips_time() -> Vec<SnippetMaker<'static>> {
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
        label: "date",
        details: "Insert current date",
        insert_text: format!(
            "#datetime(\n  year: {},\n  month: {},\n  day: {},\n).display()",
            year, month, day
        ),
    });
    snip.push(SnippetMaker {
        label: "time",
        details: "Insert current time",
        insert_text: format!(
            "#datetime(\n  hour: {},\n  minute: {},\n  second: {},\n).display()",
            hour, min, sec
        ),
    });
    snip.push(SnippetMaker {
        label: "datetime",
        details: "Insert current date and time",
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

fn snips_set_items() -> Vec<SnippetMaker<'static>> {
    vec![
    SnippetMaker {
        label: "set_par",
        details: "Insert current date",
        insert_text: "#set par(\n  leading: 0.65em,\n  first-line-indent: 1em,\n  spacing: 1.2em,\n  justify: true,\n)".to_owned(),
    },
    SnippetMaker {
        label: "set_par_line",
        details: "Insert current date",
        insert_text: "#set par.line(numbering: \"1\")".to_owned(),
    }]
}
