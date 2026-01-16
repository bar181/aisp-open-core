//! AISP Symbol definitions (Σ_512 glossary subset)
//!
//! Provides efficient symbol lookup and category classification
//! for core AISP symbols.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Symbol identifier (2 bytes)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct SymbolId(pub u16);

impl SymbolId {
    /// Anonymous/unknown symbol
    pub const ANONYMOUS: Self = Self(0xFFFF);

    /// Create a new symbol ID
    #[inline]
    pub const fn new(id: u16) -> Self {
        Self(id)
    }

    /// Check if this is an anonymous symbol
    #[inline]
    pub const fn is_anonymous(self) -> bool {
        self.0 == 0xFFFF
    }
}

/// Symbol category (AISP Σ_512)
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Category {
    /// Ω: Transmuters [0-63] — transform, derive, prove
    Omega = 0,
    /// Γ: Topologics [64-127] — structure, shape, relation
    Gamma = 1,
    /// ∀: Quantifiers [128-191] — scope, range, extent
    Forall = 2,
    /// Δ: Contractors [192-255] — binding, state, contract
    Delta = 3,
    /// 𝔻: Domaines [256-319] — type domains
    Domain = 4,
    /// Ψ: Intents [320-383] — intent, scoring
    Psi = 5,
    /// ⟦⟧: Delimiters [384-447] — blocks, structure
    Block = 6,
    /// ∅: Reserved [448-511] — operators
    Reserved = 7,
}

impl Category {
    /// Get category from symbol ID
    pub const fn from_id(id: SymbolId) -> Self {
        match id.0 {
            0..=63 => Self::Omega,
            64..=127 => Self::Gamma,
            128..=191 => Self::Forall,
            192..=255 => Self::Delta,
            256..=319 => Self::Domain,
            320..=383 => Self::Psi,
            384..=447 => Self::Block,
            _ => Self::Reserved,
        }
    }
}

/// Symbol entry in the glossary
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Symbol {
    /// UTF-8 encoded glyph (max 4 bytes)
    pub glyph: [u8; 4],
    /// Byte length of glyph
    pub len: u8,
    /// Category
    pub category: Category,
    /// Symbol ID
    pub id: u16,
}

/// Core AISP symbols (subset of Σ_512 for size optimization)
pub static AISP_SYMBOLS: &[Symbol] = &[
    // Ω: Transmuters - logic and proof
    Symbol { glyph: *b"\xE2\x8A\xA4\x00", len: 3, category: Category::Omega, id: 0 },   // ⊤ (top/true)
    Symbol { glyph: *b"\xE2\x8A\xA5\x00", len: 3, category: Category::Omega, id: 1 },   // ⊥ (bottom/false)
    Symbol { glyph: *b"\xE2\x88\xA7\x00", len: 3, category: Category::Omega, id: 2 },   // ∧ (and)
    Symbol { glyph: *b"\xE2\x88\xA8\x00", len: 3, category: Category::Omega, id: 3 },   // ∨ (or)
    Symbol { glyph: *b"\xC2\xAC\x00\x00", len: 2, category: Category::Omega, id: 4 },   // ¬ (not)
    Symbol { glyph: *b"\xE2\x86\x92\x00", len: 3, category: Category::Omega, id: 5 },   // → (implies)
    Symbol { glyph: *b"\xE2\x86\x94\x00", len: 3, category: Category::Omega, id: 6 },   // ↔ (iff)
    Symbol { glyph: *b"\xE2\x87\x92\x00", len: 3, category: Category::Omega, id: 7 },   // ⇒ (implies)
    Symbol { glyph: *b"\xE2\x87\x94\x00", len: 3, category: Category::Omega, id: 8 },   // ⇔ (iff)
    Symbol { glyph: *b"\xE2\x8A\xA2\x00", len: 3, category: Category::Omega, id: 9 },   // ⊢ (proves)
    Symbol { glyph: *b"\xE2\x8A\xA8\x00", len: 3, category: Category::Omega, id: 10 },  // ⊨ (models)
    Symbol { glyph: *b"\xE2\x89\x9C\x00", len: 3, category: Category::Omega, id: 11 },  // ≜ (defined as)
    Symbol { glyph: *b"\xE2\x89\x94\x00", len: 3, category: Category::Omega, id: 12 },  // ≔ (assign)
    Symbol { glyph: *b"\xE2\x89\xA1\x00", len: 3, category: Category::Omega, id: 13 },  // ≡ (identical)
    Symbol { glyph: *b"\xE2\x89\xA2\x00", len: 3, category: Category::Omega, id: 14 },  // ≢ (not identical)
    Symbol { glyph: *b"\xCE\xBB\x00\x00", len: 2, category: Category::Omega, id: 15 },  // λ (lambda)
    Symbol { glyph: *b"\xE2\x88\x8E\x00", len: 3, category: Category::Omega, id: 16 },  // ∎ (QED)
    Symbol { glyph: *b"\xE2\x88\x98\x00", len: 3, category: Category::Omega, id: 17 },  // ∘ (compose)
    Symbol { glyph: *b"\xE2\x86\xA6\x00", len: 3, category: Category::Omega, id: 18 },  // ↦ (mapsto)

    // Γ: Topologics - sets and relations
    Symbol { glyph: *b"\xE2\x88\x88\x00", len: 3, category: Category::Gamma, id: 64 },  // ∈ (element of)
    Symbol { glyph: *b"\xE2\x88\x89\x00", len: 3, category: Category::Gamma, id: 65 },  // ∉ (not element)
    Symbol { glyph: *b"\xE2\x8A\x86\x00", len: 3, category: Category::Gamma, id: 66 },  // ⊆ (subset)
    Symbol { glyph: *b"\xE2\x8A\x87\x00", len: 3, category: Category::Gamma, id: 67 },  // ⊇ (superset)
    Symbol { glyph: *b"\xE2\x88\xA9\x00", len: 3, category: Category::Gamma, id: 68 },  // ∩ (intersection)
    Symbol { glyph: *b"\xE2\x88\xAA\x00", len: 3, category: Category::Gamma, id: 69 },  // ∪ (union)
    Symbol { glyph: *b"\xE2\x88\x85\x00", len: 3, category: Category::Gamma, id: 70 },  // ∅ (empty set)
    Symbol { glyph: *b"\xF0\x9D\x92\xAB", len: 4, category: Category::Gamma, id: 71 },  // 𝒫 (power set)

    // ∀: Quantifiers
    Symbol { glyph: *b"\xE2\x88\x80\x00", len: 3, category: Category::Forall, id: 128 }, // ∀ (for all)
    Symbol { glyph: *b"\xE2\x88\x83\x00", len: 3, category: Category::Forall, id: 129 }, // ∃ (exists)
    Symbol { glyph: *b"\xCE\xA0\x00\x00", len: 2, category: Category::Forall, id: 130 }, // Π (product)
    Symbol { glyph: *b"\xCE\xA3\x00\x00", len: 2, category: Category::Forall, id: 131 }, // Σ (sum)
    Symbol { glyph: *b"\xE2\x8A\x95\x00", len: 3, category: Category::Forall, id: 132 }, // ⊕ (plus)
    Symbol { glyph: *b"\xE2\x8A\x96\x00", len: 3, category: Category::Forall, id: 133 }, // ⊖ (minus)
    Symbol { glyph: *b"\xE2\x8A\x97\x00", len: 3, category: Category::Forall, id: 134 }, // ⊗ (tensor)
    Symbol { glyph: *b"\xE2\x97\x8A\x00", len: 3, category: Category::Forall, id: 135 }, // ◊ (tier)

    // 𝔻: Domaines - type domains
    Symbol { glyph: *b"\xE2\x84\x95\x00", len: 3, category: Category::Domain, id: 256 }, // ℕ (naturals)
    Symbol { glyph: *b"\xE2\x84\xA4\x00", len: 3, category: Category::Domain, id: 257 }, // ℤ (integers)
    Symbol { glyph: *b"\xE2\x84\x9D\x00", len: 3, category: Category::Domain, id: 258 }, // ℝ (reals)
    Symbol { glyph: *b"\xE2\x84\x9A\x00", len: 3, category: Category::Domain, id: 259 }, // ℚ (rationals)
    Symbol { glyph: *b"\xF0\x9D\x94\xB9", len: 4, category: Category::Domain, id: 260 }, // 𝔹 (booleans)
    Symbol { glyph: *b"\xF0\x9D\x95\x8A", len: 4, category: Category::Domain, id: 261 }, // 𝕊 (strings)

    // ⟦⟧: Delimiters and blocks
    Symbol { glyph: *b"\xE2\x9F\xA6\x00", len: 3, category: Category::Block, id: 384 }, // ⟦ (block open)
    Symbol { glyph: *b"\xE2\x9F\xA7\x00", len: 3, category: Category::Block, id: 385 }, // ⟧ (block close)
    Symbol { glyph: *b"\xE2\x9F\xA8\x00", len: 3, category: Category::Block, id: 386 }, // ⟨ (tuple open)
    Symbol { glyph: *b"\xE2\x9F\xA9\x00", len: 3, category: Category::Block, id: 387 }, // ⟩ (tuple close)
    Symbol { glyph: *b"\xF0\x9D\x94\xB8", len: 4, category: Category::Block, id: 388 }, // 𝔸 (AISP header)

    // Greek block labels
    Symbol { glyph: *b"\xCE\xA9\x00\x00", len: 2, category: Category::Block, id: 389 }, // Ω (meta)
    Symbol { glyph: *b"\xCE\x93\x00\x00", len: 2, category: Category::Block, id: 390 }, // Γ (rules)
    Symbol { glyph: *b"\xCE\x9B\x00\x00", len: 2, category: Category::Block, id: 391 }, // Λ (funcs)
    Symbol { glyph: *b"\xCE\xA7\x00\x00", len: 2, category: Category::Block, id: 392 }, // Χ (errors)
    Symbol { glyph: *b"\xCE\x95\x00\x00", len: 2, category: Category::Block, id: 393 }, // Ε (evidence)
    Symbol { glyph: *b"\xCE\x98\x00\x00", len: 2, category: Category::Block, id: 394 }, // Θ (theorems)
];

/// Check if a character is an AISP symbol
pub fn is_aisp_char(c: char) -> bool {
    matches!(c,
        '≜' | '≔' | '≡' | '≢' | '⇒' | '⇔' | '↔' | '⊢' | '⊨' | '∎' |
        '∀' | '∃' | 'λ' | 'Π' | 'Σ' |
        '∈' | '∉' | '⊆' | '⊇' | '∩' | '∪' | '∅' |
        '⊕' | '⊖' | '⊗' | '∘' | '→' | '↦' |
        '⟨' | '⟩' | '⟦' | '⟧' | '◊' | '𝔸' |
        '⊤' | '⊥' | '¬' | '∧' | '∨' |
        'ℕ' | 'ℤ' | 'ℝ' | 'ℚ' | '𝔹' | '𝕊' |
        '𝒫' | 'ψ' | 'δ' | 'φ' | 'τ' | 'ε' | 'γ' | 'ρ' |
        'Ω' | 'Γ' | 'Λ' | 'Χ' | 'Ε' | 'Θ' | 'ℭ'
    )
}

/// Count AISP symbols in a string
pub fn count_symbols(s: &str) -> usize {
    s.chars().filter(|&c| is_aisp_char(c)).count()
}

/// Count non-whitespace tokens in a string
pub fn count_tokens(s: &str) -> usize {
    s.split_whitespace().count()
}

/// Look up symbol by glyph string
pub fn lookup_symbol(glyph: &str) -> Option<SymbolId> {
    let bytes = glyph.as_bytes();
    for sym in AISP_SYMBOLS.iter() {
        let len = sym.len as usize;
        if len == bytes.len() && &sym.glyph[..len] == bytes {
            return Some(SymbolId::new(sym.id));
        }
    }
    None
}

/// Check if string starts with an AISP symbol
/// Returns (SymbolId, byte length) if found
pub fn starts_with_symbol(s: &str) -> Option<(SymbolId, usize)> {
    // Try 4-byte, then 3-byte, then 2-byte symbols
    for len in (2..=4).rev() {
        if s.len() >= len {
            if let Some(id) = lookup_symbol(&s[..len]) {
                return Some((id, len));
            }
        }
    }
    None
}

/// Get the glyph string for a symbol ID
pub fn get_glyph(id: SymbolId) -> Option<&'static str> {
    for sym in AISP_SYMBOLS.iter() {
        if sym.id == id.0 {
            let len = sym.len as usize;
            // SAFETY: We know these are valid UTF-8 from construction
            return core::str::from_utf8(&sym.glyph[..len]).ok();
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_aisp_char() {
        assert!(is_aisp_char('∀'));
        assert!(is_aisp_char('λ'));
        assert!(is_aisp_char('≜'));
        assert!(is_aisp_char('⟦'));
        assert!(!is_aisp_char('a'));
        assert!(!is_aisp_char('1'));
    }

    #[test]
    fn test_count_symbols() {
        assert_eq!(count_symbols("∀x:x≥0"), 1); // only ∀ (≥ is not in AISP list)
        assert_eq!(count_symbols("f≜λx.x"), 2); // ≜ and λ
    }

    #[test]
    fn test_lookup_symbol() {
        assert!(lookup_symbol("∀").is_some());
        assert!(lookup_symbol("λ").is_some());
        assert!(lookup_symbol("𝔸").is_some());
        assert!(lookup_symbol("x").is_none());
    }

    #[test]
    fn test_starts_with_symbol() {
        let (id, len) = starts_with_symbol("∀x:P(x)").unwrap();
        assert_eq!(len, 3); // ∀ is 3 bytes
        assert_eq!(Category::from_id(id), Category::Forall);
    }

    #[test]
    fn test_get_glyph() {
        let id = lookup_symbol("λ").unwrap();
        assert_eq!(get_glyph(id), Some("λ"));
    }
}
