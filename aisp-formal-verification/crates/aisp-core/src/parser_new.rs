//! High-level AISP document parser
//!
//! This module orchestrates the parsing of complete AISP documents
//! using focused, SRP-compliant parser modules.

use crate::ast::*;
use crate::error::*;
use crate::lexer::AispLexer;
use crate::header_parser::HeaderParser;
use crate::meta_parser::MetaParser;
use crate::types_parser::TypesParser;
use crate::evidence_parser::EvidenceParser;
use crate::logic_parser::LogicParser;
use crate::token_parser::TokenParser;
use std::collections::HashMap;

/// High-level AISP parser with support for incremental parsing
pub struct AispParser {
    /// Lexer for character-level operations
    lexer: AispLexer,
    /// Collected warnings during parsing
    warnings: Vec<AispWarning>,
}

impl AispParser {
    /// Create a new parser for the given source
    pub fn new(source: String) -> Self {
        Self {
            lexer: AispLexer::new(source),
            warnings: Vec::new(),
        }
    }

    /// Parse complete AISP document
    pub fn parse(&mut self) -> AispResult<AispDocument> {
        // Parse document header
        let header = HeaderParser::parse_header(&mut self.lexer)?;
        
        // Parse metadata (γ and ρ)
        let metadata = HeaderParser::parse_metadata(&mut self.lexer)?;
        
        // Parse all blocks
        let mut blocks = Vec::new();
        
        while !self.lexer.is_at_end() {
            self.lexer.skip_whitespace_and_comments();
            if self.lexer.is_at_end() {
                break;
            }
            
            let block = self.parse_block()?;
            blocks.push(block);
        }
        
        // Validate required blocks are present
        self.validate_required_blocks(&blocks)?;
        
        let (line, column) = self.lexer.position_info();
        let span = Span::new(1, 1, line, column);
        
        Ok(AispDocument {
            header,
            metadata,
            blocks,
            span,
        })
    }

    /// Get collected warnings
    pub fn warnings(&self) -> &[AispWarning] {
        &self.warnings
    }

    /// Parse a block (⟦Type:Name⟧{...})
    fn parse_block(&mut self) -> AispResult<AispBlock> {
        // Expect ⟦
        if !self.lexer.match_char('⟦') {
            return Err(self.lexer.parse_error("Expected '⟦' to start block"));
        }
        
        // Parse block identifier
        let block_id = TokenParser::parse_block_identifier(&mut self.lexer)?;
        
        // Expect ⟧
        if !self.lexer.match_char('⟧') {
            return Err(self.lexer.parse_error("Expected '⟧' after block identifier"));
        }
        
        // Parse block content based on type
        let block = match block_id.as_str() {
            "Ε" => {
                // Evidence block uses ⟨⟩ instead of {}
                AispBlock::Evidence(EvidenceParser::parse_evidence_block(&mut self.lexer)?)
            }
            _ => {
                // Other blocks use {}
                if !self.lexer.match_char('{') {
                    return Err(self.lexer.parse_error("Expected '{' after block header"));
                }
                
                let block = match block_id.as_str() {
                    "Ω:Meta" => AispBlock::Meta(MetaParser::parse_meta_block(&mut self.lexer)?),
                    "Σ:Types" => AispBlock::Types(TypesParser::parse_types_block(&mut self.lexer)?),
                    "Γ:Rules" => AispBlock::Rules(self.parse_rules_block()?),
                    "Λ:Funcs" => AispBlock::Functions(self.parse_functions_block()?),
                    _ => {
                        return Err(self.lexer.parse_error(format!("Unknown block type: {}", block_id)));
                    }
                };
                
                // Expect }
                if !self.lexer.match_char('}') {
                    return Err(self.lexer.parse_error("Expected '}' to close block"));
                }
                
                block
            }
        };
        
        Ok(block)
    }

    /// Parse rules block (logical rules)
    fn parse_rules_block(&mut self) -> AispResult<RulesBlock> {
        let (start_line, _) = self.lexer.position_info();
        let mut rules = Vec::new();
        
        while !self.lexer.check('}') && !self.lexer.is_at_end() {
            self.lexer.skip_whitespace_and_comments();
            if self.lexer.check('}') {
                break;
            }
            
            rules.push(LogicParser::parse_logical_rule(&mut self.lexer)?);
        }
        
        let (end_line, end_column) = self.lexer.position_info();
        Ok(RulesBlock {
            rules,
            span: Span::new(start_line, 1, end_line, end_column),
        })
    }

    /// Parse functions block
    fn parse_functions_block(&mut self) -> AispResult<FunctionsBlock> {
        let (start_line, _) = self.lexer.position_info();
        let mut functions = HashMap::new();
        
        while !self.lexer.check('}') && !self.lexer.is_at_end() {
            self.lexer.skip_whitespace_and_comments();
            if self.lexer.check('}') {
                break;
            }
            
            let name = TokenParser::parse_identifier(&mut self.lexer)?;
            
            if !self.lexer.match_char('≜') {
                return Err(self.lexer.parse_error("Expected '≜' in function definition"));
            }
            
            let lambda = LogicParser::parse_lambda_expression(&mut self.lexer)?;
            let (line, column) = self.lexer.position_info();
            let span = Span::new(line, 1, line, column);
            
            functions.insert(
                name.clone(),
                FunctionDefinition {
                    name: name.clone(),
                    lambda,
                    span,
                },
            );
        }
        
        let (end_line, end_column) = self.lexer.position_info();
        Ok(FunctionsBlock {
            functions,
            span: Span::new(start_line, 1, end_line, end_column),
        })
    }

    /// Validate that all required blocks are present
    fn validate_required_blocks(&self, blocks: &[AispBlock]) -> AispResult<()> {
        let mut has_meta = false;
        let mut has_types = false;
        let mut has_rules = false;
        let mut has_functions = false;
        let mut has_evidence = false;
        
        for block in blocks {
            match block {
                AispBlock::Meta(_) => has_meta = true,
                AispBlock::Types(_) => has_types = true,
                AispBlock::Rules(_) => has_rules = true,
                AispBlock::Functions(_) => has_functions = true,
                AispBlock::Evidence(_) => has_evidence = true,
            }
        }
        
        if !has_meta {
            return Err(AispError::MissingBlock {
                block_name: "Meta".to_string(),
            });
        }
        if !has_types {
            return Err(AispError::MissingBlock {
                block_name: "Types".to_string(),
            });
        }
        if !has_rules {
            return Err(AispError::MissingBlock {
                block_name: "Rules".to_string(),
            });
        }
        if !has_functions {
            return Err(AispError::MissingBlock {
                block_name: "Functions".to_string(),
            });
        }
        if !has_evidence {
            return Err(AispError::MissingBlock {
                block_name: "Evidence".to_string(),
            });
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_minimal_document() {
        let source = r#"
𝔸5.1.test@2026-01-25

⟦Ω:Meta⟧{
  domain≜test
}

⟦Σ:Types⟧{
  State≜{A,B}
}

⟦Γ:Rules⟧{
  ∀x:State→Next(x)
}

⟦Λ:Funcs⟧{
  next≜λx.Next(x)
}

⟦Ε⟧⟨δ≜0.8⟩
        "#.trim();

        let mut parser = AispParser::new(source.to_string());
        let doc = parser.parse().unwrap();
        
        assert_eq!(doc.blocks.len(), 5);
        assert_eq!(doc.header.name, "test");
        assert!(parser.warnings().is_empty());
    }

    #[test]
    fn test_parse_document_with_metadata() {
        let source = r#"
𝔸5.1.GameLogic@2026-01-25

γ≔⟨game,turn-based⟩
ρ≔⟨protocol,state-transition⟩

⟦Ω:Meta⟧{
  domain≜game_logic
}

⟦Σ:Types⟧{
  State≜{Start,End}
}

⟦Γ:Rules⟧{
  ValidState
}

⟦Λ:Funcs⟧{
  next≜λs.Transition(s)
}

⟦Ε⟧⟨δ≜0.9;τ≜◊⁺⟩
        "#.trim();

        let mut parser = AispParser::new(source.to_string());
        let doc = parser.parse().unwrap();
        
        assert_eq!(doc.header.name, "GameLogic");
        assert!(doc.metadata.domain.is_some());
        assert!(doc.metadata.protocol.is_some());
        assert_eq!(doc.blocks.len(), 5);
    }

    #[test]
    fn test_parse_missing_block_error() {
        let source = "𝔸5.1.test@2026-01-25"; // missing all blocks

        let mut parser = AispParser::new(source.to_string());
        let result = parser.parse();
        
        assert!(result.is_err());
        match result.unwrap_err() {
            AispError::MissingBlock { block_name } => {
                assert_eq!(block_name, "Meta");
            }
            _ => panic!("Expected missing block error"),
        }
    }

    #[test]
    fn test_parse_invalid_block_type() {
        let source = r#"
𝔸5.1.test@2026-01-25

⟦Invalid⟧{
  test≜value
}
        "#.trim();

        let mut parser = AispParser::new(source.to_string());
        let result = parser.parse();
        
        assert!(result.is_err());
    }
}

/// Standalone parse function for compatibility
pub fn parse(source: &str) -> AispResult<AispDocument> {
    let mut parser = AispParser::new(source.to_string());
    parser.parse()
}