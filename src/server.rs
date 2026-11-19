use crate::completion::{
    nxd_completion_items, nxd_member_completion_items, nxd_object_initializer_completion_items,
    nxd_scope_completion_items,
};
use crate::diagnostics::language_diagnostic_to_lsp;
use crate::documents::DocumentStore;
use crate::hover::hover_info_to_lsp;

use nxd_language::diagnostics::validation::validate_source;
use nxd_language::frontend::hover::hover_at;

use tower_lsp_server::jsonrpc::Result;
use tower_lsp_server::ls_types::*;
use tower_lsp_server::{Client, LanguageServer};

#[derive(Debug)]
pub struct Backend {
    client: Client,
    documents: DocumentStore,
}

impl Backend {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            documents: DocumentStore::new(),
        }
    }

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

        result.capabilities.completion_provider = Some(CompletionOptions {
            trigger_characters: Some(vec![".".to_string()]),
            ..CompletionOptions::default()
        });

        result.capabilities.hover_provider = Some(HoverProviderCapability::Simple(true));

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

        self.documents.insert(uri.clone(), source.clone()).await;

        self.publish_nxd_diagnostics(uri, &source).await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let Some(change) = params.content_changes.into_iter().last() else {
            return;
        };

        let uri = params.text_document.uri;
        let source = change.text;

        self.documents.insert(uri.clone(), source.clone()).await;

        self.publish_nxd_diagnostics(uri, &source).await;
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        let uri = params.text_document.uri;

        self.documents.remove(&uri).await;

        self.client.publish_diagnostics(uri, Vec::new(), None).await;
    }

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        let text_document_position = params.text_document_position;

        let uri = text_document_position.text_document.uri;

        let position = text_document_position.position;

        let Some(source) = self.documents.get(&uri).await else {
            return Ok(None);
        };

        let line = position.line + 1;
        let column = position.character + 1;

        let items: Vec<CompletionItem> = nxd_member_completion_items(&source, line, column)
            .or_else(|| nxd_object_initializer_completion_items(&source, line, column))
            .unwrap_or_else(|| {
                let mut items =
                    nxd_scope_completion_items(&source, line, column).unwrap_or_default();

                items.extend(nxd_completion_items());

                items
            });

        Ok(Some(CompletionResponse::Array(items)))
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let text_document_position = params.text_document_position_params;

        let uri = text_document_position.text_document.uri;

        let position = text_document_position.position;

        let Some(source) = self.documents.get(&uri).await else {
            return Ok(None);
        };

        let info = hover_at(&source, position.line + 1, position.character + 1);

        Ok(info.map(hover_info_to_lsp))
    }
}
