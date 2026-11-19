use nxd_language::frontend::completion::{completion_words, member_completion_words};
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

pub fn nxd_member_completion_items(
    source: &str,
    line: u32,
    column: u32,
) -> Option<Vec<CompletionItem>> {
    member_completion_words(source, line, column).map(|words| {
        words
            .into_iter()
            .map(|word| CompletionItem {
                label: word,
                kind: Some(CompletionItemKind::FIELD),
                ..CompletionItem::default()
            })
            .collect()
    })
}
