//! Canonical Conversion Operations
//!
//! Conversion traits and implementations for migrating from legacy AST types
//! to canonical representation following SRP architecture.

use super::types::*;
use super::blocks::*;
use super::document::*;
use crate::parser::robust_parser::SecuritySeverity;
use std::collections::HashMap;

/// Conversion trait for migrating from legacy AST types
pub trait IntoCanonical<T> {
    fn into_canonical(self) -> T;
}

/// Convert from robust_parser types to canonical types
impl IntoCanonical<CanonicalAispDocument> for crate::parser::robust_parser::AispDocument {
    fn into_canonical(self) -> CanonicalAispDocument {
        CanonicalAispDocument {
            header: DocumentHeader {
                version: self.header.version,
                name: self.header.name,
                date: self.header.date,
                metadata: self.header.metadata.map(|_| HeaderMetadata {
                    author: None, // Map from robust_parser metadata if available
                    description: None,
                    tags: Vec::new(),
                }),
            },
            metadata: DocumentMetadata {
                domain: self.metadata.domain,
                protocol: self.metadata.protocol,
            },
            blocks: self.blocks.into_iter().map(|b| b.into_canonical()).collect(),
            span: None, // Could map from robust_parser span if available
        }
    }
}

/// Convert from robust_parser block types to canonical block types  
impl IntoCanonical<CanonicalAispBlock> for crate::parser::robust_parser::AispBlock {
    fn into_canonical(self) -> CanonicalAispBlock {
        match self {
            crate::parser::robust_parser::AispBlock::Meta(meta) => {
                CanonicalAispBlock::Meta(MetaBlock {
                    entries: HashMap::new(), // Parse structured entries from raw strings
                    raw_entries: meta.entries,
                    span: None,
                })
            }
            crate::parser::robust_parser::AispBlock::Types(types) => {
                CanonicalAispBlock::Types(TypesBlock {
                    definitions: HashMap::new(), // Parse structured types from raw strings
                    raw_definitions: types.definitions,
                    span: None,
                })
            }
            crate::parser::robust_parser::AispBlock::Rules(rules) => {
                CanonicalAispBlock::Rules(RulesBlock {
                    rules: Vec::new(), // Parse structured rules from raw strings
                    raw_rules: rules.rules,
                    span: None,
                })
            }
            crate::parser::robust_parser::AispBlock::Functions(functions) => {
                CanonicalAispBlock::Functions(FunctionsBlock {
                    functions: Vec::new(), // Parse structured functions from raw strings
                    raw_functions: functions.functions,
                    span: None,
                })
            }
            crate::parser::robust_parser::AispBlock::Evidence(evidence) => {
                CanonicalAispBlock::Evidence(EvidenceBlock {
                    delta: None, // Parse from evidence strings
                    phi: None,
                    tau: None,
                    metrics: HashMap::new(),
                    raw_evidence: evidence.evidence,
                    span: None,
                })
            }
            // Handle additional block types
            crate::parser::robust_parser::AispBlock::Errors(errors) => {
                // Map errors to meta block for now
                CanonicalAispBlock::Meta(MetaBlock {
                    entries: HashMap::new(),
                    raw_entries: errors.errors,
                    span: None,
                })
            }
        }
    }
}

/// Convert canonical types to legacy types for backward compatibility
pub trait FromCanonical<T> {
    fn from_canonical(canonical: T) -> Self;
}

/// Convert security severity between different representations
pub fn map_security_severity_to_canonical(severity: SecuritySeverity) -> MetaValue {
    match severity {
        SecuritySeverity::Low => MetaValue::String("low".to_string()),
        SecuritySeverity::Medium => MetaValue::String("medium".to_string()),
        SecuritySeverity::High => MetaValue::String("high".to_string()),
        SecuritySeverity::Critical => MetaValue::String("critical".to_string()),
    }
}

/// Convert canonical severity back to security severity
pub fn map_canonical_to_security_severity(meta_value: &MetaValue) -> SecuritySeverity {
    match meta_value {
        MetaValue::String(s) => match s.as_str() {
            "low" => SecuritySeverity::Low,
            "medium" => SecuritySeverity::Medium,
            "high" => SecuritySeverity::High,
            "critical" => SecuritySeverity::Critical,
            _ => SecuritySeverity::Medium, // Default fallback
        },
        _ => SecuritySeverity::Medium, // Default for non-string values
    }
}

/// Utility function to extract specific metadata from canonical document
pub fn extract_domain_metadata(doc: &CanonicalAispDocument) -> Option<String> {
    doc.metadata.domain.clone()
}

/// Utility function to extract protocol metadata from canonical document
pub fn extract_protocol_metadata(doc: &CanonicalAispDocument) -> Option<String> {
    doc.metadata.protocol.clone()
}

/// Utility function to extract meta entries from all meta blocks
pub fn extract_all_meta_entries(doc: &CanonicalAispDocument) -> HashMap<String, MetaValue> {
    let mut all_entries = HashMap::new();
    
    for meta_block in doc.get_meta_blocks() {
        for (key, entry) in &meta_block.entries {
            all_entries.insert(key.clone(), entry.value.clone());
        }
    }
    
    all_entries
}

/// Utility function to extract all type definitions
pub fn extract_all_type_definitions(doc: &CanonicalAispDocument) -> HashMap<String, TypeExpression> {
    let mut all_definitions = HashMap::new();
    
    for types_block in doc.get_types_blocks() {
        for (name, type_def) in &types_block.definitions {
            all_definitions.insert(name.clone(), type_def.type_expr.clone());
        }
    }
    
    all_definitions
}

/// Utility function to extract all logical rules
pub fn extract_all_logical_rules(doc: &CanonicalAispDocument) -> Vec<LogicalRule> {
    let mut all_rules = Vec::new();
    
    for rules_block in doc.get_rules_blocks() {
        all_rules.extend(rules_block.rules.clone());
    }
    
    all_rules
}

/// Utility function to extract all function definitions
pub fn extract_all_function_definitions(doc: &CanonicalAispDocument) -> Vec<FunctionDefinition> {
    let mut all_functions = Vec::new();
    
    for functions_block in doc.get_functions_blocks() {
        all_functions.extend(functions_block.functions.clone());
    }
    
    all_functions
}

/// Utility function to extract all evidence metrics
pub fn extract_all_evidence_metrics(doc: &CanonicalAispDocument) -> HashMap<String, f64> {
    let mut all_metrics = HashMap::new();
    
    for evidence_block in doc.get_evidence_blocks() {
        all_metrics.extend(evidence_block.metrics.clone());
    }
    
    all_metrics
}

/// Conversion builder for complex migrations
pub struct ConversionBuilder {
    preserve_raw_data: bool,
    validate_on_conversion: bool,
    auto_parse_structured: bool,
}

impl ConversionBuilder {
    pub fn new() -> Self {
        Self {
            preserve_raw_data: true,
            validate_on_conversion: false,
            auto_parse_structured: true,
        }
    }

    pub fn preserve_raw_data(mut self, preserve: bool) -> Self {
        self.preserve_raw_data = preserve;
        self
    }

    pub fn validate_on_conversion(mut self, validate: bool) -> Self {
        self.validate_on_conversion = validate;
        self
    }

    pub fn auto_parse_structured(mut self, auto_parse: bool) -> Self {
        self.auto_parse_structured = auto_parse;
        self
    }

    pub fn convert(
        &self,
        source: crate::parser::robust_parser::AispDocument,
    ) -> Result<CanonicalAispDocument, ConversionError> {
        let mut canonical = source.into_canonical();

        if self.auto_parse_structured {
            canonical.parse_structured_data();
        }

        if self.validate_on_conversion {
            let validation = canonical.validate_structure();
            if !validation.is_valid() {
                return Err(ConversionError::ValidationFailed(validation.errors));
            }
        }

        Ok(canonical)
    }
}

impl Default for ConversionBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Conversion errors
#[derive(Debug, Clone)]
pub enum ConversionError {
    ValidationFailed(Vec<String>),
    ParseError(String),
    UnsupportedType(String),
}

impl std::fmt::Display for ConversionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConversionError::ValidationFailed(errors) => {
                write!(f, "Validation failed: {}", errors.join(", "))
            }
            ConversionError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            ConversionError::UnsupportedType(typ) => write!(f, "Unsupported type: {}", typ),
        }
    }
}

impl std::error::Error for ConversionError {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::robust_parser::{self, DocumentHeader as RobustHeader, DocumentMetadata as RobustMetadata};

    #[test]
    fn test_security_severity_conversion() {
        let canonical = map_security_severity_to_canonical(SecuritySeverity::High);
        assert_eq!(canonical, MetaValue::String("high".to_string()));

        let back = map_canonical_to_security_severity(&canonical);
        assert_eq!(back, SecuritySeverity::High);
    }

    #[test]
    fn test_document_conversion() {
        let robust_doc = robust_parser::AispDocument {
            header: RobustHeader {
                version: "5.1".to_string(),
                name: "test".to_string(),
                date: "2026-01-27".to_string(),
                metadata: None,
            },
            metadata: RobustMetadata {
                domain: Some("math".to_string()),
                protocol: Some("aisp".to_string()),
            },
            blocks: Vec::new(),
        };

        let canonical = robust_doc.into_canonical();
        assert_eq!(canonical.header.version, "5.1");
        assert_eq!(canonical.header.name, "test");
        assert_eq!(canonical.metadata.domain, Some("math".to_string()));
        assert_eq!(canonical.metadata.protocol, Some("aisp".to_string()));
    }

    #[test]
    fn test_conversion_builder() {
        let robust_doc = robust_parser::AispDocument {
            header: RobustHeader {
                version: "5.1".to_string(),
                name: "test".to_string(),
                date: "2026-01-27".to_string(),
                metadata: None,
            },
            metadata: RobustMetadata {
                domain: Some("math".to_string()),
                protocol: Some("aisp".to_string()),
            },
            blocks: Vec::new(),
        };

        let builder = ConversionBuilder::new()
            .preserve_raw_data(true)
            .validate_on_conversion(true)
            .auto_parse_structured(false);

        let result = builder.convert(robust_doc);
        assert!(result.is_ok());
    }

    #[test]
    fn test_meta_extraction_utilities() {
        let mut doc = CanonicalAispDocument::default();
        
        let mut entries = HashMap::new();
        entries.insert("test_key".to_string(), MetaEntry {
            key: "test_key".to_string(),
            value: MetaValue::String("test_value".to_string()),
            span: None,
        });

        doc.add_block(CanonicalAispBlock::Meta(MetaBlock {
            entries,
            raw_entries: Vec::new(),
            span: None,
        }));

        let extracted = extract_all_meta_entries(&doc);
        assert_eq!(extracted.len(), 1);
        assert_eq!(extracted.get("test_key"), Some(&MetaValue::String("test_value".to_string())));
    }

    #[test]
    fn test_domain_protocol_extraction() {
        let mut doc = CanonicalAispDocument::default();
        doc.set_domain("mathematics".to_string());
        doc.set_protocol("aisp".to_string());

        assert_eq!(extract_domain_metadata(&doc), Some("mathematics".to_string()));
        assert_eq!(extract_protocol_metadata(&doc), Some("aisp".to_string()));
    }

    #[test]
    fn test_conversion_error_display() {
        let error = ConversionError::ValidationFailed(vec!["Error 1".to_string(), "Error 2".to_string()]);
        let display = format!("{}", error);
        assert!(display.contains("Validation failed"));
        assert!(display.contains("Error 1"));
        assert!(display.contains("Error 2"));
    }

    #[test]
    fn test_block_type_conversion() {
        let meta_block = robust_parser::AispBlock::Meta(robust_parser::MetaBlock {
            entries: vec!["Vision≜\"test\"".to_string()],
        });

        let canonical_block = meta_block.into_canonical();
        match canonical_block {
            CanonicalAispBlock::Meta(meta) => {
                assert_eq!(meta.raw_entries, vec!["Vision≜\"test\"".to_string()]);
                assert!(meta.entries.is_empty()); // Not parsed yet
            }
            _ => panic!("Expected Meta block"),
        }
    }
}