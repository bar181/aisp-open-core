//! Lexical analysis for AISP documents
//!
//! This module handles character-level parsing, position tracking, and basic tokenization
//! for AISP documents with proper Unicode support.

use crate::error::*;

/// Lexer for AISP documents with Unicode-aware character handling
#[derive(Debug)]
pub struct AispLexer {
    /// Source text being parsed
    source: String,
    /// Current parsing position (character index)
    position: usize,
    /// Current line number (1-indexed)
    line: usize,
    /// Current column number (1-indexed)  
    column: usize,
}

impl AispLexer {
    /// Create a new lexer for the given source
    pub fn new(source: String) -> Self {
        Self {
            source,
            position: 0,
            line: 1,
            column: 1,
        }
    }

    /// Check if we've reached the end of input
    pub fn is_at_end(&self) -> bool {
        self.position >= self.source.chars().count()
    }

    /// Peek at the current character without advancing
    pub fn peek(&self) -> Option<char> {
        self.source.chars().nth(self.position)
    }

    /// Peek at the character at offset positions ahead
    pub fn peek_ahead(&self, offset: usize) -> Option<char> {
        self.source.chars().nth(self.position + offset)
    }

    /// Advance to the next character, updating position and line/column
    pub fn advance(&mut self) -> Option<char> {
        if let Some(ch) = self.peek() {
            self.position += 1;
            
            if ch == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
            Some(ch)
        } else {
            None
        }
    }

    /// Check if the current character matches expected without consuming
    pub fn check(&self, expected: char) -> bool {
        self.peek() == Some(expected)
    }

    /// Consume character if it matches expected
    pub fn match_char(&mut self, expected: char) -> bool {
        if self.check(expected) {
            self.advance();
            true
        } else {
            false
        }
    }

    /// Match a string at current position
    pub fn match_str(&mut self, expected: &str) -> bool {
        let start_pos = self.position;
        let chars_from_pos: Vec<char> = self.source.chars().skip(start_pos).collect();
        let expected_chars: Vec<char> = expected.chars().collect();
        
        if chars_from_pos.len() < expected_chars.len() {
            return false;
        }
        
        // Check if the expected string matches at current position
        for (i, &expected_char) in expected_chars.iter().enumerate() {
            if i >= chars_from_pos.len() || chars_from_pos[i] != expected_char {
                return false;
            }
        }
        
        // If we got here, the string matches - now consume it
        for _ in expected.chars() {
            self.advance();
        }
        true
    }

    /// Skip whitespace and comments
    pub fn skip_whitespace_and_comments(&mut self) {
        while let Some(ch) = self.peek() {
            if ch.is_whitespace() {
                self.advance();
            } else if ch == '/' {
                // Check if next character is also '/'
                let chars_from_pos: Vec<char> = self.source.chars().skip(self.position).collect();
                if chars_from_pos.len() > 1 && chars_from_pos[1] == '/' {
                    // Skip line comment
                    while let Some(ch) = self.peek() {
                        self.advance();
                        if ch == '\n' {
                            break;
                        }
                    }
                } else {
                    break;
                }
            } else {
                break;
            }
        }
    }

    /// Get current position info
    pub fn position_info(&self) -> (usize, usize) {
        (self.line, self.column)
    }

    /// Get remaining source from current position
    pub fn remaining_source(&self) -> String {
        self.source.chars().skip(self.position).collect()
    }

    /// Create parse error at current position
    pub fn parse_error(&self, message: impl Into<String>) -> AispError {
        AispError::parse_error(self.line, self.column, message)
    }

    /// Get the source string
    pub fn source(&self) -> &str {
        &self.source
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_lexer_operations() {
        let mut lexer = AispLexer::new("abc".to_string());
        
        assert_eq!(lexer.peek(), Some('a'));
        assert_eq!(lexer.advance(), Some('a'));
        assert_eq!(lexer.peek(), Some('b'));
        assert_eq!(lexer.advance(), Some('b'));
        assert_eq!(lexer.advance(), Some('c'));
        assert!(lexer.is_at_end());
    }

    #[test]
    fn test_unicode_handling() {
        let mut lexer = AispLexer::new("𝔸5.1⟦".to_string());
        
        assert_eq!(lexer.advance(), Some('𝔸'));
        assert_eq!(lexer.advance(), Some('5'));
        assert_eq!(lexer.advance(), Some('.'));
        assert_eq!(lexer.advance(), Some('1'));
        assert_eq!(lexer.advance(), Some('⟦'));
        assert!(lexer.is_at_end());
    }

    #[test]
    fn test_line_column_tracking() {
        let mut lexer = AispLexer::new("line1\nline2".to_string());
        
        assert_eq!(lexer.position_info(), (1, 1));
        for _ in 0..5 { lexer.advance(); } // consume "line1"
        assert_eq!(lexer.position_info(), (1, 6));
        lexer.advance(); // consume '\n'
        assert_eq!(lexer.position_info(), (2, 1));
    }

    #[test]
    fn test_string_matching() {
        let mut lexer = AispLexer::new("hello world".to_string());
        
        assert!(lexer.match_str("hello"));
        assert!(!lexer.match_str("foo"));
        lexer.skip_whitespace_and_comments();
        assert!(lexer.match_str("world"));
    }

    #[test]
    fn test_whitespace_skipping() {
        let mut lexer = AispLexer::new("  \t\n  abc".to_string());
        
        lexer.skip_whitespace_and_comments();
        assert_eq!(lexer.peek(), Some('a'));
    }

    #[test]
    fn test_comment_skipping() {
        let mut lexer = AispLexer::new("abc // comment\ndef".to_string());
        
        assert_eq!(lexer.advance(), Some('a'));
        assert_eq!(lexer.advance(), Some('b'));
        assert_eq!(lexer.advance(), Some('c'));
        lexer.skip_whitespace_and_comments();
        assert_eq!(lexer.peek(), Some('d'));
    }
}