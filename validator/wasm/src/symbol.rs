//! Symbol interning for AISP Σ_512 glossary
//!
//! Provides efficient symbol lookup and category classification
//! for the 512 AISP symbols.

/// Symbol identifier (2 bytes)
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct SymbolId(pub u16);

impl SymbolId {
    pub const ANONYMOUS: Self = Self(0xFFFF);

    #[inline]
    pub const fn new(id: u16) -> Self {
        Self(id)
    }

    #[inline]
    pub const fn is_anonymous(self) -> bool {
        self.0 == 0xFFFF
    }
}

/// Symbol category (AISP Σ_512)
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq)]
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
    pub fn from_id(id: SymbolId) -> Self {
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

/// Core AISP symbols (subset for size optimization)
/// Full Σ_512 would be loaded from external data
pub static AISP_SYMBOLS: &[Symbol] = &[
    // Ω: Transmuters
    Symbol { glyph: *b"\xE2\x8A\xA4\x00", len: 3, category: Category::Omega, id: 0 },  // ⊤
    Symbol { glyph: *b"\xE2\x8A\xA5\x00", len: 3, category: Category::Omega, id: 1 },  // ⊥
    Symbol { glyph: *b"\xE2\x88\xA7\x00", len: 3, category: Category::Omega, id: 2 },  // ∧
    Symbol { glyph: *b"\xE2\x88\xA8\x00", len: 3, category: Category::Omega, id: 3 },  // ∨
    Symbol { glyph: *b"\xC2\xAC\x00\x00", len: 2, category: Category::Omega, id: 4 },  // ¬
    Symbol { glyph: *b"\xE2\x86\x92\x00", len: 3, category: Category::Omega, id: 5 },  // →
    Symbol { glyph: *b"\xE2\x86\x94\x00", len: 3, category: Category::Omega, id: 6 },  // ↔
    Symbol { glyph: *b"\xE2\x87\x92\x00", len: 3, category: Category::Omega, id: 7 },  // ⇒
    Symbol { glyph: *b"\xE2\x8A\xA2\x00", len: 3, category: Category::Omega, id: 8 },  // ⊢
    Symbol { glyph: *b"\xE2\x8A\xA8\x00", len: 3, category: Category::Omega, id: 9 },  // ⊨
    Symbol { glyph: *b"\xE2\x89\x9C\x00", len: 3, category: Category::Omega, id: 10 }, // ≜
    Symbol { glyph: *b"\xE2\x89\x94\x00", len: 3, category: Category::Omega, id: 11 }, // ≔
    Symbol { glyph: *b"\xCE\xBB\x00\x00", len: 2, category: Category::Omega, id: 12 }, // λ
    Symbol { glyph: *b"\xE2\x88\x8E\x00", len: 3, category: Category::Omega, id: 13 }, // ∎

    // Γ: Topologics
    Symbol { glyph: *b"\xE2\x88\x88\x00", len: 3, category: Category::Gamma, id: 64 }, // ∈
    Symbol { glyph: *b"\xE2\x8A\x86\x00", len: 3, category: Category::Gamma, id: 65 }, // ⊆
    Symbol { glyph: *b"\xE2\x88\xA9\x00", len: 3, category: Category::Gamma, id: 66 }, // ∩
    Symbol { glyph: *b"\xE2\x88\xAA\x00", len: 3, category: Category::Gamma, id: 67 }, // ∪
    Symbol { glyph: *b"\xE2\x88\x85\x00", len: 3, category: Category::Gamma, id: 68 }, // ∅

    // ∀: Quantifiers
    Symbol { glyph: *b"\xE2\x88\x80\x00", len: 3, category: Category::Forall, id: 128 }, // ∀
    Symbol { glyph: *b"\xE2\x88\x83\x00", len: 3, category: Category::Forall, id: 129 }, // ∃
    Symbol { glyph: *b"\xCE\xA0\x00\x00", len: 2, category: Category::Forall, id: 130 }, // Π
    Symbol { glyph: *b"\xCE\xA3\x00\x00", len: 2, category: Category::Forall, id: 131 }, // Σ
    Symbol { glyph: *b"\xE2\x8A\x95\x00", len: 3, category: Category::Forall, id: 132 }, // ⊕
    Symbol { glyph: *b"\xE2\x8A\x96\x00", len: 3, category: Category::Forall, id: 133 }, // ⊖
    Symbol { glyph: *b"\xE2\x8A\x97\x00", len: 3, category: Category::Forall, id: 134 }, // ⊗
    Symbol { glyph: *b"\xE2\x97\x8A\x00", len: 3, category: Category::Forall, id: 135 }, // ◊

    // 𝔻: Domaines
    Symbol { glyph: *b"\xE2\x84\x95\x00", len: 3, category: Category::Domain, id: 256 }, // ℕ
    Symbol { glyph: *b"\xE2\x84\xA4\x00", len: 3, category: Category::Domain, id: 257 }, // ℤ
    Symbol { glyph: *b"\xE2\x84\x9D\x00", len: 3, category: Category::Domain, id: 258 }, // ℝ
    Symbol { glyph: *b"\xE2\x84\x9A\x00", len: 3, category: Category::Domain, id: 259 }, // ℚ
    Symbol { glyph: *b"\xF0\x9D\x94\xB9", len: 4, category: Category::Domain, id: 260 }, // 𝔹
    Symbol { glyph: *b"\xF0\x9D\x95\x8A", len: 4, category: Category::Domain, id: 261 }, // 𝕊

    // ⟦⟧: Delimiters
    Symbol { glyph: *b"\xE2\x9F\xA6\x00", len: 3, category: Category::Block, id: 384 }, // ⟦
    Symbol { glyph: *b"\xE2\x9F\xA7\x00", len: 3, category: Category::Block, id: 385 }, // ⟧
    Symbol { glyph: *b"\xE2\x9F\xA8\x00", len: 3, category: Category::Block, id: 386 }, // ⟨
    Symbol { glyph: *b"\xE2\x9F\xA9\x00", len: 3, category: Category::Block, id: 387 }, // ⟩
    Symbol { glyph: *b"\xF0\x9D\x94\xB8", len: 4, category: Category::Block, id: 388 }, // 𝔸

    // Greek letters used in AISP
    Symbol { glyph: *b"\xCE\xA9\x00\x00", len: 2, category: Category::Block, id: 389 }, // Ω
    Symbol { glyph: *b"\xCE\x93\x00\x00", len: 2, category: Category::Block, id: 390 }, // Γ
    Symbol { glyph: *b"\xCE\x9B\x00\x00", len: 2, category: Category::Block, id: 391 }, // Λ
    Symbol { glyph: *b"\xCE\xA7\x00\x00", len: 2, category: Category::Block, id: 392 }, // Χ
    Symbol { glyph: *b"\xCE\x95\x00\x00", len: 2, category: Category::Block, id: 393 }, // Ε
    Symbol { glyph: *b"\xCE\x98\x00\x00", len: 2, category: Category::Block, id: 394 }, // Θ
];

/// Look up symbol by glyph
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

// Compile-time size assertions
const _: () = assert!(core::mem::size_of::<SymbolId>() == 2);
const _: () = assert!(core::mem::size_of::<Symbol>() == 8);
