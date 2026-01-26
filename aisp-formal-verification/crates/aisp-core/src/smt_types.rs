//! SMT Types and Data Structures
//!
//! This module defines the core data structures for SMT-LIB
//! formula generation and Z3 integration.

use std::collections::HashMap;

/// SMT-LIB formula representation
#[derive(Debug, Clone, PartialEq)]
pub enum SmtFormula {
    /// Variable reference
    Variable(String),
    /// Integer literal
    IntLiteral(i64),
    /// Boolean literal
    BoolLiteral(bool),
    /// String literal
    StringLiteral(String),
    /// Function application
    Application(String, Vec<SmtFormula>),
    /// Logical conjunction
    And(Vec<SmtFormula>),
    /// Logical disjunction
    Or(Vec<SmtFormula>),
    /// Logical negation
    Not(Box<SmtFormula>),
    /// Universal quantification
    Forall(Vec<(String, SmtSort)>, Box<SmtFormula>),
    /// Existential quantification
    Exists(Vec<(String, SmtSort)>, Box<SmtFormula>),
}

/// SMT sort (type) representation
#[derive(Debug, Clone, PartialEq)]
pub enum SmtSort {
    /// Boolean sort
    Boolean,
    /// Integer sort
    Integer,
    /// Natural number sort
    Natural,
    /// Real number sort
    Real,
    /// String sort
    String,
    /// Custom sort
    Custom(String),
}

/// SMT command representation
#[derive(Debug, Clone, PartialEq)]
pub enum SmtCommand {
    /// Declare a sort
    DeclareSort(String, u32),
    /// Declare a function
    DeclareFun(String, Vec<SmtSort>, SmtSort),
    /// Assert a formula
    Assert(SmtFormula),
    /// Check satisfiability
    CheckSat,
    /// Get model
    GetModel,
    /// Set logic
    SetLogic(String),
    /// Set option
    SetOption(String, String),
}

/// Complete SMT-LIB program for Z3
#[derive(Debug, Clone)]
pub struct SMTProgram {
    /// SMT-LIB script content
    pub script: String,
    /// Property identifiers mapped to assertion names
    pub property_map: HashMap<String, String>,
    /// Check-sat commands for each property
    pub check_commands: Vec<String>,
    /// Expected results for verification
    pub expected_results: Vec<SMTExpectedResult>,
}

/// Expected result for SMT verification
#[derive(Debug, Clone, PartialEq)]
pub enum SMTExpectedResult {
    /// Property should be satisfiable
    Sat,
    /// Property should be unsatisfiable (valid)
    Unsat,
    /// Result is unknown/timeout acceptable
    Unknown,
}

/// SMT-LIB generation result
#[derive(Debug, Clone)]
pub struct SMTGenerationResult {
    /// Generated SMT program
    pub program: SMTProgram,
    /// Generation statistics
    pub stats: SMTGenerationStats,
    /// Warnings during generation
    pub warnings: Vec<String>,
}

/// Statistics about SMT generation
#[derive(Debug, Clone)]
pub struct SMTGenerationStats {
    /// Number of properties encoded
    pub properties_encoded: usize,
    /// Number of sort declarations
    pub sort_declarations: usize,
    /// Number of function declarations
    pub function_declarations: usize,
    /// Number of assertions generated
    pub assertions_generated: usize,
    /// Total script size in characters
    pub script_size: usize,
    /// Generation time in milliseconds
    pub generation_time_ms: u64,
}

impl SMTProgram {
    /// Create new empty SMT program
    pub fn new() -> Self {
        Self {
            script: String::new(),
            property_map: HashMap::new(),
            check_commands: Vec::new(),
            expected_results: Vec::new(),
        }
    }

    /// Add property mapping
    pub fn add_property(&mut self, property_id: String, assertion_name: String, expected: SMTExpectedResult) {
        self.property_map.insert(property_id, assertion_name.clone());
        self.check_commands.push(format!("(check-sat-assuming ({}))", assertion_name));
        self.expected_results.push(expected);
    }

    /// Get script size in bytes
    pub fn script_size(&self) -> usize {
        self.script.len()
    }

    /// Validate SMT program structure
    pub fn validate(&self) -> Result<(), String> {
        if self.script.is_empty() {
            return Err("Script is empty".to_string());
        }

        if self.property_map.len() != self.expected_results.len() {
            return Err("Mismatch between properties and expected results".to_string());
        }

        Ok(())
    }
}

impl SMTGenerationStats {
    /// Create new empty statistics
    pub fn new() -> Self {
        Self {
            properties_encoded: 0,
            sort_declarations: 0,
            function_declarations: 0,
            assertions_generated: 0,
            script_size: 0,
            generation_time_ms: 0,
        }
    }

    /// Calculate encoding efficiency (properties per KB)
    pub fn encoding_efficiency(&self) -> f64 {
        if self.script_size > 0 {
            (self.properties_encoded as f64) / (self.script_size as f64 / 1024.0)
        } else {
            0.0
        }
    }

    /// Get total declarations count
    pub fn total_declarations(&self) -> usize {
        self.sort_declarations + self.function_declarations
    }
}

impl Default for SMTProgram {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for SMTGenerationStats {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smt_program_creation() {
        let program = SMTProgram::new();
        assert!(program.script.is_empty());
        assert!(program.property_map.is_empty());
        assert!(program.check_commands.is_empty());
        assert!(program.expected_results.is_empty());
    }

    #[test]
    fn test_smt_program_add_property() {
        let mut program = SMTProgram::new();
        program.add_property(
            "prop_1".to_string(),
            "assertion_1".to_string(),
            SMTExpectedResult::Unsat,
        );

        assert_eq!(program.property_map.len(), 1);
        assert_eq!(program.check_commands.len(), 1);
        assert_eq!(program.expected_results.len(), 1);
        
        assert_eq!(program.property_map.get("prop_1"), Some(&"assertion_1".to_string()));
        assert_eq!(program.expected_results[0], SMTExpectedResult::Unsat);
    }

    #[test]
    fn test_smt_program_validation() {
        let mut program = SMTProgram::new();
        
        // Empty script should fail validation
        assert!(program.validate().is_err());
        
        // Add script content
        program.script = "(assert true)".to_string();
        assert!(program.validate().is_ok());
        
        // Add property without corresponding expected result should fail
        program.property_map.insert("prop_1".to_string(), "assertion_1".to_string());
        assert!(program.validate().is_err());
        
        // Fix by adding expected result
        program.expected_results.push(SMTExpectedResult::Sat);
        assert!(program.validate().is_ok());
    }

    #[test]
    fn test_smt_program_script_size() {
        let mut program = SMTProgram::new();
        program.script = "test script".to_string();
        assert_eq!(program.script_size(), 11);
    }

    #[test]
    fn test_smt_expected_result_equality() {
        assert_eq!(SMTExpectedResult::Sat, SMTExpectedResult::Sat);
        assert_eq!(SMTExpectedResult::Unsat, SMTExpectedResult::Unsat);
        assert_eq!(SMTExpectedResult::Unknown, SMTExpectedResult::Unknown);
        
        assert_ne!(SMTExpectedResult::Sat, SMTExpectedResult::Unsat);
        assert_ne!(SMTExpectedResult::Sat, SMTExpectedResult::Unknown);
        assert_ne!(SMTExpectedResult::Unsat, SMTExpectedResult::Unknown);
    }

    #[test]
    fn test_smt_generation_stats_creation() {
        let stats = SMTGenerationStats::new();
        assert_eq!(stats.properties_encoded, 0);
        assert_eq!(stats.sort_declarations, 0);
        assert_eq!(stats.function_declarations, 0);
        assert_eq!(stats.assertions_generated, 0);
        assert_eq!(stats.script_size, 0);
        assert_eq!(stats.generation_time_ms, 0);
    }

    #[test]
    fn test_smt_stats_encoding_efficiency() {
        let mut stats = SMTGenerationStats::new();
        
        // Zero script size should return 0 efficiency
        assert_eq!(stats.encoding_efficiency(), 0.0);
        
        // Set some values
        stats.properties_encoded = 10;
        stats.script_size = 2048; // 2KB
        
        // Should be 5 properties per KB
        assert_eq!(stats.encoding_efficiency(), 5.0);
    }

    #[test]
    fn test_smt_stats_total_declarations() {
        let mut stats = SMTGenerationStats::new();
        stats.sort_declarations = 3;
        stats.function_declarations = 7;
        
        assert_eq!(stats.total_declarations(), 10);
    }

    #[test]
    fn test_smt_generation_result_creation() {
        let result = SMTGenerationResult {
            program: SMTProgram::new(),
            stats: SMTGenerationStats::new(),
            warnings: vec!["test warning".to_string()],
        };
        
        assert_eq!(result.warnings.len(), 1);
        assert_eq!(result.warnings[0], "test warning");
    }

    #[test]
    fn test_default_implementations() {
        let _program: SMTProgram = Default::default();
        let _stats: SMTGenerationStats = Default::default();
        
        // Should not panic
        let default_program = SMTProgram::default();
        assert!(default_program.script.is_empty());
        
        let default_stats = SMTGenerationStats::default();
        assert_eq!(default_stats.properties_encoded, 0);
    }
}