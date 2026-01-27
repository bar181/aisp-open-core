//! Canonical Document Operations
//!
//! Document-level operations, construction, and manipulation methods
//! following SRP architecture for canonical AST representation.

use super::types::*;
use super::blocks::*;
use serde::{Serialize, Deserialize};

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
    
    /// Parse structured data from raw strings (called after construction)
    pub fn parse_structured_data(&mut self) {
        for block in &mut self.blocks {
            match block {
                CanonicalAispBlock::Meta(meta) => {
                    meta.parse_entries();
                }
                CanonicalAispBlock::Types(types) => {
                    types.parse_definitions();
                }
                CanonicalAispBlock::Rules(rules) => {
                    rules.parse_rules();
                }
                CanonicalAispBlock::Functions(functions) => {
                    functions.parse_functions();
                }
                CanonicalAispBlock::Evidence(evidence) => {
                    evidence.parse_evidence();
                }
            }
        }
    }

    /// Get all meta blocks
    pub fn get_meta_blocks(&self) -> Vec<&MetaBlock> {
        self.get_blocks_by_type(|b| b.as_meta())
    }

    /// Get all types blocks
    pub fn get_types_blocks(&self) -> Vec<&TypesBlock> {
        self.get_blocks_by_type(|b| b.as_types())
    }

    /// Get all rules blocks
    pub fn get_rules_blocks(&self) -> Vec<&RulesBlock> {
        self.get_blocks_by_type(|b| b.as_rules())
    }

    /// Get all functions blocks
    pub fn get_functions_blocks(&self) -> Vec<&FunctionsBlock> {
        self.get_blocks_by_type(|b| b.as_functions())
    }

    /// Get all evidence blocks
    pub fn get_evidence_blocks(&self) -> Vec<&EvidenceBlock> {
        self.get_blocks_by_type(|b| b.as_evidence())
    }

    /// Get document statistics
    pub fn get_statistics(&self) -> DocumentStatistics {
        let mut stats = DocumentStatistics::default();
        
        for block in &self.blocks {
            match block {
                CanonicalAispBlock::Meta(meta) => {
                    stats.meta_blocks += 1;
                    stats.meta_entries += meta.entries.len();
                }
                CanonicalAispBlock::Types(types) => {
                    stats.types_blocks += 1;
                    stats.type_definitions += types.definitions.len();
                }
                CanonicalAispBlock::Rules(rules) => {
                    stats.rules_blocks += 1;
                    stats.logical_rules += rules.rules.len();
                }
                CanonicalAispBlock::Functions(functions) => {
                    stats.functions_blocks += 1;
                    stats.function_definitions += functions.functions.len();
                }
                CanonicalAispBlock::Evidence(evidence) => {
                    stats.evidence_blocks += 1;
                    stats.evidence_metrics += evidence.metrics.len();
                }
            }
        }
        
        stats
    }

    /// Validate document structure
    pub fn validate_structure(&self) -> DocumentValidation {
        let mut validation = DocumentValidation::new();
        
        // Check header completeness
        if self.header.version.is_empty() {
            validation.add_error("Header version is empty");
        }
        if self.header.name.is_empty() {
            validation.add_error("Header name is empty");
        }
        if self.header.date.is_empty() {
            validation.add_error("Header date is empty");
        }

        // Check for duplicate block types (warnings)
        let meta_count = self.get_meta_blocks().len();
        let types_count = self.get_types_blocks().len();
        let rules_count = self.get_rules_blocks().len();
        let functions_count = self.get_functions_blocks().len();
        let evidence_count = self.get_evidence_blocks().len();

        if meta_count > 1 {
            validation.add_warning(&format!("Multiple meta blocks found: {}", meta_count));
        }
        if types_count > 1 {
            validation.add_warning(&format!("Multiple types blocks found: {}", types_count));
        }
        if rules_count > 1 {
            validation.add_warning(&format!("Multiple rules blocks found: {}", rules_count));
        }
        if functions_count > 1 {
            validation.add_warning(&format!("Multiple functions blocks found: {}", functions_count));
        }
        if evidence_count > 1 {
            validation.add_warning(&format!("Multiple evidence blocks found: {}", evidence_count));
        }

        // Check for empty document
        if self.blocks.is_empty() {
            validation.add_warning("Document has no blocks");
        }

        validation
    }

    /// Set document metadata domain
    pub fn set_domain(&mut self, domain: String) {
        self.metadata.domain = Some(domain);
    }

    /// Set document metadata protocol  
    pub fn set_protocol(&mut self, protocol: String) {
        self.metadata.protocol = Some(protocol);
    }

    /// Add metadata to header
    pub fn set_header_metadata(&mut self, metadata: HeaderMetadata) {
        self.header.metadata = Some(metadata);
    }
}

/// Document statistics for analysis
#[derive(Debug, Clone, Default)]
pub struct DocumentStatistics {
    pub meta_blocks: usize,
    pub meta_entries: usize,
    pub types_blocks: usize,
    pub type_definitions: usize,
    pub rules_blocks: usize,
    pub logical_rules: usize,
    pub functions_blocks: usize,
    pub function_definitions: usize,
    pub evidence_blocks: usize,
    pub evidence_metrics: usize,
}

/// Document validation result
#[derive(Debug, Clone)]
pub struct DocumentValidation {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

impl DocumentValidation {
    pub fn new() -> Self {
        Self {
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }

    pub fn add_error(&mut self, error: &str) {
        self.errors.push(error.to_string());
    }

    pub fn add_warning(&mut self, warning: &str) {
        self.warnings.push(warning.to_string());
    }

    pub fn is_valid(&self) -> bool {
        self.errors.is_empty()
    }

    pub fn has_warnings(&self) -> bool {
        !self.warnings.is_empty()
    }
}

impl Default for DocumentValidation {
    fn default() -> Self {
        Self::new()
    }
}

/// Type alias for backward compatibility during migration
pub type AispDocument = CanonicalAispDocument;

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    
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
    fn test_block_filtering() {
        let mut doc = CanonicalAispDocument::default();
        
        doc.add_block(CanonicalAispBlock::Meta(MetaBlock {
            entries: HashMap::new(),
            raw_entries: vec!["meta1".to_string()],
            span: None,
        }));
        
        doc.add_block(CanonicalAispBlock::Types(TypesBlock {
            definitions: HashMap::new(),
            raw_definitions: Vec::new(),
            span: None,
        }));
        
        let meta_blocks = doc.get_blocks_by_type(|b| b.as_meta());
        let type_blocks = doc.get_blocks_by_type(|b| b.as_types());
        
        assert_eq!(meta_blocks.len(), 1);
        assert_eq!(type_blocks.len(), 1);
    }

    #[test]
    fn test_document_statistics() {
        let mut doc = CanonicalAispDocument::default();
        
        // Add meta block with entries
        let mut meta_entries = HashMap::new();
        meta_entries.insert("key1".to_string(), MetaEntry {
            key: "key1".to_string(),
            value: MetaValue::String("value1".to_string()),
            span: None,
        });
        
        doc.add_block(CanonicalAispBlock::Meta(MetaBlock {
            entries: meta_entries,
            raw_entries: Vec::new(),
            span: None,
        }));
        
        let stats = doc.get_statistics();
        assert_eq!(stats.meta_blocks, 1);
        assert_eq!(stats.meta_entries, 1);
        assert_eq!(stats.types_blocks, 0);
    }

    #[test]
    fn test_document_validation() {
        let mut doc = CanonicalAispDocument::default();
        doc.header.name = "".to_string(); // Invalid empty name
        
        let validation = doc.validate_structure();
        assert!(!validation.is_valid());
        assert!(validation.errors.iter().any(|e| e.contains("name is empty")));
    }

    #[test]
    fn test_document_metadata_operations() {
        let mut doc = CanonicalAispDocument::default();
        
        doc.set_domain("mathematics".to_string());
        doc.set_protocol("aisp".to_string());
        
        assert_eq!(doc.metadata.domain, Some("mathematics".to_string()));
        assert_eq!(doc.metadata.protocol, Some("aisp".to_string()));
    }

    #[test]
    fn test_structured_data_parsing() {
        let mut doc = CanonicalAispDocument::default();
        
        doc.add_block(CanonicalAispBlock::Meta(MetaBlock {
            entries: HashMap::new(),
            raw_entries: vec!["Vision≜\"test\"".to_string()],
            span: None,
        }));
        
        doc.parse_structured_data();
        
        let meta_blocks = doc.get_meta_blocks();
        assert_eq!(meta_blocks.len(), 1);
        assert_eq!(meta_blocks[0].entries.len(), 1);
        assert!(meta_blocks[0].entries.contains_key("Vision"));
    }

    #[test]
    fn test_convenience_accessors() {
        let mut doc = CanonicalAispDocument::default();
        
        doc.add_block(CanonicalAispBlock::Rules(RulesBlock {
            rules: Vec::new(),
            raw_rules: Vec::new(),
            span: None,
        }));
        
        doc.add_block(CanonicalAispBlock::Functions(FunctionsBlock {
            functions: Vec::new(),
            raw_functions: Vec::new(),
            span: None,
        }));
        
        assert_eq!(doc.get_rules_blocks().len(), 1);
        assert_eq!(doc.get_functions_blocks().len(), 1);
        assert_eq!(doc.get_evidence_blocks().len(), 0);
    }
}