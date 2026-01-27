//! Canonical AST Block Definitions
//!
//! Focused module containing block structure definitions, operations,
//! and parsing logic following SRP architecture.

use super::types::*;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

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
    pub entries: HashMap<String, MetaEntry>,
    pub raw_entries: Vec<String>, // Keep raw strings for parsing
    pub span: Option<Span>,
}

/// Types block for type definitions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TypesBlock {
    pub definitions: HashMap<String, TypeDefinition>,
    pub raw_definitions: Vec<String>, // Keep raw strings for parsing later
    pub span: Option<Span>,
}

/// Rules block for logical rules and constraints
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RulesBlock {
    pub rules: Vec<LogicalRule>,
    pub raw_rules: Vec<String>, // Keep raw strings for parsing
    pub span: Option<Span>,
}

/// Functions block for function definitions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FunctionsBlock {
    pub functions: Vec<FunctionDefinition>,
    pub raw_functions: Vec<String>, // Keep raw strings for parsing
    pub span: Option<Span>,
}

/// Evidence block for validation evidence
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EvidenceBlock {
    pub delta: Option<f64>,
    pub phi: Option<u64>, 
    pub tau: Option<String>,
    pub metrics: HashMap<String, f64>,
    pub raw_evidence: Vec<String>, // Keep raw strings for parsing
    pub span: Option<Span>,
}

impl MetaBlock {
    /// Parse raw entries into structured MetaEntry objects
    pub fn parse_entries(&mut self) {
        for raw_entry in &self.raw_entries {
            if let Some((key, value)) = Self::parse_meta_entry(raw_entry) {
                self.entries.insert(key.clone(), MetaEntry {
                    key,
                    value,
                    span: None,
                });
            }
        }
    }
    
    fn parse_meta_entry(entry: &str) -> Option<(String, MetaValue)> {
        // Simple parsing - extend as needed
        if let Some(pos) = entry.find('≜') { // ≜ definition symbol
            let key = entry[..pos].trim().to_string();
            let value_str = entry[pos + '≜'.len_utf8()..].trim();
            
            let value = if let Ok(num) = value_str.parse::<f64>() {
                MetaValue::Number(num)
            } else if value_str == "true" || value_str == "false" {
                MetaValue::Boolean(value_str == "true")
            } else if value_str.contains('∀') || value_str.contains('∃') || value_str.contains('⇒') {
                // Simple heuristic for logical expressions
                MetaValue::Constraint(LogicalExpression::Raw(value_str.to_string()))
            } else {
                MetaValue::String(value_str.to_string())
            };
            
            Some((key, value))
        } else {
            None
        }
    }
}

impl TypesBlock {
    /// Parse raw definitions into structured TypeDefinition objects
    pub fn parse_definitions(&mut self) {
        for raw_def in &self.raw_definitions {
            if let Some((name, type_expr)) = Self::parse_type_definition(raw_def) {
                self.definitions.insert(name.clone(), TypeDefinition {
                    name,
                    type_expr,
                    span: None,
                });
            }
        }
    }
    
    fn parse_type_definition(def: &str) -> Option<(String, TypeExpression)> {
        // Simple parsing - extend as needed
        if let Some(pos) = def.find('≜') {
            let name = def[..pos].trim().to_string();
            let type_str = def[pos + '≜'.len_utf8()..].trim();
            
            let type_expr = if type_str.starts_with('{') && type_str.ends_with('}') {
                // Set type
                let inner = &type_str[1..type_str.len()-1];
                let elements: Vec<String> = inner.split(',').map(|s| s.trim().to_string()).collect();
                TypeExpression::Union(elements.into_iter().map(|e| 
                    TypeExpression::Basic(BasicType::Custom(e))
                ).collect())
            } else {
                // Basic type
                TypeExpression::Basic(BasicType::Custom(type_str.to_string()))
            };
            
            Some((name, type_expr))
        } else {
            None
        }
    }
}

impl RulesBlock {
    /// Parse raw rules into structured LogicalRule objects
    pub fn parse_rules(&mut self) {
        for raw_rule in &self.raw_rules {
            if let Some(rule) = Self::parse_logical_rule(raw_rule) {
                self.rules.push(rule);
            }
        }
    }
    
    fn parse_logical_rule(rule_str: &str) -> Option<LogicalRule> {
        // Simple parsing - for now just wrap as raw expression
        Some(LogicalRule {
            quantifier: None,
            expression: LogicalExpression::Raw(rule_str.to_string()),
            raw_text: rule_str.to_string(),
            span: None,
        })
    }
}

impl FunctionsBlock {
    /// Parse raw functions into structured FunctionDefinition objects
    pub fn parse_functions(&mut self) {
        for raw_func in &self.raw_functions {
            if let Some(func) = Self::parse_function_definition(raw_func) {
                self.functions.push(func);
            }
        }
    }
    
    fn parse_function_definition(func_str: &str) -> Option<FunctionDefinition> {
        // Simple parsing - for now just wrap as raw
        if let Some(pos) = func_str.find('≜') {
            let name = func_str[..pos].trim().to_string();
            let lambda_str = func_str[pos + '≜'.len_utf8()..].trim();
            
            Some(FunctionDefinition {
                name,
                lambda: LambdaExpression {
                    parameters: Vec::new(), // TODO: Parse parameters
                    body: LogicalExpression::Raw(lambda_str.to_string()),
                    span: None,
                },
                raw_text: func_str.to_string(),
                span: None,
            })
        } else {
            None
        }
    }
}

impl EvidenceBlock {
    /// Parse raw evidence into structured metrics
    pub fn parse_evidence(&mut self) {
        let raw_evidence_clone = self.raw_evidence.clone();
        for raw_ev in &raw_evidence_clone {
            self.parse_evidence_entry(raw_ev);
        }
    }
    
    fn parse_evidence_entry(&mut self, evidence_str: &str) {
        // Parse δ, φ, τ values
        if evidence_str.contains('δ') {
            if let Some(delta_val) = Self::extract_numeric_value(evidence_str, 'δ') {
                self.delta = Some(delta_val);
            }
        }
        if evidence_str.contains('φ') {
            if let Some(phi_val) = Self::extract_numeric_value(evidence_str, 'φ') {
                self.phi = Some(phi_val as u64);
            }
        }
        if evidence_str.contains('τ') {
            if let Some(tau_val) = Self::extract_string_value(evidence_str, 'τ') {
                self.tau = Some(tau_val);
            }
        }
    }
    
    fn extract_numeric_value(text: &str, symbol: char) -> Option<f64> {
        if let Some(pos) = text.find(symbol) {
            let after_symbol = &text[pos + symbol.len_utf8()..];
            if let Some(eq_pos) = after_symbol.find('≜') {
                let value_str = after_symbol[eq_pos + '≜'.len_utf8()..]
                    .split_whitespace().next()?;
                value_str.parse().ok()
            } else {
                None
            }
        } else {
            None
        }
    }
    
    fn extract_string_value(text: &str, symbol: char) -> Option<String> {
        if let Some(pos) = text.find(symbol) {
            let after_symbol = &text[pos + symbol.len_utf8()..];
            if let Some(eq_pos) = after_symbol.find('≜') {
                let value_str = after_symbol[eq_pos + '≜'.len_utf8()..]
                    .split_whitespace().next()?;
                Some(value_str.to_string())
            } else {
                None
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_type_identification() {
        let meta_block = CanonicalAispBlock::Meta(MetaBlock {
            entries: HashMap::new(),
            raw_entries: vec!["test".to_string()],
            span: None,
        });
        
        assert_eq!(meta_block.block_type(), "Meta");
        assert!(meta_block.as_meta().is_some());
        assert!(meta_block.as_types().is_none());
    }

    #[test]
    fn test_meta_entry_parsing() {
        let mut meta_block = MetaBlock {
            entries: HashMap::new(),
            raw_entries: vec!["Vision≜\"test\"".to_string(), "count≜42".to_string()],
            span: None,
        };
        
        meta_block.parse_entries();
        assert_eq!(meta_block.entries.len(), 2);
        assert!(meta_block.entries.contains_key("Vision"));
        assert!(meta_block.entries.contains_key("count"));
    }

    #[test]
    fn test_evidence_parsing() {
        let mut evidence_block = EvidenceBlock {
            delta: None,
            phi: None,
            tau: None,
            metrics: HashMap::new(),
            raw_evidence: vec!["δ≜0.001".to_string(), "φ≜42".to_string()],
            span: None,
        };
        
        evidence_block.parse_evidence();
        assert_eq!(evidence_block.delta, Some(0.001));
        assert_eq!(evidence_block.phi, Some(42));
    }

    #[test]
    fn test_block_accessor_methods() {
        let types_block = CanonicalAispBlock::Types(TypesBlock {
            definitions: HashMap::new(),
            raw_definitions: Vec::new(),
            span: None,
        });
        
        assert!(types_block.as_types().is_some());
        assert!(types_block.as_meta().is_none());
        assert!(types_block.as_rules().is_none());
        assert!(types_block.as_functions().is_none());
        assert!(types_block.as_evidence().is_none());
    }

    #[test]
    fn test_function_definition_parsing() {
        let mut functions_block = FunctionsBlock {
            functions: Vec::new(),
            raw_functions: vec!["f≜λx.x + 1".to_string()],
            span: None,
        };
        
        functions_block.parse_functions();
        assert_eq!(functions_block.functions.len(), 1);
        assert_eq!(functions_block.functions[0].name, "f");
        assert_eq!(functions_block.functions[0].raw_text, "f≜λx.x + 1");
    }
}