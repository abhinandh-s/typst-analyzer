use dashmap::DashMap;
use tower_lsp::{LspService, Server};
use typst_analyzer::backend::Backend;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let (stdin, stdout) = (tokio::io::stdin(), tokio::io::stdout());

    let (service, socket) = LspService::new(|client| Backend {
        client,
        doc_map: DashMap::new(),
        ast_map: DashMap::new(),
        symbol_table: DashMap::new(),
    });
    Server::new(stdin, stdout, socket).serve(service).await;
}
