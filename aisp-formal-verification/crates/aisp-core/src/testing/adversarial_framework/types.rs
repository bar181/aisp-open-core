//! Adversarial Testing Types and Structures
//!
//! Defines core types for adversarial attack testing including attack vectors,
//! severity levels, and result structures following SRP architecture.

use crate::parser::robust_parser::{ParseResult, SecuritySeverity};
use std::fmt;

/// Attack severity levels
#[derive(Debug, Clone, PartialEq)]
pub enum AttackSeverity {
    Low,
    Medium, 
    High,
    Critical,
}

/// Attack category classifications
#[derive(Debug, Clone, PartialEq)]
pub enum AttackCategory {
    ParseBypass,
    UnicodeConfusion,
    BoundaryExploitation,
    ResourceExhaustion,
    InjectionAttack,
    DeceptionAttack,
}

/// Expected behavior for attack tests
#[derive(Debug, Clone, PartialEq)]
pub enum ExpectedBehavior {
    ShouldReject,
    ShouldParseWithWarnings,
    ShouldTriggerSecurity,
    ShouldBypassValidation,
}

/// Unicode attack detection methods
#[derive(Debug, Clone, PartialEq)]
pub enum DetectionMethod {
    UnicodeNormalization,
    VisualSimilarity,
    StatisticalAnalysis,
    PatternMatching,
}

/// Boundary condition types
#[derive(Debug, Clone, PartialEq)]
pub enum BoundaryType {
    BlockDelimiters,
    StringLiterals,
    NumericOverflow,
    NestingDepth,
    UnicodeCodePoints,
}

/// Failure mode classifications
#[derive(Debug, Clone, PartialEq)]
pub enum FailureMode {
    ParseError,
    SecurityViolation,
    MemoryExhaustion,
    InfiniteLoop,
    IncorrectBehavior,
}

/// Parser component targets
#[derive(Debug, Clone, PartialEq)]
pub enum ParserComponent {
    Lexer,
    GrammarParser,
    ASTBuilder,
    SemanticAnalyzer,
    UnicodeProcessor,
}

/// Recovery behavior types
#[derive(Debug, Clone, PartialEq)]
pub enum RecoveryBehavior {
    GracefulDegradation,
    PartialRecovery,
    FailFast,
    SecurityLockdown,
}

/// Resource exhaustion targets
#[derive(Debug, Clone, PartialEq)]
pub enum ResourceTarget {
    Memory,
    CpuTime,
    StackDepth,
    FileDescriptors,
    NetworkConnections,
}

/// Payload generation strategies
#[derive(Debug, Clone, PartialEq)]
pub enum PayloadGenerator {
    ExponentialNesting,
    RepeatedPattern,
    LargeInput,
    ComplexUnicode,
    CyclicReference,
}

/// Injection attack contexts
#[derive(Debug, Clone, PartialEq)]
pub enum InjectionContext {
    StringLiteral,
    Identifier,
    MathematicalExpression,
    BlockContent,
    MetaData,
}

/// Recommendation priority levels
#[derive(Debug, Clone, PartialEq)]
pub enum RecommendationPriority {
    Immediate,
    Short_term,
    Medium_term,
    Long_term,
}

/// Implementation effort estimates
#[derive(Debug, Clone, PartialEq)]
pub enum ImplementationEffort {
    Minimal,
    Low,
    Medium,
    High,
    Extensive,
}

/// Resource limits for attack testing
#[derive(Debug, Clone)]
pub struct ResourceLimits {
    pub memory_mb: usize,
    pub cpu_time_ms: usize,
    pub max_depth: usize,
    pub max_iterations: usize,
}

/// Performance impact measurements
#[derive(Debug, Clone)]
pub struct PerformanceImpact {
    pub parsing_time_ms: u64,
    pub memory_usage_mb: usize,
    pub cpu_usage_percent: f64,
}

/// Vulnerability summary statistics
#[derive(Debug, Clone)]
pub struct VulnerabilitySummary {
    pub critical_issues: usize,
    pub high_risk_issues: usize,
    pub medium_risk_issues: usize,
    pub low_risk_issues: usize,
    pub most_critical_attack: Option<String>,
    pub common_weakness_patterns: Vec<String>,
}

/// Security improvement recommendation
#[derive(Debug, Clone)]
pub struct SecurityRecommendation {
    pub priority: RecommendationPriority,
    pub category: String,
    pub description: String,
    pub implementation_effort: ImplementationEffort,
    pub risk_reduction: f64,
}

/// Parse bypass attack vector
#[derive(Debug, Clone)]
pub struct ParseBypassAttack {
    pub name: &'static str,
    pub description: &'static str,
    pub attack_payload: String,
    pub expected_behavior: ExpectedBehavior,
    pub severity: AttackSeverity,
    pub attack_category: AttackCategory,
}

/// Unicode confusion and spoofing attack
#[derive(Debug, Clone)]
pub struct UnicodeAttack {
    pub name: &'static str,
    pub description: &'static str,
    pub malicious_payload: String,
    pub spoofed_target: String,
    pub detection_method: DetectionMethod,
    pub mitigation: String,
}

/// Boundary condition exploitation attack
#[derive(Debug, Clone)]
pub struct BoundaryAttack {
    pub name: &'static str,
    pub description: &'static str,
    pub boundary_type: BoundaryType,
    pub attack_vector: String,
    pub expected_failure_mode: FailureMode,
}

/// Malformed document attack
#[derive(Debug, Clone)]
pub struct MalformedDocumentAttack {
    pub name: &'static str,
    pub description: &'static str,
    pub malformed_payload: String,
    pub target_parser_component: ParserComponent,
    pub expected_recovery_behavior: RecoveryBehavior,
}

/// Resource exhaustion attack
#[derive(Debug, Clone)]
pub struct ResourceExhaustionAttack {
    pub name: &'static str,
    pub description: &'static str,
    pub payload_generator: PayloadGenerator,
    pub resource_target: ResourceTarget,
    pub threshold_limits: ResourceLimits,
}

/// Code/script injection attack
#[derive(Debug, Clone)]
pub struct InjectionAttack {
    pub name: &'static str,
    pub description: &'static str,
    pub injection_payload: String,
    pub injection_context: InjectionContext,
    pub expected_sanitization: String,
}

/// Attack execution result
#[derive(Debug, Clone)]
pub struct AttackResult {
    pub attack_name: String,
    pub attack_category: AttackCategory,
    pub success: bool,
    pub bypass_achieved: bool,
    pub security_impact: SecuritySeverity,
    pub parser_response: ParseResult,
    pub detection_triggered: bool,
    pub mitigation_effective: bool,
    pub performance_impact: PerformanceImpact,
    pub details: String,
}

/// Comprehensive security assessment report
#[derive(Debug, Clone)]
pub struct SecurityAssessmentReport {
    pub timestamp: String,
    pub total_attacks: usize,
    pub successful_attacks: usize,
    pub bypasses_achieved: usize,
    pub critical_vulnerabilities: usize,
    pub attack_results: Vec<AttackResult>,
    pub vulnerability_summary: VulnerabilitySummary,
    pub recommendations: Vec<SecurityRecommendation>,
    pub overall_security_score: f64,
}

impl fmt::Display for AttackSeverity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AttackSeverity::Low => write!(f, "LOW"),
            AttackSeverity::Medium => write!(f, "MEDIUM"),
            AttackSeverity::High => write!(f, "HIGH"),
            AttackSeverity::Critical => write!(f, "CRITICAL"),
        }
    }
}

impl fmt::Display for AttackCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AttackCategory::ParseBypass => write!(f, "Parse Bypass"),
            AttackCategory::UnicodeConfusion => write!(f, "Unicode Confusion"),
            AttackCategory::BoundaryExploitation => write!(f, "Boundary Exploitation"),
            AttackCategory::ResourceExhaustion => write!(f, "Resource Exhaustion"),
            AttackCategory::InjectionAttack => write!(f, "Injection Attack"),
            AttackCategory::DeceptionAttack => write!(f, "Deception Attack"),
        }
    }
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self {
            memory_mb: 100,
            cpu_time_ms: 5000,
            max_depth: 50,
            max_iterations: 10000,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_attack_severity_display() {
        assert_eq!(format!("{}", AttackSeverity::Critical), "CRITICAL");
        assert_eq!(format!("{}", AttackSeverity::High), "HIGH");
        assert_eq!(format!("{}", AttackSeverity::Medium), "MEDIUM");
        assert_eq!(format!("{}", AttackSeverity::Low), "LOW");
    }

    #[test]
    fn test_attack_category_display() {
        assert_eq!(format!("{}", AttackCategory::ParseBypass), "Parse Bypass");
        assert_eq!(format!("{}", AttackCategory::UnicodeConfusion), "Unicode Confusion");
        assert_eq!(format!("{}", AttackCategory::InjectionAttack), "Injection Attack");
    }

    #[test]
    fn test_resource_limits_default() {
        let limits = ResourceLimits::default();
        assert_eq!(limits.memory_mb, 100);
        assert_eq!(limits.cpu_time_ms, 5000);
        assert_eq!(limits.max_depth, 50);
        assert_eq!(limits.max_iterations, 10000);
    }

    #[test]
    fn test_attack_severity_ordering() {
        assert!(AttackSeverity::Critical != AttackSeverity::Low);
        assert!(AttackSeverity::High != AttackSeverity::Medium);
    }

    #[test]
    fn test_enum_cloning() {
        let severity = AttackSeverity::High;
        let cloned = severity.clone();
        assert_eq!(severity, cloned);
        
        let category = AttackCategory::ParseBypass;
        let cloned_cat = category.clone();
        assert_eq!(category, cloned_cat);
    }
}