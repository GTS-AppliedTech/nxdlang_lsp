use nxd_language::frontend::completion::{
    LanguageCompletionKind, ScopeCompletionKind, completion_words, function_completion_words,
    member_completion_words, object_initializer_completion_words, scope_completion_words,
};
use tower_lsp_server::ls_types::{CompletionItem, CompletionItemKind};

pub fn nxd_completion_items() -> Vec<CompletionItem> {
    completion_words()
        .into_iter()
        .map(|item| {
            let kind = match item.kind {
                LanguageCompletionKind::Keyword => CompletionItemKind::KEYWORD,

                LanguageCompletionKind::Operator => CompletionItemKind::OPERATOR,
            };

            CompletionItem {
                label: item.name,
                kind: Some(kind),
                ..CompletionItem::default()
            }
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

pub fn nxd_object_initializer_completion_items(
    source: &str,
    line: u32,
    column: u32,
) -> Option<Vec<CompletionItem>> {
    object_initializer_completion_words(source, line, column).map(|words| {
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

pub fn nxd_scope_completion_items(
    source: &str,
    line: u32,
    column: u32,
) -> Option<Vec<CompletionItem>> {
    scope_completion_words(source, line, column).map(|items| {
        items
            .into_iter()
            .map(|item| {
                let kind = match item.kind {
                    ScopeCompletionKind::Parameter => CompletionItemKind::VARIABLE,

                    ScopeCompletionKind::Variable => CompletionItemKind::VARIABLE,

                    ScopeCompletionKind::Constant => CompletionItemKind::CONSTANT,
                };

                CompletionItem {
                    label: item.name,
                    kind: Some(kind),
                    detail: Some(item.ty),
                    ..CompletionItem::default()
                }
            })
            .collect()
    })
}

pub fn nxd_function_completion_items(
    source: &str,
    line: u32,
    column: u32,
) -> Option<Vec<CompletionItem>> {
    function_completion_words(source, line, column).map(|functions| {
        functions
            .into_iter()
            .map(|function| {
                let params = function.params.join(", ");

                let detail = if function.is_async {
                    format!("async ({}) -> {}", params, function.return_type,)
                } else {
                    format!("({}) -> {}", params, function.return_type,)
                };

                CompletionItem {
                    label: function.name,
                    kind: Some(CompletionItemKind::FUNCTION),
                    detail: Some(detail),
                    ..CompletionItem::default()
                }
            })
            .collect()
    })
}
