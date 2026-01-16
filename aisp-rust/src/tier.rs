//! Quality tier definitions for AISP documents
//!
//! Tiers represent the quality level of an AISP specification based on
//! its semantic density (δ).

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Quality tier enumeration
///
/// Tiers are assigned based on semantic density (δ):
/// - **⊘ Reject**: δ < 0.20 — Document is too sparse
/// - **◊⁻ Bronze**: δ ≥ 0.20 — Draft/review quality
/// - **◊ Silver**: δ ≥ 0.40 — Development/testing
/// - **◊⁺ Gold**: δ ≥ 0.60 — Staging/pre-production
/// - **◊⁺⁺ Platinum**: δ ≥ 0.75 — Production deployment
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
    /// Compute tier from semantic density score
    ///
    /// # Arguments
    /// * `delta` - Semantic density δ ∈ [0, 1]
    ///
    /// # Example
    /// ```rust
    /// use aisp::Tier;
    ///
    /// assert_eq!(Tier::from_delta(0.80), Tier::Platinum);
    /// assert_eq!(Tier::from_delta(0.65), Tier::Gold);
    /// assert_eq!(Tier::from_delta(0.45), Tier::Silver);
    /// assert_eq!(Tier::from_delta(0.25), Tier::Bronze);
    /// assert_eq!(Tier::from_delta(0.10), Tier::Reject);
    /// ```
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

    /// Get the Unicode tier symbol
    ///
    /// # Example
    /// ```rust
    /// use aisp::Tier;
    ///
    /// assert_eq!(Tier::Platinum.symbol(), "◊⁺⁺");
    /// assert_eq!(Tier::Reject.symbol(), "⊘");
    /// ```
    pub const fn symbol(self) -> &'static str {
        match self {
            Self::Reject => "⊘",
            Self::Bronze => "◊⁻",
            Self::Silver => "◊",
            Self::Gold => "◊⁺",
            Self::Platinum => "◊⁺⁺",
        }
    }

    /// Get the tier name
    ///
    /// # Example
    /// ```rust
    /// use aisp::Tier;
    ///
    /// assert_eq!(Tier::Platinum.name(), "Platinum");
    /// ```
    pub const fn name(self) -> &'static str {
        match self {
            Self::Reject => "Reject",
            Self::Bronze => "Bronze",
            Self::Silver => "Silver",
            Self::Gold => "Gold",
            Self::Platinum => "Platinum",
        }
    }

    /// Get the minimum delta threshold for this tier
    pub const fn threshold(self) -> f32 {
        match self {
            Self::Reject => 0.0,
            Self::Bronze => 0.20,
            Self::Silver => 0.40,
            Self::Gold => 0.60,
            Self::Platinum => 0.75,
        }
    }

    /// Check if tier meets minimum production requirement (Gold or better)
    pub const fn is_production_ready(self) -> bool {
        matches!(self, Self::Gold | Self::Platinum)
    }
}

impl Default for Tier {
    fn default() -> Self {
        Self::Reject
    }
}

impl core::fmt::Display for Tier {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} {}", self.symbol(), self.name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tier_thresholds() {
        assert_eq!(Tier::from_delta(0.75), Tier::Platinum);
        assert_eq!(Tier::from_delta(0.74), Tier::Gold);
        assert_eq!(Tier::from_delta(0.60), Tier::Gold);
        assert_eq!(Tier::from_delta(0.59), Tier::Silver);
        assert_eq!(Tier::from_delta(0.40), Tier::Silver);
        assert_eq!(Tier::from_delta(0.39), Tier::Bronze);
        assert_eq!(Tier::from_delta(0.20), Tier::Bronze);
        assert_eq!(Tier::from_delta(0.19), Tier::Reject);
    }

    #[test]
    fn test_tier_ordering() {
        assert!(Tier::Platinum > Tier::Gold);
        assert!(Tier::Gold > Tier::Silver);
        assert!(Tier::Silver > Tier::Bronze);
        assert!(Tier::Bronze > Tier::Reject);
    }
}
