//! Parser for logical expressions in AISP documents
//!
//! This module handles parsing logical rules, quantifiers, lambda expressions,
//! and other logical constructs used in Rules and Functions blocks.

use crate::ast::{
    LogicalRule, Quantifier, QuantifierKind, LogicalExpression, 
    LambdaExpression, Span
};
use crate::error::*;
use crate::lexer::AispLexer;
use crate::token_parser::TokenParser;

/// Parser for logical expressions and constructs
pub struct LogicParser;

impl LogicParser {
    /// Parse a logical rule
    pub fn parse_logical_rule(lexer: &mut AispLexer) -> AispResult<LogicalRule> {
        let (start_line, _) = lexer.position_info();
        
        // Check for quantifier
        let quantifier = if lexer.peek() == Some('∀') || lexer.peek() == Some('∃') {
            Some(Self::parse_quantifier(lexer)?)
        } else {
            None
        };
        
        let expression = Self::parse_logical_expression(lexer)?;
        
        let (end_line, end_column) = lexer.position_info();
        Ok(LogicalRule {
            quantifier,
            expression,
            span: Span::new(start_line, 1, end_line, end_column),
        })
    }

    /// Parse quantifier (∀ or ∃)
    pub fn parse_quantifier(lexer: &mut AispLexer) -> AispResult<Quantifier> {
        let (start_line, _) = lexer.position_info();
        
        let kind = match lexer.advance() {
            Some('∀') => QuantifierKind::Universal,
            Some('∃') => QuantifierKind::Existential,
            _ => {
                return Err(lexer.parse_error("Expected quantifier"));
            }
        };
        
        let variable = TokenParser::parse_identifier(lexer)?;
        
        // Optional domain specification
        let domain = if lexer.match_char('∈') || lexer.match_char(':') {
            Some(TokenParser::parse_identifier(lexer)?)
        } else {
            None
        };
        
        let (end_line, end_column) = lexer.position_info();
        Ok(Quantifier {
            kind,
            variable,
            domain,
            span: Span::new(start_line, 1, end_line, end_column),
        })
    }

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

    /// Parse lambda expression
    pub fn parse_lambda_expression(lexer: &mut AispLexer) -> AispResult<LambdaExpression> {
        let (start_line, _) = lexer.position_info();
        
        if !lexer.match_char('λ') {
            return Err(lexer.parse_error("Expected 'λ' in lambda expression"));
        }
        
        // Parse parameters - can be with or without parentheses
        let mut parameters = Vec::new();
        
        if lexer.check('(') {
            // Parse parameters with parentheses: λ(x,y)
            lexer.advance(); // consume '('
            while !lexer.check(')') && !lexer.is_at_end() {
                parameters.push(TokenParser::parse_identifier(lexer)?);
                if lexer.match_char(',') {
                    continue;
                } else if lexer.check(')') {
                    break;
                } else {
                    return Err(lexer.parse_error("Expected ',' or ')' in parameter list"));
                }
            }
            
            if !lexer.match_char(')') {
                return Err(lexer.parse_error("Expected ')' after parameter list"));
            }
        } else {
            // Parse single parameter without parentheses: λx
            // Read until we hit '.'
            while !lexer.check('.') && !lexer.is_at_end() {
                let param_char = lexer.advance().unwrap();
                if param_char.is_alphanumeric() {
                    // Simple single-letter parameter
                    parameters.push(param_char.to_string());
                    break;
                }
            }
        }
        
        if !lexer.match_char('.') {
            return Err(lexer.parse_error("Expected '.' after lambda parameters"));
        }
        
        let body = Self::parse_logical_expression(lexer)?;
        
        let (end_line, end_column) = lexer.position_info();
        Ok(LambdaExpression {
            parameters,
            body,
            span: Span::new(start_line, 1, end_line, end_column),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_universal_quantifier() {
        let mut lexer = AispLexer::new("∀x∈State".to_string());
        let quantifier = LogicParser::parse_quantifier(&mut lexer).unwrap();
        
        assert_eq!(quantifier.kind, QuantifierKind::Universal);
        assert_eq!(quantifier.variable, "x");
        assert_eq!(quantifier.domain, Some("State".to_string()));
    }

    #[test]
    fn test_parse_existential_quantifier() {
        let mut lexer = AispLexer::new("∃y".to_string());
        let quantifier = LogicParser::parse_quantifier(&mut lexer).unwrap();
        
        assert_eq!(quantifier.kind, QuantifierKind::Existential);
        assert_eq!(quantifier.variable, "y");
        assert_eq!(quantifier.domain, None);
    }

    #[test]
    fn test_parse_logical_rule_with_quantifier() {
        let mut lexer = AispLexer::new("∀x:State→Valid(x)".to_string());
        let rule = LogicParser::parse_logical_rule(&mut lexer).unwrap();
        
        assert!(rule.quantifier.is_some());
        let quantifier = rule.quantifier.unwrap();
        assert_eq!(quantifier.kind, QuantifierKind::Universal);
        assert_eq!(quantifier.variable, "x");
        assert_eq!(quantifier.domain, Some("State".to_string()));
    }

    #[test]
    fn test_parse_logical_rule_without_quantifier() {
        let mut lexer = AispLexer::new("ValidState".to_string());
        let rule = LogicParser::parse_logical_rule(&mut lexer).unwrap();
        
        assert!(rule.quantifier.is_none());
        match rule.expression {
            LogicalExpression::Variable(expr) => assert_eq!(expr, "ValidState"),
            _ => panic!("Expected variable expression"),
        }
    }

    #[test]
    fn test_parse_lambda_with_single_parameter() {
        let mut lexer = AispLexer::new("λx.Next(x)".to_string());
        let lambda = LogicParser::parse_lambda_expression(&mut lexer).unwrap();
        
        assert_eq!(lambda.parameters, vec!["x"]);
        match lambda.body {
            LogicalExpression::Variable(expr) => assert_eq!(expr, "Next(x)"),
            _ => panic!("Expected variable expression"),
        }
    }

    #[test]
    fn test_parse_lambda_with_multiple_parameters() {
        let mut lexer = AispLexer::new("λ(x,y).Add(x,y)".to_string());
        let lambda = LogicParser::parse_lambda_expression(&mut lexer).unwrap();
        
        assert_eq!(lambda.parameters, vec!["x", "y"]);
        match lambda.body {
            LogicalExpression::Variable(expr) => assert_eq!(expr, "Add(x,y)"),
            _ => panic!("Expected variable expression"),
        }
    }

    #[test]
    fn test_parse_complex_logical_expression() {
        let mut lexer = AispLexer::new("□(Playing→◊GameOver)".to_string());
        let expr = LogicParser::parse_logical_expression(&mut lexer).unwrap();
        
        match expr {
            LogicalExpression::Variable(text) => {
                assert_eq!(text, "□(Playing→◊GameOver)");
            }
            _ => panic!("Expected variable expression"),
        }
    }

    #[test]
    fn test_parse_logical_expression_with_symbols() {
        let mut lexer = AispLexer::new("∀p:Player→HasTurn(p)⇒CanMove(p)".to_string());
        let expr = LogicParser::parse_logical_expression(&mut lexer).unwrap();
        
        match expr {
            LogicalExpression::Variable(text) => {
                assert!(text.contains("Player"));
                assert!(text.contains("HasTurn"));
                assert!(text.contains("CanMove"));
            }
            _ => panic!("Expected variable expression"),
        }
    }

    #[test]
    fn test_empty_lambda_error() {
        let mut lexer = AispLexer::new("λ".to_string());
        let result = LogicParser::parse_lambda_expression(&mut lexer);
        assert!(result.is_err());
    }
}