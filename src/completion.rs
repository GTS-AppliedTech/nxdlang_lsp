use nxd_language::frontend::completion::completion_words;

use tower_lsp_server::ls_types::{CompletionItem, CompletionItemKind};

pub fn nxd_completion_items() -> Vec<CompletionItem> {
    completion_words()
        .into_iter()
        .map(|word| CompletionItem {
            label: word,
            kind: Some(CompletionItemKind::KEYWORD),
            ..CompletionItem::default()
        })
        .collect()
}
