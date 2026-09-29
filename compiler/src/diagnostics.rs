use serde::Serialize;
use std::fmt;

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

#[derive(Debug, Clone)]
pub enum QclDiagnostic {
    DimensionMismatch {
        expected: String,
        actual: String,
    },
    IncompatibleFrames {
        lhs: String,
        rhs: String,
        op: String,
    },
    LegacyQclDeprecationWarning {
        feature: String,
        recommendation: String,
    },
}

impl fmt::Display for QclDiagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            QclDiagnostic::DimensionMismatch { expected, actual } => {
                write!(
                    f,
                    "[E001] Dimension Mismatch: Expected {}, found {}",
                    expected, actual
                )
            }
            QclDiagnostic::IncompatibleFrames { lhs, rhs, op } => {
                write!(
                    f,
                    "[E002] Spatial Frame Incompatibility: Cannot perform '{}' between frame '{}' and frame '{}'",
                    op, lhs, rhs
                )
            }
            QclDiagnostic::LegacyQclDeprecationWarning { feature, recommendation } => {
                write!(
                    f,
                    "[W001] Legacy Quantum Syntax '{}' detected. Suggestion: Migrate to {}",
                    feature, recommendation
                )
            }
        }
    }
}

impl std::error::Error for QclDiagnostic {}
