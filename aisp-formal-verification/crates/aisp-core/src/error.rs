//! Error types for AISP parsing and validation

use thiserror::Error;

/// Main error type for AISP operations
#[derive(Error, Debug, Clone, PartialEq)]
pub enum AispError {
    #[error("Parse error at line {line}, column {column}: {message}")]
    ParseError {
        line: usize,
        column: usize,
        message: String,
    },

    #[error("Semantic error: {message}")]
    SemanticError { message: String },

    #[error("Validation error: {message}")]
    ValidationError { message: String },

    #[error("Missing required block: {block_name}")]
    MissingBlock { block_name: String },

    #[error("Invalid block content in {block_name}: {message}")]
    InvalidBlock { block_name: String, message: String },

    #[error("Type error: {message}")]
    TypeError { message: String },

    #[error("Undefined symbol: {symbol}")]
    UndefinedSymbol { symbol: String },

    #[error("Ambiguity too high: {actual:.3} > {threshold:.3}")]
    AmbiguityError { actual: f64, threshold: f64 },

    #[error("Document too large: {size} bytes > {max} bytes")]
    DocumentTooLarge { size: usize, max: usize },

    #[error("Unsupported AISP version: {version}")]
    UnsupportedVersion { version: String },

    #[error("IO error: {message}")]
    IoError { message: String },

    #[error("Z3 error: {message}")]
    Z3Error { message: String },

    #[error("Verification failed: {0}")]
    VerificationFailed(String),
}

impl AispError {
    /// Create a parse error at a specific location
    pub fn parse_error(line: usize, column: usize, message: impl Into<String>) -> Self {
        Self::ParseError {
            line,
            column,
            message: message.into(),
        }
    }

    /// Create a semantic error
    pub fn semantic_error(message: impl Into<String>) -> Self {
        Self::SemanticError {
            message: message.into(),
        }
    }

    /// Create a validation error
    pub fn validation_error(message: impl Into<String>) -> Self {
        Self::ValidationError {
            message: message.into(),
        }
    }

    /// Check if this error is recoverable
    pub fn is_recoverable(&self) -> bool {
        match self {
            Self::ParseError { .. } => false,
            Self::SemanticError { .. } => false,
            Self::ValidationError { .. } => true,
            Self::MissingBlock { .. } => false,
            Self::InvalidBlock { .. } => false,
            Self::TypeError { .. } => false,
            Self::UndefinedSymbol { .. } => true,
            Self::AmbiguityError { .. } => true,
            Self::DocumentTooLarge { .. } => false,
            Self::UnsupportedVersion { .. } => false,
            Self::IoError { .. } => false,
            Self::Z3Error { .. } => true,
            Self::VerificationFailed(_) => true,
        }
    }
}

impl From<std::fmt::Error> for AispError {
    fn from(err: std::fmt::Error) -> Self {
        Self::IoError {
            message: format!("Format error: {}", err),
        }
    }
}

/// Result type for AISP operations
pub type AispResult<T> = Result<T, AispError>;

/// Warning that doesn't prevent validation but should be noted
#[derive(Debug, Clone, PartialEq)]
pub struct AispWarning {
    pub message: String,
    pub line: Option<usize>,
    pub severity: WarningSeverity,
}

#[derive(Debug, Clone, PartialEq)]
pub enum WarningSeverity {
    Info,
    Warning,
    Error,
}

impl AispWarning {
    pub fn info(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            line: None,
            severity: WarningSeverity::Info,
        }
    }

    pub fn warning(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            line: None,
            severity: WarningSeverity::Warning,
        }
    }

    pub fn error(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            line: None,
            severity: WarningSeverity::Error,
        }
    }

    pub fn with_line(mut self, line: usize) -> Self {
        self.line = Some(line);
        self
    }
}

impl std::fmt::Display for AispWarning {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.line {
            Some(line) => write!(f, "{}: {} (line {})", self.severity, self.message, line),
            None => write!(f, "{}: {}", self.severity, self.message),
        }
    }
}

impl std::fmt::Display for WarningSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Info => write!(f, "Info"),
            Self::Warning => write!(f, "Warning"),
            Self::Error => write!(f, "Error"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let error = AispError::parse_error(10, 5, "Invalid symbol");
        match error {
            AispError::ParseError { line, column, message } => {
                assert_eq!(line, 10);
                assert_eq!(column, 5);
                assert_eq!(message, "Invalid symbol");
            }
            _ => panic!("Expected parse error"),
        }
    }

    #[test]
    fn test_error_recoverability() {
        assert!(!AispError::parse_error(1, 1, "test").is_recoverable());
        assert!(AispError::validation_error("test").is_recoverable());
        assert!(AispError::UndefinedSymbol { symbol: "test".to_string() }.is_recoverable());
    }
}