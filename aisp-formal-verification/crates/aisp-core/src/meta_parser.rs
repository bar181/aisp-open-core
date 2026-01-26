//! Parser for AISP Meta blocks
//!
//! This module handles parsing Meta blocks (⟦Ω:Meta⟧) which can contain
//! both key-value pairs and logical constraints.

use crate::ast::{MetaBlock, MetaEntry, MetaValue, Span};
use crate::error::*;
use crate::lexer::AispLexer;
use crate::logic_parser::LogicParser;
use crate::token_parser::TokenParser;
use std::collections::HashMap;

/// Parser for AISP Meta blocks
pub struct MetaParser;

impl MetaParser {
    /// Parse meta block content
    pub fn parse_meta_block(lexer: &mut AispLexer) -> AispResult<MetaBlock> {
        let (start_line, _) = lexer.position_info();
        let mut entries = HashMap::new();
        
        while !lexer.check('}') && !lexer.is_at_end() {
            lexer.skip_whitespace_and_comments();
            if lexer.check('}') || lexer.is_at_end() {
                break;
            }
            
            // Check if this line starts with a quantifier (logical constraint)
            if lexer.peek() == Some('∀') || lexer.peek() == Some('∃') {
                // Parse as logical constraint and store with special key
                let constraint = LogicParser::parse_logical_expression(lexer)?;
                let (line, column) = lexer.position_info();
                let span = Span::new(line, 1, line, column);
                
                // Use a special key pattern for constraints
                let constraint_key = format!("_constraint_{}", entries.len());
                entries.insert(
                    constraint_key.clone(),
                    MetaEntry { 
                        key: constraint_key, 
                        value: MetaValue::Constraint(constraint), 
                        span 
                    },
                );
            } else {
                // Check if we have a valid identifier start character
                if let Some(ch) = lexer.peek() {
                    if !ch.is_alphabetic() && ch != '_' {
                        // Skip invalid characters or whitespace
                        lexer.advance();
                        continue;
                    }
                } else {
                    break;
                }
                
                // Parse as regular key=value pair
                let key = TokenParser::parse_identifier(lexer)?;
                
                if !lexer.match_char('≜') {
                    return Err(lexer.parse_error("Expected '≜' in meta entry"));
                }
                
                let value = Self::parse_meta_value(lexer)?;
                let (line, column) = lexer.position_info();
                let span = Span::new(line, 1, line, column);
                
                entries.insert(
                    key.clone(),
                    MetaEntry { key, value, span },
                );
            }
            
            // Skip to next line or entry
            lexer.skip_whitespace_and_comments();
        }
        
        let (end_line, end_column) = lexer.position_info();
        Ok(MetaBlock {
            entries,
            span: Span::new(start_line, 1, end_line, end_column),
        })
    }

    /// Parse meta value (string, number, boolean, or constraint)
    fn parse_meta_value(lexer: &mut AispLexer) -> AispResult<MetaValue> {
        lexer.skip_whitespace_and_comments();
        
        // Try to parse as logical constraint first (contains AISP symbols)
        if TokenParser::is_logical_expression(lexer) {
            return Ok(MetaValue::Constraint(LogicParser::parse_logical_expression(lexer)?));
        }
        
        if let Some(ch) = lexer.peek() {
            match ch {
                '"' => {
                    let value = TokenParser::parse_quoted_string(lexer)?;
                    Ok(MetaValue::String(value))
                }
                ch if ch.is_ascii_digit() || ch == '.' || ch == '-' => {
                    Ok(MetaValue::Number(TokenParser::parse_number(lexer)?))
                }
                't' | 'f' => {
                    // Try to parse boolean
                    let remaining = lexer.remaining_source();
                    if remaining.starts_with("true") {
                        // Advance 4 characters
                        for _ in 0..4 { lexer.advance(); }
                        Ok(MetaValue::Boolean(true))
                    } else if remaining.starts_with("false") {
                        // Advance 5 characters
                        for _ in 0..5 { lexer.advance(); }
                        Ok(MetaValue::Boolean(false))
                    } else {
                        Ok(MetaValue::String(TokenParser::parse_identifier(lexer)?))
                    }
                }
                _ => Ok(MetaValue::String(TokenParser::parse_identifier(lexer)?)),
            }
        } else {
            Err(lexer.parse_error("Unexpected end of input in meta value"))
        }
    }
}

// Import logic parser module - we'll create this next
mod logic_parser {
    use crate::ast::LogicalExpression;
    use crate::error::*;
    use crate::lexer::AispLexer;

    pub struct LogicParser;

    impl LogicParser {
        /// Parse logical expression (simplified - consumes rest of rule)
        pub fn parse_logical_expression(lexer: &mut AispLexer) -> AispResult<LogicalExpression> {
            lexer.skip_whitespace_and_comments();
            
            // Simplified: just consume the rest of the rule as a single expression
            let mut expression = String::new();
            
            while !lexer.is_at_end() && !lexer.check('\n') && !lexer.check('}') {
                if let Some(ch) = lexer.advance() {
                    expression.push(ch);
                }
            }
            
            let expression = expression.trim().to_string();
            
            if expression.is_empty() {
                Err(lexer.parse_error("Expected logical expression"))
            } else {
                // For now, represent as a variable with the full expression text
                Ok(LogicalExpression::Variable(expression))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_meta_block_with_entries() {
        let mut lexer = AispLexer::new("domain≜test\nversion≜\"1.0.0\"\n".to_string());
        let meta = MetaParser::parse_meta_block(&mut lexer).unwrap();
        
        assert_eq!(meta.entries.len(), 2);
        assert!(meta.entries.contains_key("domain"));
        assert!(meta.entries.contains_key("version"));
    }

    #[test]
    fn test_parse_meta_value_string() {
        let mut lexer = AispLexer::new("\"hello world\"".to_string());
        let value = MetaParser::parse_meta_value(&mut lexer).unwrap();
        
        match value {
            MetaValue::String(s) => assert_eq!(s, "hello world"),
            _ => panic!("Expected string value"),
        }
    }

    #[test]
    fn test_parse_meta_value_number() {
        let mut lexer = AispLexer::new("123.45".to_string());
        let value = MetaParser::parse_meta_value(&mut lexer).unwrap();
        
        match value {
            MetaValue::Number(n) => assert_eq!(n, 123.45),
            _ => panic!("Expected number value"),
        }
    }

    #[test]
    fn test_parse_meta_value_boolean() {
        let mut lexer = AispLexer::new("true".to_string());
        let value = MetaParser::parse_meta_value(&mut lexer).unwrap();
        
        match value {
            MetaValue::Boolean(b) => assert!(b),
            _ => panic!("Expected boolean value"),
        }
    }

    #[test]
    fn test_parse_meta_block_with_constraint() {
        let mut lexer = AispLexer::new("domain≜test\n∀D∈AISP:Ambig(D)<0.02\n".to_string());
        let meta = MetaParser::parse_meta_block(&mut lexer).unwrap();
        
        assert_eq!(meta.entries.len(), 2);
        assert!(meta.entries.contains_key("domain"));
        // Should contain a constraint entry with generated key
        let has_constraint = meta.entries.keys().any(|k| k.starts_with("_constraint_"));
        assert!(has_constraint);
    }

    #[test]
    fn test_parse_empty_meta_block() {
        let mut lexer = AispLexer::new("".to_string());
        let meta = MetaParser::parse_meta_block(&mut lexer).unwrap();
        
        assert!(meta.entries.is_empty());
    }
}