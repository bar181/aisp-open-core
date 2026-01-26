//! Abstract Syntax Tree for AISP documents
//! 
//! Provides type-safe representations of all AISP constructs with
//! zero-copy parsing where possible.

use std::collections::HashMap;
use std::fmt;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Complete AISP document AST
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AispDocument {
    /// Document header (𝔸5.1.name@date)
    pub header: DocumentHeader,
    /// Document metadata
    pub metadata: DocumentMetadata,
    /// All blocks in the document
    pub blocks: Vec<AispBlock>,
    /// Source location information
    pub span: Span,
}

/// Document header information
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DocumentHeader {
    /// AISP version (e.g., "5.1")
    pub version: String,
    /// Document name/identifier
    pub name: String,
    /// Creation date
    pub date: String,
    /// Optional additional metadata
    pub metadata: Option<String>,
}

/// Document-level metadata
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DocumentMetadata {
    /// Domain classification (γ)
    pub domain: Option<String>,
    /// Protocol type (ρ)
    pub protocol: Option<String>,
}

/// AISP block types
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum AispBlock {
    /// Meta block (⟦Ω:Meta⟧)
    Meta(MetaBlock),
    /// Types block (⟦Σ:Types⟧)
    Types(TypesBlock),
    /// Rules block (⟦Γ:Rules⟧)
    Rules(RulesBlock),
    /// Functions block (⟦Λ:Funcs⟧)
    Functions(FunctionsBlock),
    /// Evidence block (⟦Ε⟧)
    Evidence(EvidenceBlock),
}

impl AispBlock {
    /// Get the block type name
    pub fn block_type(&self) -> &'static str {
        match self {
            Self::Meta(_) => "Meta",
            Self::Types(_) => "Types",
            Self::Rules(_) => "Rules",
            Self::Functions(_) => "Functions",
            Self::Evidence(_) => "Evidence",
        }
    }

    /// Check if this is a required block
    pub fn is_required(&self) -> bool {
        match self {
            Self::Meta(_) | Self::Types(_) | Self::Rules(_) | 
            Self::Functions(_) | Self::Evidence(_) => true,
        }
    }
}

/// Meta block containing document metadata
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MetaBlock {
    pub entries: HashMap<String, MetaEntry>,
    pub span: Span,
}

/// Entry in meta block
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MetaEntry {
    pub key: String,
    pub value: MetaValue,
    pub span: Span,
}

/// Meta value types
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum MetaValue {
    String(String),
    Number(f64),
    Boolean(bool),
    Constraint(LogicalExpression),
}

/// Types block defining data types
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TypesBlock {
    pub definitions: HashMap<String, TypeDefinition>,
    pub span: Span,
}

/// Type definition
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TypeDefinition {
    pub name: String,
    pub type_expr: TypeExpression,
    pub span: Span,
}

/// Type expression
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TypeExpression {
    /// Basic type (ℕ, ℤ, ℝ, 𝔹, 𝕊)
    Basic(BasicType),
    /// Enumeration {A, B, C}
    Enumeration(Vec<String>),
    /// Array type Type[n]
    Array {
        element_type: Box<TypeExpression>,
        size: Option<usize>,
    },
    /// Tuple type (A, B, C)
    Tuple(Vec<TypeExpression>),
    /// Function type A → B
    Function {
        input: Box<TypeExpression>,
        output: Box<TypeExpression>,
    },
    /// Generic type with parameters
    Generic {
        name: String,
        parameters: Vec<TypeExpression>,
    },
    /// Type reference
    Reference(String),
}

/// Basic AISP types
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum BasicType {
    /// Natural numbers (ℕ)
    Natural,
    /// Integers (ℤ)
    Integer,
    /// Real numbers (ℝ)
    Real,
    /// Booleans (𝔹)
    Boolean,
    /// Strings (𝕊)
    String,
}

/// Rules block containing logical rules
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RulesBlock {
    pub rules: Vec<LogicalRule>,
    pub span: Span,
}

/// Logical rule
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LogicalRule {
    pub quantifier: Option<Quantifier>,
    pub expression: LogicalExpression,
    pub span: Span,
}

/// Quantifier (∀, ∃)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Quantifier {
    pub kind: QuantifierKind,
    pub variable: String,
    pub domain: Option<String>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum QuantifierKind {
    /// Universal quantifier (∀)
    Universal,
    /// Existential quantifier (∃)
    Existential,
}

/// Logical expression
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum LogicalExpression {
    /// Variable reference
    Variable(String),
    /// Constant value
    Constant(ConstantValue),
    /// Binary operation
    Binary {
        op: BinaryOperator,
        left: Box<LogicalExpression>,
        right: Box<LogicalExpression>,
    },
    /// Unary operation
    Unary {
        op: UnaryOperator,
        operand: Box<LogicalExpression>,
    },
    /// Function application
    Application {
        function: String,
        arguments: Vec<LogicalExpression>,
    },
    /// Set membership
    Membership {
        element: Box<LogicalExpression>,
        set: Box<LogicalExpression>,
    },
    /// Temporal operator
    Temporal {
        op: TemporalOperator,
        operand: Box<LogicalExpression>,
    },
}

/// Constant values
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ConstantValue {
    Number(f64),
    String(String),
    Boolean(bool),
}

/// Binary logical operators
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum BinaryOperator {
    /// Definition (≜)
    Definition,
    /// Assignment (≔)
    Assignment,
    /// Equivalence (≡)
    Equivalence,
    /// Implication (⇒, →)
    Implication,
    /// Biconditional (⇔, ↔)
    Biconditional,
    /// Conjunction (∧)
    And,
    /// Disjunction (∨)
    Or,
    /// Exclusive or (⊕)
    Xor,
    /// Equality (=)
    Equals,
    /// Inequality (≠)
    NotEquals,
    /// Less than (<)
    LessThan,
    /// Less than or equal (≤)
    LessEqual,
    /// Greater than (>)
    GreaterThan,
    /// Greater than or equal (≥)
    GreaterEqual,
    /// Set union (∪)
    Union,
    /// Set intersection (∩)
    Intersection,
}

/// Unary logical operators
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum UnaryOperator {
    /// Negation (¬)
    Not,
    /// Power set (𝒫)
    PowerSet,
}

/// Temporal operators for temporal logic
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TemporalOperator {
    /// Always/Globally (□, G)
    Always,
    /// Eventually/Finally (◊, F)
    Eventually,
    /// Next (X)
    Next,
    /// Until (U)
    Until,
    /// Weak until (W)
    WeakUntil,
    /// Release (R)
    Release,
}

/// Functions block containing lambda expressions
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FunctionsBlock {
    pub functions: HashMap<String, FunctionDefinition>,
    pub span: Span,
}

/// Function definition
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FunctionDefinition {
    pub name: String,
    pub lambda: LambdaExpression,
    pub span: Span,
}

/// Lambda expression
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LambdaExpression {
    pub parameters: Vec<String>,
    pub body: LogicalExpression,
    pub span: Span,
}

/// Evidence block containing quality metrics
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EvidenceBlock {
    /// Semantic density (δ)
    pub delta: Option<f64>,
    /// Completeness (φ)
    pub phi: Option<f64>,
    /// Tier (τ)
    pub tau: Option<String>,
    /// Additional metrics
    pub metrics: HashMap<String, f64>,
    pub span: Span,
}

/// Source location information
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Span {
    pub start: Position,
    pub end: Position,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Position {
    pub line: usize,
    pub column: usize,
    pub offset: usize,
}

impl Span {
    pub fn new(start_line: usize, start_col: usize, end_line: usize, end_col: usize) -> Self {
        Self {
            start: Position {
                line: start_line,
                column: start_col,
                offset: 0,
            },
            end: Position {
                line: end_line,
                column: end_col,
                offset: 0,
            },
        }
    }
}

impl fmt::Display for BinaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = match self {
            Self::Definition => "≜",
            Self::Assignment => "≔",
            Self::Equivalence => "≡",
            Self::Implication => "⇒",
            Self::Biconditional => "⇔",
            Self::And => "∧",
            Self::Or => "∨",
            Self::Xor => "⊕",
            Self::Equals => "=",
            Self::NotEquals => "≠",
            Self::LessThan => "<",
            Self::LessEqual => "≤",
            Self::GreaterThan => ">",
            Self::GreaterEqual => "≥",
            Self::Union => "∪",
            Self::Intersection => "∩",
        };
        write!(f, "{}", symbol)
    }
}

impl fmt::Display for TemporalOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = match self {
            Self::Always => "□",
            Self::Eventually => "◊",
            Self::Next => "X",
            Self::Until => "U",
            Self::WeakUntil => "W",
            Self::Release => "R",
        };
        write!(f, "{}", symbol)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_type() {
        let meta_block = AispBlock::Meta(MetaBlock {
            entries: HashMap::new(),
            span: Span::new(1, 1, 1, 10),
        });
        
        assert_eq!(meta_block.block_type(), "Meta");
        assert!(meta_block.is_required());
    }

    #[test]
    fn test_binary_operator_display() {
        assert_eq!(format!("{}", BinaryOperator::Definition), "≜");
        assert_eq!(format!("{}", BinaryOperator::Implication), "⇒");
        assert_eq!(format!("{}", BinaryOperator::And), "∧");
    }

    #[test]
    fn test_temporal_operator_display() {
        assert_eq!(format!("{}", TemporalOperator::Always), "□");
        assert_eq!(format!("{}", TemporalOperator::Eventually), "◊");
        assert_eq!(format!("{}", TemporalOperator::Until), "U");
    }
}