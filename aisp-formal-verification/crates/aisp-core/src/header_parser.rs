//! Parser for AISP document headers
//!
//! This module handles parsing the document header (𝔸5.1.name@date) and
//! document metadata (γ and ρ declarations).

use crate::ast::canonical::{DocumentHeader, DocumentMetadata, HeaderMetadata};
use crate::error::*;
use crate::lexer::AispLexer;
use crate::token_parser::TokenParser;

/// Parser for AISP document headers and metadata
pub struct HeaderParser;

impl HeaderParser {
    /// Parse document header (𝔸5.1.name@date)
    pub fn parse_header(lexer: &mut AispLexer) -> AispResult<DocumentHeader> {
        lexer.skip_whitespace_and_comments();
        
        // Expect 𝔸 symbol
        if !lexer.match_char('𝔸') {
            return Err(lexer.parse_error("Expected AISP header starting with 𝔸"));
        }
        
        // Parse version
        let version = TokenParser::parse_version(lexer)?;
        
        // Expect dot
        if !lexer.match_char('.') {
            return Err(lexer.parse_error("Expected '.' after version"));
        }
        
        // Parse name
        let name = TokenParser::parse_identifier(lexer)?;
        
        // Parse optional metadata and date
        let (metadata, date) = if lexer.match_char('@') {
            let date = TokenParser::parse_date(lexer)?;
            (None, date)
        } else if lexer.peek() == Some('#') {
            lexer.advance(); // consume #
            let meta = TokenParser::parse_identifier(lexer)?;
            if lexer.match_char('@') {
                let date = TokenParser::parse_date(lexer)?;
                (Some(meta), date)
            } else {
                return Err(lexer.parse_error("Expected '@' after metadata"));
            }
        } else {
            return Err(lexer.parse_error("Expected '@' or '#' after name"));
        };
        
        Ok(DocumentHeader {
            version,
            name,
            date,
            metadata: metadata.map(|m| HeaderMetadata {
                author: Some(m),
                description: None,
                tags: Vec::new(),
            }),
        })
    }

    /// Parse document metadata (γ and ρ declarations)
    pub fn parse_metadata(lexer: &mut AispLexer) -> AispResult<DocumentMetadata> {
        lexer.skip_whitespace_and_comments();
        
        let mut domain = None;
        let mut protocol = None;
        
        // Look for γ (domain) and ρ (protocol) declarations
        while !lexer.is_at_end() && (lexer.peek() == Some('γ') || lexer.peek() == Some('ρ')) {
            let var = lexer.advance().unwrap();
            
            if !lexer.match_char('≔') {
                return Err(lexer.parse_error(format!("Expected '≔' after '{}'", var)));
            }
            
            let value = Self::parse_metadata_value(lexer)?;
            
            match var {
                'γ' => domain = Some(value),
                'ρ' => protocol = Some(value),
                _ => unreachable!(),
            }
            
            lexer.skip_whitespace_and_comments();
        }
        
        Ok(DocumentMetadata { domain, protocol })
    }

    /// Parse metadata value (tuple or simple identifier)
    fn parse_metadata_value(lexer: &mut AispLexer) -> AispResult<String> {
        lexer.skip_whitespace_and_comments();
        
        if lexer.match_char('⟨') {
            // Parse tuple value like ⟨game,turn-based⟩
            let mut value = String::new();
            while !lexer.check('⟩') && !lexer.is_at_end() {
                if let Some(ch) = lexer.advance() {
                    value.push(ch);
                }
            }
            
            if !lexer.match_char('⟩') {
                return Err(lexer.parse_error("Expected '⟩' to close tuple"));
            }
            
            Ok(value.trim().to_string())
        } else {
            // Parse simple identifier
            TokenParser::parse_identifier(lexer)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_basic_header() {
        let mut lexer = AispLexer::new("𝔸5.1.test@2026-01-25".to_string());
        let header = HeaderParser::parse_header(&mut lexer).unwrap();
        
        assert_eq!(header.version, "5.1");
        assert_eq!(header.name, "test");
        assert_eq!(header.date, "2026-01-25");
        assert!(header.metadata.is_none());
    }

    #[test]
    fn test_parse_header_with_metadata() {
        let mut lexer = AispLexer::new("𝔸5.1.test#meta@2026-01-25".to_string());
        let header = HeaderParser::parse_header(&mut lexer).unwrap();
        
        assert_eq!(header.version, "5.1");
        assert_eq!(header.name, "test");
        assert_eq!(header.date, "2026-01-25");
        assert!(header.metadata.is_some());
        assert_eq!(header.metadata.as_ref().unwrap().author, Some("meta".to_string()));
    }

    #[test]
    fn test_parse_metadata_simple() {
        let mut lexer = AispLexer::new("γ≔game\nρ≔protocol".to_string());
        let metadata = HeaderParser::parse_metadata(&mut lexer).unwrap();
        
        assert_eq!(metadata.domain, Some("game".to_string()));
        assert_eq!(metadata.protocol, Some("protocol".to_string()));
    }

    #[test]
    fn test_parse_metadata_tuple() {
        let mut lexer = AispLexer::new("γ≔⟨game,turn-based⟩".to_string());
        let metadata = HeaderParser::parse_metadata(&mut lexer).unwrap();
        
        assert_eq!(metadata.domain, Some("game,turn-based".to_string()));
        assert_eq!(metadata.protocol, None);
    }

    #[test]
    fn test_parse_empty_metadata() {
        let mut lexer = AispLexer::new("".to_string());
        let metadata = HeaderParser::parse_metadata(&mut lexer).unwrap();
        
        assert_eq!(metadata.domain, None);
        assert_eq!(metadata.protocol, None);
    }

    #[test]
    fn test_invalid_header_error() {
        let mut lexer = AispLexer::new("A5.1.test@2026-01-25".to_string()); // wrong symbol
        let result = HeaderParser::parse_header(&mut lexer);
        assert!(result.is_err());
    }

    #[test]
    fn test_missing_version_dot() {
        let mut lexer = AispLexer::new("𝔸51test@2026-01-25".to_string());
        let result = HeaderParser::parse_header(&mut lexer);
        assert!(result.is_err());
    }
}