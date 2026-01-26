//! AISP symbol definitions and Unicode handling
//! 
//! This module provides efficient lookup and parsing of AISP's special
//! Unicode symbols with compile-time verification.

use std::collections::HashMap;
use std::sync::OnceLock;

/// AISP symbol categories for density calculation
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SymbolCategory {
    /// Block delimiters (⟦, ⟧)
    BlockDelimiter,
    /// Definition operators (≜, ≔, ≡, ≢)
    Definition,
    /// Quantifiers (∀, ∃)
    Quantifier,
    /// Lambda (λ)
    Lambda,
    /// Logical operators (⇒, ⇔, →, ↔, ∧, ∨, ¬, ⊕)
    Logic,
    /// Set operators (∈, ∉, ⊆, ⊇, ∩, ∪, ∅, 𝒫)
    Set,
    /// Relational operators (≤, ≥, <, >)
    Relation,
    /// Type symbols (ℕ, ℤ, ℝ, 𝔹, 𝕊)
    Type,
    /// Document header (𝔸)
    Document,
    /// Tier symbols (◊, ⊘)
    Tier,
    /// Tuple delimiters (⟨, ⟩)
    Tuple,
    /// Temporal operators (□, X, U, etc.)
    Temporal,
    /// Greek letters for variables
    Greek,
}

/// AISP symbol definition
#[derive(Debug, Clone)]
pub struct Symbol {
    pub char: char,
    pub category: SymbolCategory,
    pub name: &'static str,
    pub ascii_alt: Option<&'static str>,
}

impl Symbol {
    pub const fn new(
        char: char,
        category: SymbolCategory,
        name: &'static str,
        ascii_alt: Option<&'static str>,
    ) -> Self {
        Self {
            char,
            category,
            name,
            ascii_alt,
        }
    }
}

/// Complete AISP symbol set (Σ_512 subset)
pub static AISP_SYMBOLS: &[Symbol] = &[
    // Block delimiters
    Symbol::new('⟦', SymbolCategory::BlockDelimiter, "LEFT_DOUBLE_BRACKET", Some("((")),
    Symbol::new('⟧', SymbolCategory::BlockDelimiter, "RIGHT_DOUBLE_BRACKET", Some("))")),

    // Definition operators
    Symbol::new('≜', SymbolCategory::Definition, "DEFINED_AS", Some("::=")),
    Symbol::new('≔', SymbolCategory::Definition, "ASSIGNMENT", Some(":=")),
    Symbol::new('≡', SymbolCategory::Definition, "EQUIVALENT", Some("===")),
    Symbol::new('≢', SymbolCategory::Definition, "NOT_EQUIVALENT", Some("!==")),

    // Quantifiers
    Symbol::new('∀', SymbolCategory::Quantifier, "FOR_ALL", Some("forall")),
    Symbol::new('∃', SymbolCategory::Quantifier, "EXISTS", Some("exists")),

    // Lambda
    Symbol::new('λ', SymbolCategory::Lambda, "LAMBDA", Some("lambda")),

    // Logical operators
    Symbol::new('⇒', SymbolCategory::Logic, "IMPLIES", Some("=>")),
    Symbol::new('⇔', SymbolCategory::Logic, "IFF", Some("<=>")),
    Symbol::new('→', SymbolCategory::Logic, "ARROW", Some("->")),
    Symbol::new('↔', SymbolCategory::Logic, "BICONDITIONAL", Some("<->")),
    Symbol::new('∧', SymbolCategory::Logic, "AND", Some("/\\")),
    Symbol::new('∨', SymbolCategory::Logic, "OR", Some("\\/")),
    Symbol::new('¬', SymbolCategory::Logic, "NOT", Some("~")),
    Symbol::new('⊕', SymbolCategory::Logic, "XOR", Some("xor")),

    // Set operators
    Symbol::new('∈', SymbolCategory::Set, "ELEMENT_OF", Some("in")),
    Symbol::new('∉', SymbolCategory::Set, "NOT_ELEMENT_OF", Some("notin")),
    Symbol::new('⊆', SymbolCategory::Set, "SUBSET", Some("subset")),
    Symbol::new('⊇', SymbolCategory::Set, "SUPERSET", Some("superset")),
    Symbol::new('∩', SymbolCategory::Set, "INTERSECTION", Some("intersect")),
    Symbol::new('∪', SymbolCategory::Set, "UNION", Some("union")),
    Symbol::new('∅', SymbolCategory::Set, "EMPTY_SET", Some("emptyset")),
    Symbol::new('𝒫', SymbolCategory::Set, "POWER_SET", Some("powerset")),

    // Relational operators
    Symbol::new('≤', SymbolCategory::Relation, "LESS_EQUAL", Some("<=")),
    Symbol::new('≥', SymbolCategory::Relation, "GREATER_EQUAL", Some(">=")),

    // Type symbols
    Symbol::new('ℕ', SymbolCategory::Type, "NATURALS", Some("Nat")),
    Symbol::new('ℤ', SymbolCategory::Type, "INTEGERS", Some("Int")),
    Symbol::new('ℝ', SymbolCategory::Type, "REALS", Some("Real")),
    Symbol::new('𝔹', SymbolCategory::Type, "BOOLEANS", Some("Bool")),
    Symbol::new('𝕊', SymbolCategory::Type, "STRINGS", Some("String")),

    // Document header
    Symbol::new('𝔸', SymbolCategory::Document, "AISP_HEADER", Some("AISP")),

    // Tier symbols
    Symbol::new('◊', SymbolCategory::Tier, "DIAMOND", Some("diamond")),
    Symbol::new('⊘', SymbolCategory::Tier, "REJECT", Some("reject")),

    // Tuple delimiters
    Symbol::new('⟨', SymbolCategory::Tuple, "LEFT_ANGLE", Some("<")),
    Symbol::new('⟩', SymbolCategory::Tuple, "RIGHT_ANGLE", Some(">")),

    // Temporal operators
    Symbol::new('□', SymbolCategory::Temporal, "ALWAYS", Some("[]")),
    Symbol::new('X', SymbolCategory::Temporal, "NEXT", None),
    Symbol::new('U', SymbolCategory::Temporal, "UNTIL", None),

    // Common Greek letters
    Symbol::new('α', SymbolCategory::Greek, "ALPHA", Some("alpha")),
    Symbol::new('β', SymbolCategory::Greek, "BETA", Some("beta")),
    Symbol::new('γ', SymbolCategory::Greek, "GAMMA", Some("gamma")),
    Symbol::new('δ', SymbolCategory::Greek, "DELTA", Some("delta")),
    Symbol::new('ε', SymbolCategory::Greek, "EPSILON", Some("epsilon")),
    Symbol::new('φ', SymbolCategory::Greek, "PHI", Some("phi")),
    Symbol::new('τ', SymbolCategory::Greek, "TAU", Some("tau")),
    Symbol::new('ρ', SymbolCategory::Greek, "RHO", Some("rho")),
    Symbol::new('Ω', SymbolCategory::Greek, "OMEGA", Some("Omega")),
    Symbol::new('Σ', SymbolCategory::Greek, "SIGMA", Some("Sigma")),
    Symbol::new('Γ', SymbolCategory::Greek, "GAMMA_UPPER", Some("Gamma")),
    Symbol::new('Λ', SymbolCategory::Greek, "LAMBDA_UPPER", Some("Lambda")),
    Symbol::new('Ε', SymbolCategory::Greek, "EPSILON_UPPER", Some("Epsilon")),
    Symbol::new('Θ', SymbolCategory::Greek, "THETA", Some("Theta")),
    Symbol::new('Χ', SymbolCategory::Greek, "CHI", Some("Chi")),
    Symbol::new('Δ', SymbolCategory::Greek, "DELTA_UPPER", Some("Delta")),
    Symbol::new('Π', SymbolCategory::Greek, "PI", Some("Pi")),
];

/// Symbol lookup table for fast parsing
static SYMBOL_MAP: OnceLock<HashMap<char, &Symbol>> = OnceLock::new();

/// ASCII alternative lookup for compatibility
static ASCII_MAP: OnceLock<HashMap<&str, &Symbol>> = OnceLock::new();

/// Initialize symbol lookup tables
fn init_symbol_maps() -> (&'static HashMap<char, &'static Symbol>, &'static HashMap<&'static str, &'static Symbol>) {
    let symbol_map = SYMBOL_MAP.get_or_init(|| {
        AISP_SYMBOLS.iter().map(|s| (s.char, s)).collect()
    });
    
    let ascii_map = ASCII_MAP.get_or_init(|| {
        AISP_SYMBOLS
            .iter()
            .filter_map(|s| s.ascii_alt.map(|alt| (alt, s)))
            .collect()
    });
    
    (symbol_map, ascii_map)
}

/// Look up an AISP symbol by Unicode character
pub fn lookup_symbol(ch: char) -> Option<&'static Symbol> {
    let (symbol_map, _) = init_symbol_maps();
    symbol_map.get(&ch).copied()
}

/// Look up an AISP symbol by ASCII alternative
pub fn lookup_ascii(ascii: &str) -> Option<&'static Symbol> {
    let (_, ascii_map) = init_symbol_maps();
    ascii_map.get(ascii).copied()
}

/// Check if character is an AISP symbol
pub fn is_aisp_symbol(ch: char) -> bool {
    lookup_symbol(ch).is_some()
}

/// Get all symbols in a category
pub fn symbols_in_category(category: SymbolCategory) -> Vec<&'static Symbol> {
    AISP_SYMBOLS
        .iter()
        .filter(|s| s.category == category)
        .collect()
}

/// Calculate pure symbol density
pub fn calculate_symbol_density(text: &str) -> f64 {
    let total_chars = text.chars().filter(|c| !c.is_whitespace()).count();
    let symbol_count = text.chars().filter(|&c| is_aisp_symbol(c)).count();
    
    if total_chars == 0 {
        0.0
    } else {
        symbol_count as f64 / total_chars as f64
    }
}

/// Calculate weighted symbol density by category
pub fn calculate_weighted_density(text: &str) -> f64 {
    let mut category_counts = HashMap::new();
    let mut total_chars = 0;
    
    for ch in text.chars() {
        if !ch.is_whitespace() {
            total_chars += 1;
            if let Some(symbol) = lookup_symbol(ch) {
                *category_counts.entry(&symbol.category).or_insert(0) += 1;
            }
        }
    }
    
    if total_chars == 0 {
        return 0.0;
    }
    
    // Weight different symbol categories
    let mut weighted_score = 0.0;
    for (category, count) in category_counts {
        let weight = match category {
            SymbolCategory::Definition => 3.0,   // Definitions are highly semantic
            SymbolCategory::Logic => 2.5,        // Logic is core to AISP
            SymbolCategory::Quantifier => 2.5,   // Quantifiers are highly formal
            SymbolCategory::Lambda => 2.0,       // Functions are important
            SymbolCategory::Set => 2.0,          // Set operations are formal
            SymbolCategory::Temporal => 3.0,     // Temporal logic is advanced
            SymbolCategory::Type => 1.5,         // Types are structural
            SymbolCategory::Greek => 1.0,        // Variables are less semantic
            _ => 1.0,                             // Other symbols
        };
        weighted_score += count as f64 * weight;
    }
    
    weighted_score / total_chars as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_symbol_lookup() {
        assert!(lookup_symbol('≜').is_some());
        assert!(lookup_symbol('∀').is_some());
        assert!(lookup_symbol('x').is_none());
        
        let definition = lookup_symbol('≜').unwrap();
        assert_eq!(definition.name, "DEFINED_AS");
        assert_eq!(definition.ascii_alt, Some("::="));
    }

    #[test]
    fn test_ascii_lookup() {
        assert!(lookup_ascii("::=").is_some());
        assert!(lookup_ascii("forall").is_some());
        assert!(lookup_ascii("invalid").is_none());
        
        let forall = lookup_ascii("forall").unwrap();
        assert_eq!(forall.char, '∀');
    }

    #[test]
    fn test_is_aisp_symbol() {
        assert!(is_aisp_symbol('≜'));
        assert!(is_aisp_symbol('∀'));
        assert!(is_aisp_symbol('⇒'));
        assert!(!is_aisp_symbol('x'));
        assert!(!is_aisp_symbol(' '));
    }

    #[test]
    fn test_symbol_density() {
        let text = "≜∀⇒abc";
        let density = calculate_symbol_density(text);
        assert_eq!(density, 3.0 / 6.0); // 3 symbols out of 6 non-whitespace chars
    }

    #[test]
    fn test_weighted_density() {
        let text = "≜∀⇒"; // Definition + Quantifier + Logic
        let weighted = calculate_weighted_density(text);
        assert!(weighted > calculate_symbol_density(text)); // Weighted should be higher
    }

    #[test]
    fn test_symbols_in_category() {
        let logic_symbols = symbols_in_category(SymbolCategory::Logic);
        assert!(!logic_symbols.is_empty());
        assert!(logic_symbols.iter().any(|s| s.char == '⇒'));
        assert!(logic_symbols.iter().any(|s| s.char == '∧'));
    }
}