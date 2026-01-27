//! Canonical AISP AST Types - Single Source of Truth
//! 
//! This module provides the unified, production-ready AST representation
//! that replaces both ast::AispDocument and robust_parser::AispDocument
//! to eliminate type system fragmentation.

// Note: Import from parent module after it's defined
use std::fmt;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

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

/// Canonical AISP Document representation - SINGLE SOURCE OF TRUTH
/// 
/// This replaces both `ast::AispDocument` and `robust_parser::AispDocument`
/// with a unified, production-ready type that all modules use consistently.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CanonicalAispDocument {
    pub header: DocumentHeader,
    pub metadata: DocumentMetadata,
    pub blocks: Vec<CanonicalAispBlock>,
    pub span: Option<Span>,
}

/// Canonical Block representation with consistent method access patterns
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CanonicalAispBlock {
    Meta(MetaBlock),
    Types(TypesBlock),
    Rules(RulesBlock), 
    Functions(FunctionsBlock),
    Evidence(EvidenceBlock),
}

impl CanonicalAispBlock {
    /// Get block type name - CANONICAL method access pattern
    pub fn block_type(&self) -> &'static str {
        match self {
            CanonicalAispBlock::Meta(_) => "Meta",
            CanonicalAispBlock::Types(_) => "Types",
            CanonicalAispBlock::Rules(_) => "Rules", 
            CanonicalAispBlock::Functions(_) => "Functions",
            CanonicalAispBlock::Evidence(_) => "Evidence",
        }
    }
    
    /// Get block as meta block if applicable
    pub fn as_meta(&self) -> Option<&MetaBlock> {
        match self {
            CanonicalAispBlock::Meta(meta) => Some(meta),
            _ => None,
        }
    }
    
    /// Get block as types block if applicable
    pub fn as_types(&self) -> Option<&TypesBlock> {
        match self {
            CanonicalAispBlock::Types(types) => Some(types),
            _ => None,
        }
    }
    
    /// Get block as rules block if applicable
    pub fn as_rules(&self) -> Option<&RulesBlock> {
        match self {
            CanonicalAispBlock::Rules(rules) => Some(rules),
            _ => None,
        }
    }
    
    /// Get block as functions block if applicable
    pub fn as_functions(&self) -> Option<&FunctionsBlock> {
        match self {
            CanonicalAispBlock::Functions(functions) => Some(functions),
            _ => None,
        }
    }
    
    /// Get block as evidence block if applicable
    pub fn as_evidence(&self) -> Option<&EvidenceBlock> {
        match self {
            CanonicalAispBlock::Evidence(evidence) => Some(evidence),
            _ => None,
        }
    }
}

/// Meta block for document metadata and configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MetaBlock {
    pub entries: Vec<String>,
    pub span: Option<Span>,
}

/// Types block for type definitions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TypesBlock {
    pub definitions: HashMap<String, TypeDefinition>,
    pub span: Option<Span>,
}

/// Rules block for logical rules and constraints
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RulesBlock {
    pub rules: Vec<String>,
    pub span: Option<Span>,
}

/// Functions block for function definitions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FunctionsBlock {
    pub functions: Vec<String>,
    pub span: Option<Span>,
}

/// Evidence block for validation evidence
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EvidenceBlock {
    pub delta: Option<f64>,
    pub phi: Option<u64>,
    pub tau: Option<String>,
    pub span: Option<Span>,
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
    Custom(String),
}

impl Default for CanonicalAispDocument {
    fn default() -> Self {
        Self {
            header: DocumentHeader {
                version: "5.1".to_string(),
                name: "default".to_string(),
                date: "2026-01-27".to_string(),
                metadata: None,
            },
            metadata: DocumentMetadata {
                domain: None,
                protocol: None,
            },
            blocks: Vec::new(),
            span: None,
        }
    }
}

impl CanonicalAispDocument {
    /// Create new document with header
    pub fn new(name: String, version: String, date: String) -> Self {
        Self {
            header: DocumentHeader {
                version,
                name,
                date,
                metadata: None,
            },
            metadata: DocumentMetadata {
                domain: None, 
                protocol: None,
            },
            blocks: Vec::new(),
            span: None,
        }
    }
    
    /// Add block to document
    pub fn add_block(&mut self, block: CanonicalAispBlock) {
        self.blocks.push(block);
    }
    
    /// Get all blocks of a specific type
    pub fn get_blocks_by_type<T>(&self, block_type: fn(&CanonicalAispBlock) -> Option<&T>) -> Vec<&T> {
        self.blocks.iter().filter_map(block_type).collect()
    }
    
    /// Get first block of a specific type
    pub fn get_first_block<T>(&self, block_type: fn(&CanonicalAispBlock) -> Option<&T>) -> Option<&T> {
        self.blocks.iter().find_map(block_type)
    }
}

/// Conversion trait for migrating from legacy AST types
pub trait IntoCanonical<T> {
    fn into_canonical(self) -> T;
}

/// Type alias for backward compatibility during migration
pub type AispDocument = CanonicalAispDocument;
pub type AispBlock = CanonicalAispBlock;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_canonical_document_creation() {
        let doc = CanonicalAispDocument::new(
            "test".to_string(),
            "5.1".to_string(),
            "2026-01-27".to_string(),
        );
        
        assert_eq!(doc.header.name, "test");
        assert_eq!(doc.header.version, "5.1");
        assert_eq!(doc.blocks.len(), 0);
    }
    
    #[test]
    fn test_block_type_identification() {
        let meta_block = CanonicalAispBlock::Meta(MetaBlock {
            entries: vec!["test".to_string()],
            span: None,
        });
        
        assert_eq!(meta_block.block_type(), "Meta");
        assert!(meta_block.as_meta().is_some());
        assert!(meta_block.as_types().is_none());
    }
    
    #[test]
    fn test_block_filtering() {
        let mut doc = CanonicalAispDocument::default();
        
        doc.add_block(CanonicalAispBlock::Meta(MetaBlock {
            entries: vec!["meta1".to_string()],
            span: None,
        }));
        
        doc.add_block(CanonicalAispBlock::Types(TypesBlock {
            definitions: HashMap::new(),
            span: None,
        }));
        
        let meta_blocks = doc.get_blocks_by_type(|b| b.as_meta());
        let type_blocks = doc.get_blocks_by_type(|b| b.as_types());
        
        assert_eq!(meta_blocks.len(), 1);
        assert_eq!(type_blocks.len(), 1);
    }
}