use nxd_language::frontend::completion::{
    LanguageCompletionKind, ScopeCompletionKind, completion_type_compatible, completion_words,
    expected_binary_type_at_position, function_completion_words, member_completion_words,
    object_initializer_completion_words, scope_completion_words,
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
    let expected_type = expected_binary_type_at_position(source, line, column);

    scope_completion_words(source, line, column).map(|items| {
        items
            .into_iter()
            .filter(|item| {
                expected_type
                    .as_deref()
                    .map(|expected| completion_type_compatible(expected, &item.ty))
                    .unwrap_or(true)
            })
            .map(|item| {
                let kind = match item.kind {
                    ScopeCompletionKind::Parameter => CompletionItemKind::VARIABLE,
                    ScopeCompletionKind::Variable => CompletionItemKind::VARIABLE,
                    ScopeCompletionKind::Constant => CompletionItemKind::CONSTANT,
                };

                let preferred = expected_type.as_deref() == Some(item.ty.as_str());

                let sort_text = if preferred {
                    format!("0_{}", item.name)
                } else {
                    format!("1_{}", item.name)
                };

                CompletionItem {
                    label: item.name,
                    kind: Some(kind),
                    detail: Some(item.ty),
                    sort_text: Some(sort_text),
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
    let expected_type = expected_binary_type_at_position(source, line, column);

    function_completion_words(source, line, column).map(|functions| {
        functions
            .into_iter()
            .filter(|function| {
                expected_type
                    .as_deref()
                    .map(|expected| completion_type_compatible(expected, &function.return_type))
                    .unwrap_or(true)
            })
            .map(|function| {
                let params = function.params.join(", ");

                let detail = if function.is_async {
                    format!("async ({}) -> {}", params, function.return_type,)
                } else {
                    format!("({}) -> {}", params, function.return_type,)
                };

                let preferred = expected_type.as_deref() == Some(function.return_type.as_str());

                let sort_text = if preferred {
                    format!("0_{}", function.name)
                } else {
                    format!("1_{}", function.name)
                };

                CompletionItem {
                    label: function.name,
                    kind: Some(CompletionItemKind::FUNCTION),
                    detail: Some(detail),
                    sort_text: Some(sort_text),
                    ..CompletionItem::default()
                }
            })
            .collect()
    })
}
