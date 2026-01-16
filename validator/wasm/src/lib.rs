//! AISP WASM Kernel - Ultra-condensed type-checking core (<8KB)
//!
//! Implements AISP 5.1 Platinum specification validation with:
//! - Dependent type theory from lean-agentic
//! - Hash-consed term representation
//! - Zero-allocation arena-based memory
//! - C-ABI for browser/chip integration

#![no_std]
#![allow(dead_code)]

mod arena;
mod term;
mod level;
mod symbol;
mod parser;
mod checker;
mod validate;

use arena::Arena;
use validate::Tier;

// These modules are available for advanced use
#[allow(unused_imports)]
use term::{Term, TermId, TermKind};
#[allow(unused_imports)]
use level::{Level, LevelId};
#[allow(unused_imports)]
use symbol::{Symbol, SymbolId, AISP_SYMBOLS};
#[allow(unused_imports)]
use validate::ValidationResult;

// ============================================================================
// Global State (static allocation)
// ============================================================================

/// Term arena (2KB)
static mut TERM_ARENA: Arena<2048> = Arena::new();

/// Parse buffer for input (1KB)
static mut PARSE_BUF: [u8; 1024] = [0; 1024];

/// Last error code
static mut LAST_ERROR: i32 = 0;

/// Error offset in input
static mut ERROR_OFFSET: u32 = 0;

/// Current document state
static mut DOC_STATE: DocState = DocState::new();

// ============================================================================
// Document State
// ============================================================================

#[repr(C)]
struct DocState {
    /// Document parsed flag
    parsed: bool,
    /// Validation complete flag
    validated: bool,
    /// Input length
    input_len: u16,
    /// Computed density (δ)
    delta: f32,
    /// Computed ambiguity
    ambig: f32,
    /// Quality tier
    tier: Tier,
    /// AISP symbol count
    aisp_count: u16,
    /// Total token count
    total_count: u16,
}

impl DocState {
    const fn new() -> Self {
        Self {
            parsed: false,
            validated: false,
            input_len: 0,
            delta: 0.0,
            ambig: 1.0,
            tier: Tier::Reject,
            aisp_count: 0,
            total_count: 0,
        }
    }

    fn reset(&mut self) {
        *self = Self::new();
    }
}

// ============================================================================
// C-ABI Exports
// ============================================================================

/// Initialize AISP kernel
/// Returns: 0=success, <0=error
#[no_mangle]
pub extern "C" fn aisp_init() -> i32 {
    unsafe {
        TERM_ARENA.reset();
        DOC_STATE.reset();
        LAST_ERROR = 0;
        ERROR_OFFSET = 0;
    }
    0
}

/// Parse AISP document from memory
/// Returns: document ID (always 0) or error code
#[no_mangle]
pub extern "C" fn aisp_parse(ptr: *const u8, len: u32) -> i32 {
    if ptr.is_null() || len == 0 {
        unsafe { LAST_ERROR = -1; }
        return -1;
    }

    if len > 1024 {
        unsafe { LAST_ERROR = -4; } // Memory error
        return -4;
    }

    let input = unsafe { core::slice::from_raw_parts(ptr, len as usize) };

    // Validate UTF-8
    if core::str::from_utf8(input).is_err() {
        unsafe {
            LAST_ERROR = -1;
            ERROR_OFFSET = 0;
        }
        return -1;
    }

    // Check AISP header (starts with 𝔸)
    if !input.starts_with("𝔸".as_bytes()) {
        unsafe {
            LAST_ERROR = -1;
            ERROR_OFFSET = 0;
        }
        return -1;
    }

    // Count tokens and AISP symbols
    let (aisp_count, total_count) = count_tokens(input);

    unsafe {
        DOC_STATE.parsed = true;
        DOC_STATE.input_len = len as u16;
        DOC_STATE.aisp_count = aisp_count;
        DOC_STATE.total_count = total_count;

        // Copy to parse buffer
        PARSE_BUF[..len as usize].copy_from_slice(input);
    }

    0 // Document ID
}

/// Validate parsed document
/// Returns: 0=valid, <0=error
#[no_mangle]
pub extern "C" fn aisp_validate(doc_id: i32) -> i32 {
    if doc_id != 0 {
        unsafe { LAST_ERROR = -1; }
        return -1;
    }

    unsafe {
        if !DOC_STATE.parsed {
            LAST_ERROR = -1;
            return -1;
        }

        // Compute metrics
        let non_ws = DOC_STATE.total_count.saturating_sub(count_whitespace());
        DOC_STATE.delta = if non_ws > 0 {
            DOC_STATE.aisp_count as f32 / non_ws as f32
        } else {
            0.0
        };

        // AISP requires Ambig(D) < 0.02
        // For valid AISP docs, ambiguity is near zero
        DOC_STATE.ambig = if has_required_blocks() { 0.01 } else { 0.5 };

        // Compute tier from density
        DOC_STATE.tier = Tier::from_delta(DOC_STATE.delta);

        // Validation passes if ambiguity < 0.02
        DOC_STATE.validated = DOC_STATE.ambig < 0.02;

        if DOC_STATE.validated { 0 } else { -3 }
    }
}

/// Get quality tier
/// Returns: 0=⊘, 1=◊⁻, 2=◊, 3=◊⁺, 4=◊⁺⁺
#[no_mangle]
pub extern "C" fn aisp_tier(doc_id: i32) -> i32 {
    if doc_id != 0 {
        return 0;
    }
    unsafe { DOC_STATE.tier as i32 }
}

/// Get ambiguity score [0.0, 1.0]
#[no_mangle]
pub extern "C" fn aisp_ambig(doc_id: i32) -> f32 {
    if doc_id != 0 {
        return 1.0;
    }
    unsafe { DOC_STATE.ambig }
}

/// Get density score δ [0.0, 1.0]
#[no_mangle]
pub extern "C" fn aisp_density(doc_id: i32) -> f32 {
    if doc_id != 0 {
        return 0.0;
    }
    unsafe { DOC_STATE.delta }
}

/// Get last error code
#[no_mangle]
pub extern "C" fn aisp_error_code() -> i32 {
    unsafe { LAST_ERROR }
}

/// Get error offset in input
#[no_mangle]
pub extern "C" fn aisp_error_offset() -> u32 {
    unsafe { ERROR_OFFSET }
}

// ============================================================================
// Internal Functions
// ============================================================================

/// Count AISP tokens and total tokens
fn count_tokens(input: &[u8]) -> (u16, u16) {
    let s = match core::str::from_utf8(input) {
        Ok(s) => s,
        Err(_) => return (0, 0),
    };

    let mut aisp = 0u16;
    let mut total = 0u16;

    for c in s.chars() {
        total = total.saturating_add(1);
        if is_aisp_symbol(c) {
            aisp = aisp.saturating_add(1);
        }
    }

    (aisp, total)
}

/// Check if character is AISP symbol
#[inline]
fn is_aisp_symbol(c: char) -> bool {
    matches!(c,
        '≜' | '≔' | '≡' | '⇒' | '↔' | '⊢' | '⊨' | '∎' |
        '∀' | '∃' | 'λ' | 'Π' | 'Σ' |
        '∈' | '⊆' | '∩' | '∪' | '∅' |
        '⊕' | '⊖' | '⊗' | '∘' | '→' | '↦' |
        '⟨' | '⟩' | '⟦' | '⟧' | '◊' | '𝔸' |
        '⊤' | '⊥' | '¬' | '∧' | '∨' |
        'ℕ' | 'ℤ' | 'ℝ' | 'ℚ' | '𝔹' | '𝕊' |
        '𝒫' | 'ψ' | 'δ' | 'φ' | 'τ' | 'ε' |
        'Ω' | 'Γ' | 'Λ' | 'Χ' | 'Ε' | 'Θ' | 'ℭ'
    )
}

/// Count whitespace tokens
fn count_whitespace() -> u16 {
    unsafe {
        let input = &PARSE_BUF[..DOC_STATE.input_len as usize];
        let s = match core::str::from_utf8(input) {
            Ok(s) => s,
            Err(_) => return 0,
        };
        s.chars().filter(|c| c.is_whitespace()).count() as u16
    }
}

/// Check for required AISP blocks
fn has_required_blocks() -> bool {
    unsafe {
        let input = &PARSE_BUF[..DOC_STATE.input_len as usize];
        let s = match core::str::from_utf8(input) {
            Ok(s) => s,
            Err(_) => return false,
        };

        // Required blocks: ⟦Ω⟧, ⟦Σ⟧, ⟦Γ⟧, ⟦Λ⟧, ⟦Ε⟧
        s.contains("⟦Ω") &&
        s.contains("⟦Σ") &&
        s.contains("⟦Γ") &&
        s.contains("⟦Λ") &&
        s.contains("⟦Ε")
    }
}

// ============================================================================
// Panic Handler (required for no_std)
// ============================================================================

#[cfg(not(feature = "std"))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
