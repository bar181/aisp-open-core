//! Parser for AISP Evidence blocks
//!
//! This module handles parsing Evidence blocks (⟦Ε⟧) which contain
//! quality metrics and tier information.

use crate::ast::{EvidenceBlock, Span};
use crate::error::*;
use crate::lexer::AispLexer;
use crate::token_parser::TokenParser;
use std::collections::HashMap;

/// Parser for AISP Evidence blocks
pub struct EvidenceParser;

impl EvidenceParser {
    /// Parse evidence block
    pub fn parse_evidence_block(lexer: &mut AispLexer) -> AispResult<EvidenceBlock> {
        let (start_line, _) = lexer.position_info();
        let mut delta = None;
        let mut phi = None;
        let mut tau = None;
        let mut metrics = HashMap::new();
        
        lexer.skip_whitespace_and_comments();
        
        if lexer.match_char('⟨') {
            // Parse evidence tuple ⟨δ≜0.65;φ≜100;τ≜◊⁺⟩
            while !lexer.check('⟩') && !lexer.is_at_end() {
                let key = TokenParser::parse_identifier(lexer)?;
                
                if !lexer.match_char('≜') {
                    return Err(lexer.parse_error("Expected '≜' in evidence entry"));
                }
                
                match key.as_str() {
                    "δ" => delta = Some(TokenParser::parse_number(lexer)?),
                    "φ" => phi = Some(TokenParser::parse_number(lexer)?),
                    "τ" => tau = Some(TokenParser::parse_tier_symbol(lexer)?),
                    _ => {
                        metrics.insert(key, TokenParser::parse_number(lexer)?);
                    }
                }
                
                if lexer.match_char(';') {
                    continue;
                } else if lexer.check('⟩') {
                    break;
                } else {
                    return Err(lexer.parse_error("Expected ';' or '⟩' in evidence block"));
                }
            }
            
            if !lexer.match_char('⟩') {
                return Err(lexer.parse_error("Expected '⟩' to close evidence block"));
            }
        }
        
        let (end_line, end_column) = lexer.position_info();
        Ok(EvidenceBlock {
            delta,
            phi,
            tau,
            metrics,
            span: Span::new(start_line, 1, end_line, end_column),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_evidence_block_with_delta() {
        let mut lexer = AispLexer::new("⟨δ≜0.8⟩".to_string());
        let evidence = EvidenceParser::parse_evidence_block(&mut lexer).unwrap();
        
        assert_eq!(evidence.delta, Some(0.8));
        assert_eq!(evidence.phi, None);
        assert_eq!(evidence.tau, None);
        assert!(evidence.metrics.is_empty());
    }

    #[test]
    fn test_parse_evidence_block_full() {
        let mut lexer = AispLexer::new("⟨δ≜0.85;φ≜100;τ≜◊⁺⟩".to_string());
        let evidence = EvidenceParser::parse_evidence_block(&mut lexer).unwrap();
        
        assert_eq!(evidence.delta, Some(0.85));
        assert_eq!(evidence.phi, Some(100.0));
        assert_eq!(evidence.tau, Some("◊⁺".to_string()));
        assert!(evidence.metrics.is_empty());
    }

    #[test]
    fn test_parse_evidence_block_with_custom_metrics() {
        let mut lexer = AispLexer::new("⟨δ≜0.9;custom≜42.5⟩".to_string());
        let evidence = EvidenceParser::parse_evidence_block(&mut lexer).unwrap();
        
        assert_eq!(evidence.delta, Some(0.9));
        assert_eq!(evidence.metrics.get("custom"), Some(&42.5));
    }

    #[test]
    fn test_parse_evidence_block_platinum_tier() {
        let mut lexer = AispLexer::new("⟨δ≜1.0;τ≜◊⁺⁺⟩".to_string());
        let evidence = EvidenceParser::parse_evidence_block(&mut lexer).unwrap();
        
        assert_eq!(evidence.delta, Some(1.0));
        assert_eq!(evidence.tau, Some("◊⁺⁺".to_string()));
    }

    #[test]
    fn test_parse_evidence_block_reject_tier() {
        let mut lexer = AispLexer::new("⟨δ≜0.1;τ≜⊘⟩".to_string());
        let evidence = EvidenceParser::parse_evidence_block(&mut lexer).unwrap();
        
        assert_eq!(evidence.delta, Some(0.1));
        assert_eq!(evidence.tau, Some("⊘".to_string()));
    }

    #[test]
    fn test_parse_empty_evidence_block() {
        let mut lexer = AispLexer::new("".to_string());
        let evidence = EvidenceParser::parse_evidence_block(&mut lexer).unwrap();
        
        assert_eq!(evidence.delta, None);
        assert_eq!(evidence.phi, None);
        assert_eq!(evidence.tau, None);
        assert!(evidence.metrics.is_empty());
    }

    #[test]
    fn test_parse_evidence_block_malformed() {
        let mut lexer = AispLexer::new("⟨δ≜0.8".to_string()); // missing closing ⟩
        let result = EvidenceParser::parse_evidence_block(&mut lexer);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_evidence_block_missing_equals() {
        let mut lexer = AispLexer::new("⟨δ0.8⟩".to_string()); // missing ≜
        let result = EvidenceParser::parse_evidence_block(&mut lexer);
        assert!(result.is_err());
    }
}