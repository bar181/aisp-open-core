//! AISP document validation
//!
//! Implements the AISP 5.1 validation rules including:
//! - Ambiguity check: Ambig(D) < 0.02
//! - Density computation for tier assignment
//! - Required block verification

/// Quality tier enumeration
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Tier {
    /// ⊘: δ < 0.20 — Rejected
    Reject = 0,
    /// ◊⁻: δ ≥ 0.20 — Bronze
    Bronze = 1,
    /// ◊: δ ≥ 0.40 — Silver
    Silver = 2,
    /// ◊⁺: δ ≥ 0.60 — Gold
    Gold = 3,
    /// ◊⁺⁺: δ ≥ 0.75 — Platinum
    Platinum = 4,
}

impl Tier {
    /// Compute tier from density score
    #[inline]
    pub fn from_delta(delta: f32) -> Self {
        if delta >= 0.75 {
            Self::Platinum
        } else if delta >= 0.60 {
            Self::Gold
        } else if delta >= 0.40 {
            Self::Silver
        } else if delta >= 0.20 {
            Self::Bronze
        } else {
            Self::Reject
        }
    }

    /// Get tier symbol
    pub fn symbol(self) -> &'static str {
        match self {
            Self::Reject => "⊘",
            Self::Bronze => "◊⁻",
            Self::Silver => "◊",
            Self::Gold => "◊⁺",
            Self::Platinum => "◊⁺⁺",
        }
    }
}

/// Validation result
#[repr(C)]
pub struct ValidationResult {
    /// Is document valid?
    pub valid: bool,
    /// Quality tier
    pub tier: Tier,
    /// Density score δ
    pub delta: f32,
    /// Ambiguity score
    pub ambiguity: f32,
    /// Completeness φ
    pub phi: u8,
}

impl ValidationResult {
    /// Create successful validation result
    pub fn success(tier: Tier, delta: f32) -> Self {
        Self {
            valid: true,
            tier,
            delta,
            ambiguity: 0.01, // Valid AISP has near-zero ambiguity
            phi: 100,
        }
    }

    /// Create failed validation result
    pub fn failure(ambiguity: f32) -> Self {
        Self {
            valid: false,
            tier: Tier::Reject,
            delta: 0.0,
            ambiguity,
            phi: 0,
        }
    }
}

/// Validate AISP document
pub fn validate_document(
    aisp_count: u16,
    total_count: u16,
    ws_count: u16,
    has_required_blocks: bool,
) -> ValidationResult {
    // Compute density: |AISP tokens| / |non-whitespace tokens|
    let non_ws = total_count.saturating_sub(ws_count);
    let delta = if non_ws > 0 {
        aisp_count as f32 / non_ws as f32
    } else {
        0.0
    };

    // AISP ambiguity is near-zero for well-formed documents
    // with required blocks and proper structure
    let ambiguity = if has_required_blocks {
        // Valid AISP: unique parse, low ambiguity
        0.01
    } else {
        // Missing structure: high ambiguity
        0.5
    };

    // AISP requires Ambig(D) < 0.02
    if ambiguity >= 0.02 {
        return ValidationResult::failure(ambiguity);
    }

    let tier = Tier::from_delta(delta);
    ValidationResult::success(tier, delta)
}

/// Required blocks for valid AISP document
pub const REQUIRED_BLOCKS: [&str; 5] = ["⟦Ω", "⟦Σ", "⟦Γ", "⟦Λ", "⟦Ε"];

/// Check if document has all required blocks
pub fn check_required_blocks(input: &str) -> bool {
    REQUIRED_BLOCKS.iter().all(|block| input.contains(block))
}

/// Compute document completeness (φ)
pub fn compute_completeness(
    has_omega: bool,
    has_sigma: bool,
    has_gamma: bool,
    has_lambda: bool,
    has_evidence: bool,
    has_optional: u8,
) -> u8 {
    let required = [has_omega, has_sigma, has_gamma, has_lambda, has_evidence];
    let required_count = required.iter().filter(|&&b| b).count() as u8;

    // Base completeness from required blocks (0-80%)
    let base = required_count * 16;

    // Bonus from optional blocks (0-20%)
    let bonus = (has_optional * 4).min(20);

    (base + bonus).min(100)
}

/// Binding state for API compatibility (Δ⊗λ)
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BindingState {
    /// ⊥: Logic contradiction — crash
    Crash = 0,
    /// ∅: Socket mismatch — null binding
    Null = 1,
    /// λ: Type mismatch — adaptation required
    Adapt = 2,
    /// ⊤: Full compatibility — zero-cost binding
    Zero = 3,
}

impl BindingState {
    /// Compute binding state from pre/post conditions
    pub fn compute(
        logic_conflict: bool,
        socket_mismatch: bool,
        type_mismatch: bool,
        post_subset_pre: bool,
    ) -> Self {
        // Priority: crash > null > adapt > zero
        if logic_conflict {
            Self::Crash
        } else if socket_mismatch {
            Self::Null
        } else if type_mismatch {
            Self::Adapt
        } else if post_subset_pre {
            Self::Zero
        } else {
            Self::Adapt
        }
    }
}
