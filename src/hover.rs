use nxd_language::frontend::hover::HoverInfo;

use tower_lsp_server::ls_types::{Hover, HoverContents, MarkupContent, MarkupKind};

pub fn hover_info_to_lsp(info: HoverInfo) -> Hover {
    Hover {
        contents: HoverContents::Markup(MarkupContent {
            kind: MarkupKind::Markdown,
            value: format!("**{}**\n\n{}", info.label, info.detail,),
        }),

        range: None,
    }
}
