//! Core Canonical AST Types
//!
//! Defines the fundamental types for the canonical AISP AST representation
//! including spans, headers, and basic type system following SRP architecture.

use serde::{Serialize, Deserialize};
use std::collections::HashMap;

/// Source location span information
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Span {
    pub start: usize,
    pub end: usize,
    pub line: usize,
    pub column: usize,
}

/// Document header information
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DocumentHeader {
    /// AISP version (e.g., "5.1")
    pub version: String,
    /// Document name
    pub name: String,
    /// Creation date
    pub date: String,
    /// Optional metadata
    pub metadata: Option<HeaderMetadata>,
}

/// Additional header metadata
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HeaderMetadata {
    pub author: Option<String>,
    pub description: Option<String>,
    pub tags: Vec<String>,
}

/// Document metadata 
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DocumentMetadata {
    pub domain: Option<String>,
    pub protocol: Option<String>,
}

/// Type definition with canonical structure
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TypeDefinition {
    pub name: String,
    pub type_expr: TypeExpression,
    pub span: Option<Span>,
}

/// Type expression for type system representation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TypeExpression {
    Basic(BasicType),
    Set(Box<TypeExpression>),
    Union(Vec<TypeExpression>),
    Product(Vec<TypeExpression>),
    Function {
        params: Vec<TypeExpression>,
        return_type: Box<TypeExpression>,
    },
}

/// Basic type enumeration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BasicType {
    Natural,
    Integer,
    Real,
    Boolean,
    String,
    Symbol,
    VectorSpace(usize),
    RealVector,
    DirectSum,
    MathematicalStructure(String),
    Custom(String),
}

/// Meta entry in meta blocks
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MetaEntry {
    pub key: String,
    pub value: MetaValue,
    pub span: Option<Span>,
}

/// Meta value types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MetaValue {
    String(String),
    Number(f64),
    Boolean(bool),
    List(Vec<MetaValue>),
    Map(HashMap<String, MetaValue>),
    Constraint(LogicalExpression),
}

/// Logical rule with quantifiers and expressions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LogicalRule {
    pub quantifier: Option<Quantifier>,
    pub expression: LogicalExpression,
    pub raw_text: String,
    pub span: Option<Span>,
}

/// Logical expression
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
    /// Raw text (for fallback parsing)
    Raw(String),
}

/// Constant values
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConstantValue {
    Number(f64),
    String(String),
    Boolean(bool),
}

/// Binary logical operators
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
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
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UnaryOperator {
    /// Negation (¬)
    Not,
    /// Power set (𝒫)
    PowerSet,
}

/// Temporal operators for temporal logic
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
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

/// Quantifier (∀, ∃)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Quantifier {
    pub kind: QuantifierKind,
    pub variable: String,
    pub domain: Option<String>,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum QuantifierKind {
    /// Universal quantifier (∀)
    Universal,
    /// Existential quantifier (∃)
    Existential,
}

/// Lambda expression
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LambdaExpression {
    pub parameters: Vec<String>,
    pub body: LogicalExpression,
    pub span: Option<Span>,
}

/// Function definition with lambda expressions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FunctionDefinition {
    pub name: String,
    pub lambda: LambdaExpression,
    pub raw_text: String,
    pub span: Option<Span>,
}

/// Function parameter representation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Parameter {
    pub name: String,
    pub param_type: Option<TypeExpression>,
    pub span: Option<Span>,
}

impl Span {
    /// Create new span
    pub fn new(start: usize, end: usize, line: usize, column: usize) -> Self {
        Self { start, end, line, column }
    }

    /// Check if span contains position
    pub fn contains(&self, pos: usize) -> bool {
        pos >= self.start && pos < self.end
    }

    /// Get span length
    pub fn len(&self) -> usize {
        self.end - self.start
    }

    /// Check if span is empty
    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }
}

impl DocumentHeader {
    /// Create new document header
    pub fn new(version: String, name: String, date: String) -> Self {
        Self {
            version,
            name,
            date,
            metadata: None,
        }
    }

    /// Set metadata
    pub fn with_metadata(mut self, metadata: HeaderMetadata) -> Self {
        self.metadata = Some(metadata);
        self
    }
}

impl HeaderMetadata {
    /// Create new header metadata
    pub fn new() -> Self {
        Self {
            author: None,
            description: None,
            tags: Vec::new(),
        }
    }

    /// Set author
    pub fn with_author(mut self, author: String) -> Self {
        self.author = Some(author);
        self
    }

    /// Set description
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// Add tag
    pub fn add_tag(mut self, tag: String) -> Self {
        self.tags.push(tag);
        self
    }
}

impl Default for HeaderMetadata {
    fn default() -> Self {
        Self::new()
    }
}

impl DocumentMetadata {
    /// Create new document metadata
    pub fn new() -> Self {
        Self {
            domain: None,
            protocol: None,
        }
    }

    /// Set domain
    pub fn with_domain(mut self, domain: String) -> Self {
        self.domain = Some(domain);
        self
    }

    /// Set protocol
    pub fn with_protocol(mut self, protocol: String) -> Self {
        self.protocol = Some(protocol);
        self
    }
}

impl Default for DocumentMetadata {
    fn default() -> Self {
        Self::new()
    }
}

impl TypeExpression {
    /// Check if type expression is basic
    pub fn is_basic(&self) -> bool {
        matches!(self, TypeExpression::Basic(_))
    }

    /// Get basic type if this is a basic type expression
    pub fn as_basic(&self) -> Option<&BasicType> {
        match self {
            TypeExpression::Basic(basic) => Some(basic),
            _ => None,
        }
    }

    /// Check if type expression is a function type
    pub fn is_function(&self) -> bool {
        matches!(self, TypeExpression::Function { .. })
    }
}

impl BasicType {
    /// Check if type is numeric
    pub fn is_numeric(&self) -> bool {
        matches!(self, 
            BasicType::Natural | 
            BasicType::Integer | 
            BasicType::Real
        )
    }

    /// Check if type is a collection
    pub fn is_collection(&self) -> bool {
        matches!(self, BasicType::VectorSpace(_) | BasicType::RealVector)
    }

    /// Get type name as string
    pub fn type_name(&self) -> &str {
        match self {
            BasicType::Natural => "Natural",
            BasicType::Integer => "Integer",
            BasicType::Real => "Real",
            BasicType::Boolean => "Boolean",
            BasicType::String => "String",
            BasicType::Symbol => "Symbol",
            BasicType::VectorSpace(_) => "VectorSpace",
            BasicType::RealVector => "RealVector",
            BasicType::DirectSum => "DirectSum",
            BasicType::MathematicalStructure(_) => "MathematicalStructure",
            BasicType::Custom(name) => name,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_span_operations() {
        let span = Span::new(10, 20, 1, 10);
        assert_eq!(span.len(), 10);
        assert!(!span.is_empty());
        assert!(span.contains(15));
        assert!(!span.contains(25));
    }

    #[test]
    fn test_document_header_creation() {
        let header = DocumentHeader::new(
            "5.1".to_string(),
            "test".to_string(),
            "2026-01-27".to_string()
        );
        assert_eq!(header.version, "5.1");
        assert_eq!(header.name, "test");
        assert_eq!(header.date, "2026-01-27");
        assert!(header.metadata.is_none());
    }

    #[test]
    fn test_header_metadata_builder() {
        let metadata = HeaderMetadata::new()
            .with_author("Test Author".to_string())
            .with_description("Test Description".to_string())
            .add_tag("test".to_string())
            .add_tag("demo".to_string());
        
        assert_eq!(metadata.author, Some("Test Author".to_string()));
        assert_eq!(metadata.description, Some("Test Description".to_string()));
        assert_eq!(metadata.tags, vec!["test".to_string(), "demo".to_string()]);
    }

    #[test]
    fn test_type_expression_checks() {
        let basic_type = TypeExpression::Basic(BasicType::Integer);
        assert!(basic_type.is_basic());
        assert!(!basic_type.is_function());
        assert_eq!(basic_type.as_basic(), Some(&BasicType::Integer));

        let function_type = TypeExpression::Function {
            params: vec![TypeExpression::Basic(BasicType::Integer)],
            return_type: Box::new(TypeExpression::Basic(BasicType::Real)),
        };
        assert!(!function_type.is_basic());
        assert!(function_type.is_function());
    }

    #[test]
    fn test_basic_type_properties() {
        assert!(BasicType::Integer.is_numeric());
        assert!(!BasicType::String.is_numeric());
        assert!(BasicType::VectorSpace(3).is_collection());
        assert!(!BasicType::Boolean.is_collection());
        assert_eq!(BasicType::Real.type_name(), "Real");
    }

    #[test]
    fn test_document_metadata_builder() {
        let metadata = DocumentMetadata::new()
            .with_domain("mathematics".to_string())
            .with_protocol("aisp".to_string());
        
        assert_eq!(metadata.domain, Some("mathematics".to_string()));
        assert_eq!(metadata.protocol, Some("aisp".to_string()));
    }
}