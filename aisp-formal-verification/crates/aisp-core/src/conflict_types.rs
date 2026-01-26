//! Shared Conflict Types and Severity Levels
//!
//! This module defines common conflict types and severity levels 
//! used across different analysis modules to prevent re-export conflicts.

/// Conflict severity levels for different types of analysis conflicts
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ConflictSeverity {
    /// Informational notice
    Info,
    /// Minor issue that doesn't affect correctness
    Minor,
    /// Warning that should be addressed
    Warning,
    /// Major issue that affects functionality
    Major,
    /// Error that prevents validation
    Error,
    /// Critical error that breaks the system
    Critical,
}

impl ConflictSeverity {
    /// Get numeric weight for severity calculation
    pub fn weight(&self) -> f64 {
        match self {
            Self::Info => 0.05,
            Self::Minor => 0.1,
            Self::Warning => 0.2,
            Self::Major => 0.3,
            Self::Error => 0.5,
            Self::Critical => 1.0,
        }
    }

    /// Check if severity indicates an error condition
    pub fn is_error(&self) -> bool {
        matches!(self, Self::Error | Self::Critical)
    }

    /// Check if severity indicates a warning or worse
    pub fn is_warning_or_worse(&self) -> bool {
        *self >= Self::Warning
    }
}

impl std::fmt::Display for ConflictSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Info => write!(f, "Info"),
            Self::Minor => write!(f, "Minor"),
            Self::Warning => write!(f, "Warning"),
            Self::Major => write!(f, "Major"),
            Self::Error => write!(f, "Error"),
            Self::Critical => write!(f, "Critical"),
        }
    }
}