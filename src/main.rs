use std::collections::HashMap;
use tokio::sync::RwLock;

use nxd_language::diagnostics::severity::DiagnosticSeverity as NxdSeverity;
use nxd_language::diagnostics::validation::{LanguageDiagnostic, validate_source};
use tower_lsp_server::jsonrpc::Result;
use tower_lsp_server::ls_types::*;
use tower_lsp_server::{Client, LanguageServer, LspService, Server};

#[derive(Debug)]
struct Backend {
    client: Client,
    documents: RwLock<HashMap<Uri, String>>,
}

impl Backend {
    async fn publish_nxd_diagnostics(&self, uri: Uri, source: &str) {
        let validation = validate_source(source);

        let diagnostics = validation
            .diagnostics
            .iter()
            .map(language_diagnostic_to_lsp)
            .collect();

        self.client
            .publish_diagnostics(uri, diagnostics, None)
            .await;
    }
}

impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        let mut result = InitializeResult::default();

        result.capabilities.text_document_sync =
            Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::FULL));

        Ok(result)
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "NXD language server initialized.")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let document = params.text_document;
        let uri = document.uri;
        let source = document.text;

        self.documents
            .write()
            .await
            .insert(uri.clone(), source.clone());

        self.publish_nxd_diagnostics(uri, &source).await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let Some(change) = params.content_changes.into_iter().last() else {
            return;
        };

        let uri = params.text_document.uri;
        let source = change.text;

        self.documents
            .write()
            .await
            .insert(uri.clone(), source.clone());

        self.publish_nxd_diagnostics(uri, &source).await;
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        let uri = params.text_document.uri;

        self.documents.write().await.remove(&uri);

        self.client.publish_diagnostics(uri, Vec::new(), None).await;
    }
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| Backend {
        client,
        documents: RwLock::new(HashMap::new()),
    });

    Server::new(stdin, stdout, socket).serve(service).await;
}

fn language_diagnostic_to_lsp(diagnostic: &LanguageDiagnostic) -> Diagnostic {
    let severity = match diagnostic.severity {
        NxdSeverity::Error => DiagnosticSeverity::ERROR,
        NxdSeverity::Warning => DiagnosticSeverity::WARNING,
    };

    Diagnostic {
        range: Range {
            start: Position {
                line: diagnostic.span.line.saturating_sub(1),
                character: diagnostic.span.column.saturating_sub(1),
            },
            end: Position {
                line: diagnostic.span.end_line.saturating_sub(1),
                character: diagnostic.span.end_column.saturating_sub(1),
            },
        },

        severity: Some(severity),

        code: Some(NumberOrString::String(diagnostic.code.clone())),

        source: Some("NXD".to_string()),

        message: diagnostic.message.clone(),

        ..Diagnostic::default()
    }
}
