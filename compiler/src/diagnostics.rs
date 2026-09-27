use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum DiagnosticSeverity {
    Error,
    Warning,
    Information,
}

#[derive(Debug, Serialize)]
pub struct Diagnostic {
    pub code: String,
    pub message: String,
    pub severity: DiagnosticSeverity,
    pub line: usize,
    pub column: usize,
}

impl Diagnostic {
    pub fn format_lsp(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }
}
