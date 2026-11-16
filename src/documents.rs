use std::collections::HashMap;

use tokio::sync::RwLock;
use tower_lsp_server::ls_types::Uri;

#[derive(Debug, Default)]
pub struct DocumentStore {
    documents: RwLock<HashMap<Uri, String>>,
}

impl DocumentStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn insert(&self, uri: Uri, source: String) {
        self.documents.write().await.insert(uri, source);
    }

    pub async fn remove(&self, uri: &Uri) {
        self.documents.write().await.remove(uri);
    }
}
