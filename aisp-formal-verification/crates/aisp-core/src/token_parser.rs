//! Token parsing utilities for AISP documents
//!
//! This module provides parsers for basic tokens like identifiers, numbers, 
//! strings, and AISP-specific symbols.

use crate::error::*;
use crate::lexer::AispLexer;
use crate::symbols::is_aisp_symbol;

/// Parser for basic tokens in AISP documents
pub struct TokenParser;

impl TokenParser {
    /// Parse a standard identifier (alphanumeric, underscore, hyphen)
    /// Must start with a letter or underscore, not a digit
    pub fn parse_identifier(lexer: &mut AispLexer) -> AispResult<String> {
        lexer.skip_whitespace_and_comments();
        let mut identifier = String::new();
        
        // First character must be letter or underscore
        if let Some(ch) = lexer.peek() {
            if ch.is_alphabetic() || ch == '_' {
                identifier.push(lexer.advance().unwrap());
            } else {
                return Err(lexer.parse_error("Expected identifier"));
            }
        } else {
            return Err(lexer.parse_error("Expected identifier"));
        }
        
        // Subsequent characters can be alphanumeric, underscore, or hyphen
        while let Some(ch) = lexer.peek() {
            if ch.is_alphanumeric() || ch == '_' || ch == '-' {
                identifier.push(lexer.advance().unwrap());
            } else {
                break;
            }
        }
        
        Ok(identifier)
    }

    /// Parse a quality tier symbol (includes ◊, ⊘, ⁺, ⁻)
    pub fn parse_tier_symbol(lexer: &mut AispLexer) -> AispResult<String> {
        lexer.skip_whitespace_and_comments();
        let mut symbol = String::new();
        
        while let Some(ch) = lexer.peek() {
            if ch.is_alphanumeric() || ch == '_' || ch == '-' || 
               ch == '◊' || ch == '⊘' || ch == '⁺' || ch == '⁻' {
                symbol.push(lexer.advance().unwrap());
            } else {
                break;
            }
        }
        
        if symbol.is_empty() {
            return Err(lexer.parse_error("Expected tier symbol"));
        }
        
        Ok(symbol)
    }

    /// Parse a numeric value (supports decimals and negative numbers)
    pub fn parse_number(lexer: &mut AispLexer) -> AispResult<f64> {
        lexer.skip_whitespace_and_comments();
        let mut number = String::new();
        
        // Handle negative numbers
        if lexer.peek() == Some('-') {
            number.push(lexer.advance().unwrap());
        }
        
        // Parse digits
        while lexer.peek().map_or(false, |c| c.is_ascii_digit()) {
            number.push(lexer.advance().unwrap());
        }
        
        // Handle decimal point
        if lexer.peek() == Some('.') {
            number.push(lexer.advance().unwrap());
            while lexer.peek().map_or(false, |c| c.is_ascii_digit()) {
                number.push(lexer.advance().unwrap());
            }
        }
        
        if number.is_empty() || number == "-" {
            return Err(lexer.parse_error("Expected number"));
        }
        
        number.parse::<f64>().map_err(|_| {
            lexer.parse_error("Invalid number format")
        })
    }

    /// Parse a quoted string
    pub fn parse_quoted_string(lexer: &mut AispLexer) -> AispResult<String> {
        lexer.skip_whitespace_and_comments();
        
        if !lexer.match_char('"') {
            return Err(lexer.parse_error("Expected opening quote"));
        }
        
        let mut value = String::new();
        while let Some(ch) = lexer.peek() {
            if ch == '"' {
                lexer.advance();
                break;
            }
            // TODO: Handle escape sequences if needed
            value.push(lexer.advance().unwrap());
        }
        
        Ok(value)
    }

    /// Parse a version number (e.g., "5.1")
    pub fn parse_version(lexer: &mut AispLexer) -> AispResult<String> {
        let mut version = String::new();
        
        // Parse major version
        if !lexer.peek().map_or(false, |c| c.is_ascii_digit()) {
            return Err(lexer.parse_error("Expected digit in version number"));
        }
        
        while lexer.peek().map_or(false, |c| c.is_ascii_digit()) {
            version.push(lexer.advance().unwrap());
        }
        
        // Expect dot
        if !lexer.match_char('.') {
            return Err(lexer.parse_error("Expected '.' in version number"));
        }
        version.push('.');
        
        // Parse minor version
        if !lexer.peek().map_or(false, |c| c.is_ascii_digit()) {
            return Err(lexer.parse_error("Expected digit after '.' in version number"));
        }
        
        while lexer.peek().map_or(false, |c| c.is_ascii_digit()) {
            version.push(lexer.advance().unwrap());
        }
        
        Ok(version)
    }

    /// Parse a date string (YYYY-MM-DD format)
    pub fn parse_date(lexer: &mut AispLexer) -> AispResult<String> {
        let mut date = String::new();
        
        // Parse YYYY-MM-DD format (flexible length)
        while let Some(ch) = lexer.peek() {
            if ch.is_ascii_digit() || ch == '-' {
                date.push(lexer.advance().unwrap());
            } else {
                break;
            }
        }
        
        // Basic validation - at least 4 digits at start
        if date.len() >= 4 && date.chars().take(4).all(|c| c.is_ascii_digit()) {
            Ok(date)
        } else {
            Err(lexer.parse_error("Invalid date format"))
        }
    }

    /// Parse block identifier (content between ⟦ and ⟧)
    pub fn parse_block_identifier(lexer: &mut AispLexer) -> AispResult<String> {
        let mut identifier = String::new();
        
        while let Some(ch) = lexer.peek() {
            if ch != '⟧' {
                identifier.push(lexer.advance().unwrap());
            } else {
                break;
            }
        }
        
        Ok(identifier.trim().to_string())
    }

    /// Check if next content looks like a logical expression
    pub fn is_logical_expression(lexer: &AispLexer) -> bool {
        let remaining = lexer.remaining_source();
        remaining.chars().any(|c| is_aisp_symbol(c))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_identifier() {
        let mut lexer = AispLexer::new("hello_world-123".to_string());
        let id = TokenParser::parse_identifier(&mut lexer).unwrap();
        assert_eq!(id, "hello_world-123");
    }

    #[test]
    fn test_parse_tier_symbol() {
        let mut lexer = AispLexer::new("◊⁺⁺".to_string());
        let tier = TokenParser::parse_tier_symbol(&mut lexer).unwrap();
        assert_eq!(tier, "◊⁺⁺");
    }

    #[test]
    fn test_parse_number() {
        let mut lexer = AispLexer::new("-123.456".to_string());
        let num = TokenParser::parse_number(&mut lexer).unwrap();
        assert_eq!(num, -123.456);
    }

    #[test]
    fn test_parse_quoted_string() {
        let mut lexer = AispLexer::new("\"hello world\"".to_string());
        let s = TokenParser::parse_quoted_string(&mut lexer).unwrap();
        assert_eq!(s, "hello world");
    }

    #[test]
    fn test_parse_version() {
        let mut lexer = AispLexer::new("5.1".to_string());
        let version = TokenParser::parse_version(&mut lexer).unwrap();
        assert_eq!(version, "5.1");
    }

    #[test]
    fn test_parse_date() {
        let mut lexer = AispLexer::new("2026-01-25".to_string());
        let date = TokenParser::parse_date(&mut lexer).unwrap();
        assert_eq!(date, "2026-01-25");
    }

    #[test]
    fn test_parse_block_identifier() {
        let mut lexer = AispLexer::new("Ω:Meta".to_string());
        let id = TokenParser::parse_block_identifier(&mut lexer).unwrap();
        assert_eq!(id, "Ω:Meta");
    }

    #[test]
    fn test_empty_identifier_error() {
        let mut lexer = AispLexer::new("123".to_string()); // starts with digit
        let result = TokenParser::parse_identifier(&mut lexer);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_number_error() {
        let mut lexer = AispLexer::new("abc".to_string());
        let result = TokenParser::parse_number(&mut lexer);
        assert!(result.is_err());
    }
}