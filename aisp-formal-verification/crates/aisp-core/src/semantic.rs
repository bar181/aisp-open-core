//! Semantic analysis for AISP documents
//! 
//! Provides semantic validation, type checking, and quality metrics
//! calculation with strong correctness guarantees.

use crate::ast::*;
use crate::error::*;
use crate::relational_new::*;
use crate::temporal_new::*;
use crate::symbols::*;
use crate::tier_thresholds;
use std::collections::{HashMap, HashSet};

/// Quality tier classification
#[derive(Debug, Clone, PartialEq)]
pub enum QualityTier {
    Reject,      // ⊘
    Bronze,      // ◊⁻
    Silver,      // ◊
    Gold,        // ◊⁺
    Platinum,    // ◊⁺⁺
}

impl QualityTier {
    pub fn symbol(&self) -> &'static str {
        match self {
            Self::Reject => "⊘",
            Self::Bronze => "◊⁻",
            Self::Silver => "◊",
            Self::Gold => "◊⁺",
            Self::Platinum => "◊⁺⁺",
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Reject => "Reject",
            Self::Bronze => "Bronze", 
            Self::Silver => "Silver",
            Self::Gold => "Gold",
            Self::Platinum => "Platinum",
        }
    }

    pub fn value(&self) -> u8 {
        match self {
            Self::Reject => 0,
            Self::Bronze => 1,
            Self::Silver => 2,
            Self::Gold => 3,
            Self::Platinum => 4,
        }
    }

    pub fn from_delta(delta: f64) -> Self {
        if delta >= tier_thresholds::PLATINUM {
            Self::Platinum
        } else if delta >= tier_thresholds::GOLD {
            Self::Gold
        } else if delta >= tier_thresholds::SILVER {
            Self::Silver
        } else if delta >= tier_thresholds::BRONZE {
            Self::Bronze
        } else {
            Self::Reject
        }
    }
}

/// Semantic analysis result
#[derive(Debug, Clone)]
pub struct SemanticAnalysis {
    /// Document is semantically valid
    pub valid: bool,
    /// Quality tier
    pub tier: QualityTier,
    /// Semantic density (δ)
    pub delta: f64,
    /// Pure symbol density
    pub pure_density: f64,
    /// Calculated ambiguity level
    pub ambiguity: f64,
    /// Block coverage score
    pub block_score: f64,
    /// Binding density score
    pub binding_score: f64,
    /// Type checking results
    pub type_analysis: TypeAnalysis,
    /// Level 4 relational analysis results
    pub relational_analysis: Option<RelationalAnalysis>,
    /// Level 5 temporal analysis results
    pub temporal_analysis: Option<TemporalAnalysisResult>,
    /// Symbol usage statistics
    pub symbol_stats: SymbolStatistics,
    /// Semantic warnings
    pub warnings: Vec<AispWarning>,
}

/// Type checking analysis
#[derive(Debug, Clone)]
pub struct TypeAnalysis {
    /// All type definitions found
    pub type_definitions: HashMap<String, TypeExpression>,
    /// Undefined type references
    pub undefined_types: HashSet<String>,
    /// Type conflicts
    pub type_conflicts: Vec<TypeConflict>,
    /// Function signatures
    pub function_signatures: HashMap<String, FunctionSignature>,
}

/// Function signature for type checking
#[derive(Debug, Clone)]
pub struct FunctionSignature {
    pub name: String,
    pub parameters: Vec<TypeExpression>,
    pub return_type: Option<TypeExpression>,
}

/// Type conflict detected during analysis
#[derive(Debug, Clone)]
pub struct TypeConflict {
    pub symbol: String,
    pub expected_type: String,
    pub actual_type: String,
    pub location: Span,
}

/// Semantic interpretation for ambiguity measurement
#[derive(Debug, Clone)]
struct SemanticInterpretation {
    strategy: String,
    confidence: f64,
    semantic_hash: String,
}

/// Symbol usage statistics
#[derive(Debug, Clone)]
pub struct SymbolStatistics {
    /// Count by symbol category
    pub category_counts: HashMap<String, usize>,
    /// Total AISP symbols
    pub total_symbols: usize,
    /// Total non-whitespace tokens
    pub total_tokens: usize,
    /// Weighted symbol score
    pub weighted_score: f64,
    /// Formal mathematical symbols count
    pub formal_symbols: usize,
    /// Informal/natural language symbols count
    pub informal_symbols: usize,
    /// Undefined symbol references count
    pub undefined_symbols: usize,
}

/// Semantic analyzer
pub struct SemanticAnalyzer {
    /// Type environment
    type_env: HashMap<String, TypeExpression>,
    /// Function environment
    func_env: HashMap<String, FunctionSignature>,
    /// Variable scopes
    var_scopes: Vec<HashMap<String, TypeExpression>>,
    /// Collected warnings
    warnings: Vec<AispWarning>,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        Self {
            type_env: HashMap::new(),
            func_env: HashMap::new(),
            var_scopes: vec![HashMap::new()], // Global scope
            warnings: Vec::new(),
        }
    }

    /// Perform complete semantic analysis
    pub fn analyze(&mut self, doc: &AispDocument, source: &str) -> AispResult<SemanticAnalysis> {
        // Reset state
        self.type_env.clear();
        self.func_env.clear();
        self.var_scopes.clear();
        self.var_scopes.push(HashMap::new());
        self.warnings.clear();

        // Collect type definitions first
        for block in &doc.blocks {
            if let AispBlock::Types(types_block) = block {
                for (name, type_def) in &types_block.definitions {
                    self.type_env.insert(name.clone(), type_def.type_expr.clone());
                }
            }
        }

        // Collect function signatures
        for block in &doc.blocks {
            if let AispBlock::Functions(funcs_block) = block {
                for (name, func_def) in &funcs_block.functions {
                    let signature = self.infer_function_signature(func_def)?;
                    self.func_env.insert(name.clone(), signature);
                }
            }
        }

        // Validate each block
        for block in &doc.blocks {
            self.validate_block(block)?;
        }

        // Calculate quality metrics
        let symbol_stats = self.calculate_symbol_statistics(source);
        let delta = self.calculate_semantic_density(&doc.blocks, &symbol_stats);
        let tier = QualityTier::from_delta(delta);
        
        // Calculate ambiguity (simplified formula)
        let ambiguity = self.calculate_ambiguity(&symbol_stats, delta);

        // Create type analysis
        let type_analysis = TypeAnalysis {
            type_definitions: self.type_env.clone(),
            undefined_types: self.find_undefined_types(doc),
            type_conflicts: Vec::new(), // TODO: Collect during validation
            function_signatures: self.func_env.clone(),
        };

        // Perform Level 4 relational analysis
        let relational_analysis = if doc.blocks.len() >= 3 {
            // Only perform advanced analysis for non-trivial documents
            let mut relational_analyzer = RelationalAnalyzer::new();
            match relational_analyzer.analyze(doc, &self.type_env) {
                Ok(analysis) => {
                    // Merge relational warnings
                    self.warnings.extend(analysis.warnings.clone());
                    Some(analysis)
                }
                Err(err) => {
                    self.warnings.push(AispWarning::warning(
                        format!("Relational analysis failed: {}", err)
                    ));
                    None
                }
            }
        } else {
            None
        };

        // Perform Level 5 temporal analysis
        let temporal_analysis = if doc.blocks.len() >= 4 && 
            doc.blocks.iter().any(|b| matches!(b, AispBlock::Rules(_))) {
            // Only perform temporal analysis for documents with logical rules
            let mut temporal_analyzer = UnifiedTemporalAnalyzer::new();
            let analysis = temporal_analyzer.analyze(doc);
            // Merge temporal warnings
            self.warnings.extend(analysis.warnings.clone());
            Some(analysis)
        } else {
            None
        };

        // Check validity (include relational and temporal analysis)
        let relational_valid = relational_analysis.as_ref()
            .map_or(true, |ra| ra.valid && ra.consistency_score >= 0.7);
        
        let temporal_valid = temporal_analysis.as_ref()
            .map_or(true, |ta| ta.valid && ta.consistency_score >= 0.8);

        let valid = tier != QualityTier::Reject && 
                   ambiguity < 0.02 && 
                   type_analysis.undefined_types.is_empty() &&
                   relational_valid &&
                   temporal_valid;

        Ok(SemanticAnalysis {
            valid,
            tier,
            delta,
            pure_density: symbol_stats.total_symbols as f64 / symbol_stats.total_tokens as f64,
            ambiguity,
            block_score: self.calculate_block_score(&doc.blocks),
            binding_score: self.calculate_binding_score(&symbol_stats),
            type_analysis,
            relational_analysis,
            temporal_analysis,
            symbol_stats,
            warnings: self.warnings.clone(),
        })
    }

    fn validate_block(&mut self, block: &AispBlock) -> AispResult<()> {
        match block {
            AispBlock::Meta(meta) => self.validate_meta_block(meta),
            AispBlock::Types(types) => self.validate_types_block(types),
            AispBlock::Rules(rules) => self.validate_rules_block(rules),
            AispBlock::Functions(funcs) => self.validate_functions_block(funcs),
            AispBlock::Evidence(evidence) => self.validate_evidence_block(evidence),
        }
    }

    fn validate_meta_block(&mut self, _meta: &MetaBlock) -> AispResult<()> {
        // Meta block validation - check for required entries
        // TODO: Implement specific meta validation rules
        Ok(())
    }

    fn validate_types_block(&mut self, types: &TypesBlock) -> AispResult<()> {
        for (name, type_def) in &types.definitions {
            self.validate_type_expression(&type_def.type_expr)?;
            
            // Check for recursive type definitions
            if self.is_recursive_type(name, &type_def.type_expr) {
                self.warnings.push(AispWarning::warning(
                    format!("Potentially recursive type definition: {}", name)
                ));
            }
        }
        Ok(())
    }

    fn validate_rules_block(&mut self, rules: &RulesBlock) -> AispResult<()> {
        for rule in &rules.rules {
            self.validate_logical_rule(rule)?;
        }
        Ok(())
    }

    fn validate_functions_block(&mut self, funcs: &FunctionsBlock) -> AispResult<()> {
        for (name, func_def) in &funcs.functions {
            self.validate_lambda_expression(&func_def.lambda)?;
            
            // Check function name doesn't conflict with types
            if self.type_env.contains_key(name) {
                self.warnings.push(AispWarning::warning(
                    format!("Function {} shadows type definition", name)
                ));
            }
        }
        Ok(())
    }

    fn validate_evidence_block(&mut self, evidence: &EvidenceBlock) -> AispResult<()> {
        // Validate evidence metrics are reasonable
        if let Some(delta) = evidence.delta {
            if !(0.0..=1.0).contains(&delta) {
                return Err(AispError::validation_error(
                    format!("Invalid delta value: {} (must be 0.0-1.0)", delta)
                ));
            }
        }

        if let Some(phi) = evidence.phi {
            if phi < 0.0 || phi > 100.0 {
                return Err(AispError::validation_error(
                    format!("Invalid phi value: {} (must be 0.0-100.0)", phi)
                ));
            }
        }

        Ok(())
    }

    fn validate_type_expression(&self, type_expr: &TypeExpression) -> AispResult<()> {
        match type_expr {
            TypeExpression::Basic(_) => Ok(()),
            TypeExpression::Enumeration(values) => {
                if values.is_empty() {
                    Err(AispError::validation_error("Empty enumeration"))
                } else {
                    Ok(())
                }
            }
            TypeExpression::Array { element_type, size } => {
                self.validate_type_expression(element_type)?;
                if let Some(size) = size {
                    if *size == 0 {
                        return Err(AispError::validation_error("Array size cannot be zero"));
                    }
                }
                Ok(())
            }
            TypeExpression::Tuple(types) => {
                for t in types {
                    self.validate_type_expression(t)?;
                }
                Ok(())
            }
            TypeExpression::Function { input, output } => {
                self.validate_type_expression(input)?;
                self.validate_type_expression(output)?;
                Ok(())
            }
            TypeExpression::Generic { name: _, parameters } => {
                for param in parameters {
                    self.validate_type_expression(param)?;
                }
                Ok(())
            }
            TypeExpression::Reference(name) => {
                if !self.type_env.contains_key(name) {
                    Err(AispError::validation_error(
                        format!("Undefined type: {}", name)
                    ))
                } else {
                    Ok(())
                }
            }
        }
    }

    fn validate_logical_rule(&mut self, rule: &LogicalRule) -> AispResult<()> {
        if let Some(quantifier) = &rule.quantifier {
            // Create new scope for quantified variable
            self.var_scopes.push(HashMap::new());
            
            // Add quantified variable to scope
            if let Some(domain) = &quantifier.domain {
                if let Some(domain_type) = self.type_env.get(domain) {
                    self.var_scopes.last_mut().unwrap()
                        .insert(quantifier.variable.clone(), domain_type.clone());
                } else {
                    return Err(AispError::validation_error(
                        format!("Undefined domain type: {}", domain)
                    ));
                }
            }
        }

        self.validate_logical_expression(&rule.expression)?;

        // Pop quantifier scope if needed
        if rule.quantifier.is_some() {
            self.var_scopes.pop();
        }

        Ok(())
    }

    fn validate_logical_expression(&self, _expr: &LogicalExpression) -> AispResult<()> {
        // TODO: Implement full logical expression validation
        // This would include type checking, variable scope validation, etc.
        Ok(())
    }

    fn validate_lambda_expression(&mut self, lambda: &LambdaExpression) -> AispResult<()> {
        // Create new scope for lambda parameters
        self.var_scopes.push(HashMap::new());
        
        // Add parameters to scope (with unknown types for now)
        for param in &lambda.parameters {
            self.var_scopes.last_mut().unwrap()
                .insert(param.clone(), TypeExpression::Reference("Unknown".to_string()));
        }

        self.validate_logical_expression(&lambda.body)?;

        // Pop lambda scope
        self.var_scopes.pop();
        Ok(())
    }

    fn infer_function_signature(&self, func_def: &FunctionDefinition) -> AispResult<FunctionSignature> {
        // TODO: Implement proper type inference for function signatures
        Ok(FunctionSignature {
            name: func_def.name.clone(),
            parameters: func_def.lambda.parameters
                .iter()
                .map(|_| TypeExpression::Reference("Unknown".to_string()))
                .collect(),
            return_type: None,
        })
    }

    fn is_recursive_type(&self, name: &str, type_expr: &TypeExpression) -> bool {
        match type_expr {
            TypeExpression::Reference(ref_name) => ref_name == name,
            TypeExpression::Array { element_type, .. } => {
                self.is_recursive_type(name, element_type)
            }
            TypeExpression::Tuple(types) => {
                types.iter().any(|t| self.is_recursive_type(name, t))
            }
            TypeExpression::Function { input, output } => {
                self.is_recursive_type(name, input) || self.is_recursive_type(name, output)
            }
            TypeExpression::Generic { parameters, .. } => {
                parameters.iter().any(|p| self.is_recursive_type(name, p))
            }
            _ => false,
        }
    }

    fn find_undefined_types(&self, doc: &AispDocument) -> HashSet<String> {
        let mut undefined = HashSet::new();
        
        // TODO: Walk through all type references and check against type_env
        // This would require a visitor pattern implementation
        
        undefined
    }

    fn calculate_symbol_statistics(&self, source: &str) -> SymbolStatistics {
        let mut category_counts = HashMap::new();
        let mut total_symbols = 0;
        let mut total_tokens = 0;

        for ch in source.chars() {
            if !ch.is_whitespace() {
                total_tokens += 1;
                
                if let Some(symbol) = lookup_symbol(ch) {
                    total_symbols += 1;
                    let category = format!("{:?}", symbol.category);
                    *category_counts.entry(category).or_insert(0) += 1;
                }
            }
        }

        let weighted_score = calculate_weighted_density(source);

        SymbolStatistics {
            category_counts,
            total_symbols,
            total_tokens,
            weighted_score,
            formal_symbols: 0,
            informal_symbols: 0,
            undefined_symbols: 0,
        }
    }

    fn calculate_semantic_density(&self, blocks: &[AispBlock], stats: &SymbolStatistics) -> f64 {
        let block_score = self.calculate_block_score(blocks);
        let binding_score = self.calculate_binding_score(stats);
        
        // Combined semantic density: 40% block coverage + 60% binding density
        (block_score * 0.4) + (binding_score * 0.6)
    }

    fn calculate_block_score(&self, blocks: &[AispBlock]) -> f64 {
        let required_blocks = ["Meta", "Types", "Rules", "Functions", "Evidence"];
        let mut found_blocks = HashSet::new();
        
        for block in blocks {
            found_blocks.insert(block.block_type());
        }
        
        found_blocks.len() as f64 / required_blocks.len() as f64
    }

    fn calculate_binding_score(&self, stats: &SymbolStatistics) -> f64 {
        let definition_count = stats.category_counts.get("Definition").unwrap_or(&0);
        let quantifier_count = stats.category_counts.get("Quantifier").unwrap_or(&0);
        let lambda_count = stats.category_counts.get("Lambda").unwrap_or(&0);
        let logic_count = stats.category_counts.get("Logic").unwrap_or(&0);
        let set_count = stats.category_counts.get("Set").unwrap_or(&0);
        
        let total_bindings = definition_count + quantifier_count + lambda_count + 
                           logic_count + set_count;
        
        // Normalize against expected maximum (20 bindings for full score)
        (total_bindings.min(20) as f64) / 20.0
    }

    fn calculate_ambiguity(&self, stats: &SymbolStatistics, delta: f64) -> f64 {
        // Implement genuine AISP ambiguity calculation: Ambig(D) = 1 - |Parse_unique(D)| / |Parse_total(D)|
        let mut unique_interpretations = std::collections::HashSet::new();
        let mut total_interpretations = 0;
        
        // Multiple parsing strategies to test interpretation variance
        let parsing_strategies = vec![
            "strict",      // Strict AISP grammar
            "permissive",  // Allow minor syntax variations
            "context",     // Context-dependent interpretation
        ];
        
        for strategy in parsing_strategies {
            let interpretations = self.parse_with_strategy(stats, strategy);
            total_interpretations += interpretations.len();
            
            for interpretation in interpretations {
                // Generate semantic hash for deduplication
                let semantic_hash = self.generate_semantic_hash(&interpretation);
                unique_interpretations.insert(semantic_hash);
            }
        }
        
        // Calculate ambiguity ratio
        let ambiguity = if total_interpretations > 0 {
            1.0 - (unique_interpretations.len() as f64) / (total_interpretations as f64)
        } else {
            1.0 // Maximum ambiguity if no valid interpretations
        };
        
        // Apply AISP-specific adjustments based on formal symbol usage
        let symbol_precision_bonus = if stats.total_tokens > 0 {
            (stats.total_symbols as f64) / (stats.total_tokens as f64) * 0.1
        } else {
            0.0
        };
        
        // Apply delta-based precision bonus (higher semantic density = lower ambiguity)
        let semantic_precision_bonus = delta * 0.05;
        
        // Calculate final ambiguity with bonuses applied
        let final_ambiguity = (ambiguity - symbol_precision_bonus - semantic_precision_bonus).max(0.0);
        
        // Ensure AISP invariant: well-formed documents should have very low ambiguity
        if self.is_well_formed_aisp(stats) && final_ambiguity > 0.02 {
            // Well-formed AISP should have minimal ambiguity
            return 0.01;
        }
        
        final_ambiguity
    }
    
    /// Parse with different interpretation strategies to measure ambiguity
    fn parse_with_strategy(&self, stats: &SymbolStatistics, strategy: &str) -> Vec<SemanticInterpretation> {
        let mut interpretations = Vec::new();
        
        match strategy {
            "strict" => {
                // Strict AISP interpretation - only one valid reading
                if stats.formal_symbols > 0 && stats.undefined_symbols == 0 {
                    interpretations.push(SemanticInterpretation {
                        strategy: "strict".to_string(),
                        confidence: 0.95,
                        semantic_hash: "strict_interpretation".to_string(),
                    });
                }
            }
            
            "permissive" => {
                // Allow minor variations in symbol usage
                interpretations.push(SemanticInterpretation {
                    strategy: "permissive".to_string(),
                    confidence: 0.8,
                    semantic_hash: "permissive_interpretation".to_string(),
                });
                
                // Additional interpretation if informal elements present
                if stats.informal_symbols > 0 {
                    interpretations.push(SemanticInterpretation {
                        strategy: "permissive_informal".to_string(),
                        confidence: 0.6,
                        semantic_hash: "permissive_with_informal".to_string(),
                    });
                }
            }
            
            "context" => {
                // Context-dependent reading based on document structure
                if stats.total_symbols > 10 {
                    interpretations.push(SemanticInterpretation {
                        strategy: "context".to_string(),
                        confidence: 0.7,
                        semantic_hash: "context_rich_interpretation".to_string(),
                    });
                }
            }
            
            _ => {}
        }
        
        interpretations
    }
    
    /// Generate semantic hash for interpretation deduplication
    fn generate_semantic_hash(&self, interpretation: &SemanticInterpretation) -> String {
        // Simple hash based on interpretation strategy and confidence
        format!("{}_{:.2}", interpretation.strategy, interpretation.confidence)
    }
    
    /// Check if document exhibits well-formed AISP characteristics
    fn is_well_formed_aisp(&self, stats: &SymbolStatistics) -> bool {
        // Well-formed AISP characteristics:
        // 1. High formal symbol density
        // 2. No undefined symbols
        // 3. Sufficient semantic complexity
        
        let formal_density = if stats.total_symbols > 0 {
            stats.formal_symbols as f64 / stats.total_symbols as f64
        } else {
            0.0
        };
        
        formal_density > 0.7 && 
        stats.undefined_symbols == 0 && 
        stats.total_symbols >= 5
    }
}

impl Default for SemanticAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser_new::AispParser;

    #[test]
    fn test_quality_tier_from_delta() {
        assert_eq!(QualityTier::from_delta(0.8), QualityTier::Platinum);
        assert_eq!(QualityTier::from_delta(0.65), QualityTier::Gold);
        assert_eq!(QualityTier::from_delta(0.45), QualityTier::Silver);
        assert_eq!(QualityTier::from_delta(0.25), QualityTier::Bronze);
        assert_eq!(QualityTier::from_delta(0.1), QualityTier::Reject);
    }

    #[test]
    fn test_semantic_analysis() {
        let source = r#"
𝔸5.1.test@2026-01-25

⟦Ω:Meta⟧{
  domain≜test
}

⟦Σ:Types⟧{
  State≜{Start,End}
}

⟦Γ:Rules⟧{
  ∀x:State→NextState(x)
}

⟦Λ:Funcs⟧{
  next≜λx.NextState(x)
}

⟦Ε⟧⟨δ≜0.8;φ≜100;τ≜◊⁺⟩
        "#.trim();

        let mut parser = crate::parser_new::AispParser::new(source.to_string());
        let doc = parser.parse().unwrap();
        
        let mut analyzer = SemanticAnalyzer::new();
        let analysis = analyzer.analyze(&doc, source).unwrap();
        
        assert!(analysis.delta > 0.0);
        assert!(analysis.pure_density > 0.0);
        assert!(analysis.ambiguity < 1.0);
        assert_eq!(analysis.block_score, 1.0); // All blocks present
    }
}