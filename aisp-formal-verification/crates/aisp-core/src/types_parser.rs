//! Parser for AISP Types blocks
//!
//! This module handles parsing Types blocks (⟦Σ:Types⟧) which define
//! type definitions and expressions.

use crate::ast::{TypesBlock, TypeDefinition, TypeExpression, BasicType, Span};
use crate::error::*;
use crate::lexer::AispLexer;
use crate::token_parser::TokenParser;
use std::collections::HashMap;

/// Parser for AISP Types blocks
pub struct TypesParser;

impl TypesParser {
    /// Parse types block content  
    pub fn parse_types_block(lexer: &mut AispLexer) -> AispResult<TypesBlock> {
        let (start_line, _) = lexer.position_info();
        let mut definitions = HashMap::new();
        
        while !lexer.check('}') && !lexer.is_at_end() {
            lexer.skip_whitespace_and_comments();
            if lexer.check('}') || lexer.is_at_end() {
                break;
            }
            
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
            
            let name = TokenParser::parse_identifier(lexer)?;
            
            if !lexer.match_char('≜') {
                return Err(lexer.parse_error("Expected '≜' in type definition"));
            }
            
            let type_expr = Self::parse_type_expression(lexer)?;
            let (line, column) = lexer.position_info();
            let span = Span::new(line, 1, line, column);
            
            definitions.insert(
                name.clone(),
                TypeDefinition {
                    name: name.clone(),
                    type_expr,
                    span,
                },
            );
            
            // Skip to next line or entry
            lexer.skip_whitespace_and_comments();
        }
        
        let (end_line, end_column) = lexer.position_info();
        Ok(TypesBlock {
            definitions,
            span: Span::new(start_line, 1, end_line, end_column),
        })
    }

    /// Parse type expression
    fn parse_type_expression(lexer: &mut AispLexer) -> AispResult<TypeExpression> {
        lexer.skip_whitespace_and_comments();
        
        if let Some(ch) = lexer.peek() {
            match ch {
                '{' => {
                    // Enumeration type {A, B, C} or {A B C} (AISP allows space separation)
                    lexer.advance(); // consume {
                    let mut values = Vec::new();
                    
                    while !lexer.check('}') && !lexer.is_at_end() {
                        lexer.skip_whitespace_and_comments();
                        if lexer.check('}') {
                            break;
                        }
                        
                        values.push(TokenParser::parse_identifier(lexer)?);
                        
                        lexer.skip_whitespace_and_comments();
                        
                        // Handle both comma-separated and space-separated variants
                        if lexer.match_char(',') {
                            // Comma found, continue to next item
                            continue;
                        } else if lexer.check('}') {
                            // End of enumeration, break
                            break;
                        } else {
                            // Check if there's another identifier (space-separated)
                            if let Some(ch) = lexer.peek() {
                                if ch.is_alphabetic() || ch == '_' {
                                    // Space-separated variant, continue parsing
                                    continue;
                                } else if ch == '}' {
                                    // End of enumeration
                                    break;
                                } else {
                                    // Invalid character
                                    return Err(lexer.parse_error(&format!("Unexpected character '{}' in enumeration", ch)));
                                }
                            } else {
                                break;
                            }
                        }
                    }
                    
                    if !lexer.match_char('}') {
                        return Err(lexer.parse_error("Expected '}' to close enumeration"));
                    }
                    
                    Ok(TypeExpression::Enumeration(values))
                }
                'ℕ' => {
                    lexer.advance();
                    Self::parse_type_suffix(lexer, TypeExpression::Basic(BasicType::Natural))
                }
                'ℤ' => {
                    lexer.advance();
                    Self::parse_type_suffix(lexer, TypeExpression::Basic(BasicType::Integer))
                }
                'ℝ' => {
                    lexer.advance();
                    Self::parse_type_suffix(lexer, TypeExpression::Basic(BasicType::Real))
                }
                '𝔹' => {
                    lexer.advance();
                    Self::parse_type_suffix(lexer, TypeExpression::Basic(BasicType::Boolean))
                }
                '𝕊' => {
                    lexer.advance();
                    Self::parse_type_suffix(lexer, TypeExpression::Basic(BasicType::String))
                }
                _ => {
                    // Type reference
                    let name = TokenParser::parse_identifier(lexer)?;
                    Self::parse_type_suffix(lexer, TypeExpression::Reference(name))
                }
            }
        } else {
            Err(lexer.parse_error("Expected type expression"))
        }
    }

    /// Parse type suffix (array, function arrow, etc.)
    fn parse_type_suffix(lexer: &mut AispLexer, base_type: TypeExpression) -> AispResult<TypeExpression> {
        lexer.skip_whitespace_and_comments(); // Skip any whitespace before checking suffix
        
        if lexer.match_char('[') {
            // Array type Type[n]
            let size = if lexer.check(']') {
                None
            } else {
                Some(TokenParser::parse_number(lexer)? as usize)
            };
            
            if !lexer.match_char(']') {
                return Err(lexer.parse_error("Expected ']' after array size"));
            }
            
            Ok(TypeExpression::Array {
                element_type: Box::new(base_type),
                size,
            })
        } else if lexer.match_char('→') || lexer.match_str("->") {
            // Function type A → B
            let output = Self::parse_type_expression(lexer)?;
            Ok(TypeExpression::Function {
                input: Box::new(base_type),
                output: Box::new(output),
            })
        } else {
            Ok(base_type)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_enumeration_type() {
        let mut lexer = AispLexer::new("{A, B, C}".to_string());
        let type_expr = TypesParser::parse_type_expression(&mut lexer).unwrap();
        
        match type_expr {
            TypeExpression::Enumeration(values) => {
                assert_eq!(values, vec!["A", "B", "C"]);
            }
            _ => panic!("Expected enumeration type"),
        }
    }

    #[test]
    fn test_parse_basic_types() {
        let test_cases = vec![
            ("ℕ", BasicType::Natural),
            ("ℤ", BasicType::Integer),
            ("ℝ", BasicType::Real),
            ("𝔹", BasicType::Boolean),
            ("𝕊", BasicType::String),
        ];

        for (input, expected) in test_cases {
            let mut lexer = AispLexer::new(input.to_string());
            let type_expr = TypesParser::parse_type_expression(&mut lexer).unwrap();
            
            match type_expr {
                TypeExpression::Basic(basic_type) => {
                    assert_eq!(basic_type, expected);
                }
                _ => panic!("Expected basic type for {}", input),
            }
        }
    }

    #[test]
    fn test_parse_type_reference() {
        let mut lexer = AispLexer::new("MyType".to_string());
        let type_expr = TypesParser::parse_type_expression(&mut lexer).unwrap();
        
        match type_expr {
            TypeExpression::Reference(name) => {
                assert_eq!(name, "MyType");
            }
            _ => panic!("Expected type reference"),
        }
    }

    #[test]
    fn test_parse_array_type() {
        let mut lexer = AispLexer::new("ℕ[10]".to_string());
        let type_expr = TypesParser::parse_type_expression(&mut lexer).unwrap();
        
        match type_expr {
            TypeExpression::Array { element_type, size } => {
                match *element_type {
                    TypeExpression::Basic(BasicType::Natural) => {},
                    _ => panic!("Expected Natural element type"),
                }
                assert_eq!(size, Some(10));
            }
            _ => panic!("Expected array type"),
        }
    }

    #[test]
    fn test_parse_function_type() {
        let mut lexer = AispLexer::new("ℕ → 𝔹".to_string());
        let type_expr = TypesParser::parse_type_expression(&mut lexer).unwrap();
        
        match type_expr {
            TypeExpression::Function { input, output } => {
                match (*input, *output) {
                    (TypeExpression::Basic(BasicType::Natural), TypeExpression::Basic(BasicType::Boolean)) => {},
                    _ => panic!("Expected ℕ → 𝔹 function type"),
                }
            }
            _ => panic!("Expected function type"),
        }
    }

    #[test]
    fn test_parse_types_block() {
        let mut lexer = AispLexer::new("State≜{A,B,C}\nValue≜ℕ\n".to_string());
        let types_block = TypesParser::parse_types_block(&mut lexer).unwrap();
        
        assert_eq!(types_block.definitions.len(), 2);
        assert!(types_block.definitions.contains_key("State"));
        assert!(types_block.definitions.contains_key("Value"));
    }

    #[test]
    fn test_empty_enumeration() {
        let mut lexer = AispLexer::new("{}".to_string());
        let type_expr = TypesParser::parse_type_expression(&mut lexer).unwrap();
        
        match type_expr {
            TypeExpression::Enumeration(values) => {
                assert!(values.is_empty());
            }
            _ => panic!("Expected empty enumeration"),
        }
    }
}