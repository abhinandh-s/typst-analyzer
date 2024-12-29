use std::collections::VecDeque;

use tower_lsp::lsp_types::{Hover, HoverContents, HoverParams, MarkupContent, MarkupKind};
use typst_analyzer_analysis::node::node_walker;
use typst_syntax::SyntaxKind;

use crate::backend::{position_to_offset, Backend};
use crate::error_ctx::TypError;
use crate::typ_logger;

pub(crate) trait HandleHover {
    fn provide_hover_ctx(&self, params: HoverParams) -> Result<Hover, anyhow::Error>;
}

impl HandleHover for Backend {
    fn provide_hover_ctx(&self, params: HoverParams) -> Result<Hover, anyhow::Error> {
        let mut hover_ctx = String::new();
        let uri = params.text_document_position_params.text_document.uri;
        if let Some(text) = self.doc_map.get(&uri.to_string()) {
            if let Some(position) =
                position_to_offset(&text, params.text_document_position_params.position)
            {
                if let Some(ast_map_ctx) = self.ast_map.get(&uri.to_string()) {
                    // node_walker will walk throug the AST map from cursor position and return
                    // VecDeque as [Markup, Ref, RefMarker] if cursor is in
                    // a RefMarker ie, reference.
                    let syntax_kind: VecDeque<SyntaxKind> = node_walker(position, &ast_map_ctx);
                    typ_logger!("hovered syntax_kind: {:?}", syntax_kind);
                    let refmarker = syntax_kind.back();
                    if let Some(syntax) = refmarker {
                        if *syntax == SyntaxKind::RefMarker {
                            hover_ctx = REF_DETAILS.to_owned();
                        }
                        if *syntax == SyntaxKind::Label {
                            hover_ctx = LABEL_DETAILS.to_owned();
                        }
                    }
                    // refmarker = RefMarker
                    typ_logger!("hovered : {:?}", refmarker);
                    typ_logger!("hovered ctx: {:?}", &hover_ctx);
                    // dummy return
                    return Ok(Hover {
                        contents: HoverContents::Markup(MarkupContent {
                            kind: MarkupKind::Markdown,
                            value: hover_ctx,
                        }),
                        range: None,
                    });
                }
            }
        }
        Err(TypError::Invalid.into())
    }
}

const LABEL_DETAILS: &str = r#"
# label 

A label for an element.

Inserting a label into content attaches it to the closest preceding
element that is not a space. The preceding element must be in the 
same scope as the label, which means that Hello `#[<label>]`, for instance, wouldn't work.

A labelled element can be referenced, queried for, and styled through its label.

Once constructed, you can get the name of a label using str.

## Example

```typst
#show <a>: set text(blue)
#show label("b"): set text(red)

= Heading <a>
*Strong* #label("b")
```"#;
const REF_DETAILS: &str = r#"
# ref [Element function]

A reference to a label or bibliography.

## Syntax

This function also has dedicated syntax: A reference to a label can be created
by typing an @ followed by the name of the label

```typst
= Introduction <intro> can be referenced by typing @intro.
```

To customize the supplement, add content in square brackets after
the reference: 

```typst
@intro[Chapter].
```

# Example

```typst
#set heading(numbering: "1.")
#set math.equation(numbering: "(1)")

= Introduction <intro>
Recent developments in
typesetting software have
rekindled hope in previously
frustrated researchers. @distress
As shown in @results, we ...

= Results <results>
We discuss our approach in
comparison with others.

== Performance <perf>
@slow demonstrates what slow
software looks like.
$ T(n) = O(2^n) $ <slow>

#bibliography("works.bib")
```

---
> [!TIP]
> Element function can be customized with *set* and *show* rules
---

## Details

Produces a textual reference to a label.

For example, a reference to a heading will yield 
an appropriate string such as "Section 1" for a reference 
to the first heading. The references are also links to the 
respective element. Reference syntax can also be used to cite from a bibliography.

Referenceable elements include headings, figures, equations, and footnotes.
To create a custom referenceable element like a theorem, you can create a figure
of a custom kind and write a show rule for it. In the future, there might be a
more direct way to define a custom referenceable element.

If you just want to link to a labelled element and not get an automatic
textual reference, consider using the link function instead.
"#;
