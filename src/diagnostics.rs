use nxd_language::diagnostics::severity::DiagnosticSeverity as NxdSeverity;
use nxd_language::diagnostics::validation::LanguageDiagnostic;

use tower_lsp_server::ls_types::{Diagnostic, DiagnosticSeverity, NumberOrString, Position, Range};

pub fn language_diagnostic_to_lsp(diagnostic: &LanguageDiagnostic) -> Diagnostic {
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
