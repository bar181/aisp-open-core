//! AISP document parser
//!
//! Minimal parser for AISP 5.1 documents.
//! Focuses on structure validation and metric extraction.

// Symbol imports for future use
#[allow(unused_imports)]
use crate::symbol::{SymbolId, starts_with_symbol};

/// Parse error codes
#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ParseError {
    /// Success
    Ok = 0,
    /// Invalid UTF-8
    InvalidUtf8 = -1,
    /// Missing header
    MissingHeader = -2,
    /// Invalid header format
    InvalidHeader = -3,
    /// Missing required block
    MissingBlock = -4,
    /// Unterminated block
    UnterminatedBlock = -5,
    /// Unexpected token
    UnexpectedToken = -6,
}

/// Block type in AISP document
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BlockType {
    Omega = 0,    // ⟦Ω⟧
    Sigma = 1,    // ⟦Σ⟧
    Gamma = 2,    // ⟦Γ⟧
    Lambda = 3,   // ⟦Λ⟧
    Chi = 4,      // ⟦Χ⟧
    Evidence = 5, // ⟦Ε⟧
    Category = 6, // ⟦ℭ⟧
    Theorems = 7, // ⟦Θ⟧
    Unknown = 255,
}

impl BlockType {
    /// Parse block type from tag character
    pub fn from_char(c: char) -> Self {
        match c {
            'Ω' => Self::Omega,
            'Σ' => Self::Sigma,
            'Γ' => Self::Gamma,
            'Λ' => Self::Lambda,
            'Χ' => Self::Chi,
            'Ε' => Self::Evidence,
            'ℭ' => Self::Category,
            'Θ' => Self::Theorems,
            _ => Self::Unknown,
        }
    }
}

/// Parsed block info
#[repr(C)]
#[derive(Clone, Copy)]
pub struct BlockInfo {
    pub block_type: BlockType,
    pub name_start: u16,
    pub name_len: u16,
    pub body_start: u16,
    pub body_end: u16,
}

/// Parser state
pub struct Parser<'a> {
    input: &'a str,
    pos: usize,
    blocks: [BlockInfo; 8],
    block_count: u8,
}

impl<'a> Parser<'a> {
    /// Create new parser
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            pos: 0,
            blocks: [BlockInfo {
                block_type: BlockType::Unknown,
                name_start: 0,
                name_len: 0,
                body_start: 0,
                body_end: 0,
            }; 8],
            block_count: 0,
        }
    }

    /// Parse document structure
    pub fn parse(&mut self) -> Result<(), ParseError> {
        // Check header
        if !self.input.starts_with("𝔸") {
            return Err(ParseError::MissingHeader);
        }

        // Skip to first block
        self.skip_to_block();

        // Parse blocks
        while self.pos < self.input.len() && self.block_count < 8 {
            if let Some(block) = self.try_parse_block() {
                self.blocks[self.block_count as usize] = block;
                self.block_count += 1;
            } else {
                self.pos += 1;
            }
        }

        // Check required blocks
        if !self.has_required_blocks() {
            return Err(ParseError::MissingBlock);
        }

        Ok(())
    }

    /// Check if all required blocks are present
    fn has_required_blocks(&self) -> bool {
        let mut has = [false; 5]; // Ω, Σ, Γ, Λ, Ε

        for i in 0..self.block_count as usize {
            match self.blocks[i].block_type {
                BlockType::Omega => has[0] = true,
                BlockType::Sigma => has[1] = true,
                BlockType::Gamma => has[2] = true,
                BlockType::Lambda => has[3] = true,
                BlockType::Evidence => has[4] = true,
                _ => {}
            }
        }

        has.iter().all(|&b| b)
    }

    /// Skip to first block delimiter
    fn skip_to_block(&mut self) {
        while self.pos < self.input.len() {
            if self.input[self.pos..].starts_with("⟦") {
                return;
            }
            self.pos += 1;
        }
    }

    /// Try to parse a block at current position
    fn try_parse_block(&mut self) -> Option<BlockInfo> {
        let remaining = &self.input[self.pos..];

        // Must start with ⟦
        if !remaining.starts_with("⟦") {
            return None;
        }

        // Find block type character
        let block_start = self.pos + "⟦".len();
        let type_char = self.input[block_start..].chars().next()?;
        let block_type = BlockType::from_char(type_char);

        if block_type == BlockType::Unknown {
            return None;
        }

        // Find opening brace
        let body_start = self.input[self.pos..].find('{')? + self.pos + 1;

        // Find matching closing brace (simple, no nesting)
        let body_end = self.input[body_start..].find('}')? + body_start;

        let block = BlockInfo {
            block_type,
            name_start: block_start as u16,
            name_len: (body_start - block_start - 1) as u16,
            body_start: body_start as u16,
            body_end: body_end as u16,
        };

        self.pos = body_end + 1;
        Some(block)
    }

    /// Get parsed blocks
    pub fn blocks(&self) -> &[BlockInfo] {
        &self.blocks[..self.block_count as usize]
    }
}

/// Count AISP symbols in input
pub fn count_aisp_symbols(input: &str) -> (u16, u16) {
    let mut aisp = 0u16;
    let mut total = 0u16;
    let mut chars = input.chars().peekable();

    while chars.peek().is_some() {
        let c = chars.next().unwrap();
        total = total.saturating_add(1);

        // Check if this is an AISP symbol
        if is_core_aisp_char(c) {
            aisp = aisp.saturating_add(1);
        }
    }

    (aisp, total)
}

/// Check if character is a core AISP glyph
#[inline]
fn is_core_aisp_char(c: char) -> bool {
    matches!(c,
        // Logic
        '⊤' | '⊥' | '∧' | '∨' | '¬' | '→' | '↔' | '⇒' | '⊢' | '⊨' |
        // Definition
        '≜' | '≔' | '≡' | '∎' |
        // Quantifiers
        '∀' | '∃' | 'λ' | 'Π' | 'Σ' |
        // Sets
        '∈' | '⊆' | '∩' | '∪' | '∅' |
        // Operators
        '⊕' | '⊖' | '⊗' | '∘' | '↦' |
        // Delimiters
        '⟦' | '⟧' | '⟨' | '⟩' | '◊' |
        // Domains
        'ℕ' | 'ℤ' | 'ℝ' | 'ℚ' | '𝔹' | '𝕊' |
        // Greek (blocks)
        'Ω' | 'Γ' | 'Λ' | 'Χ' | 'Ε' | 'Θ' | 'ℭ' |
        // Other AISP
        '𝔸' | '𝒫' | 'ψ' | 'δ' | 'φ' | 'τ' | 'ε' | 'μ'
    )
}
