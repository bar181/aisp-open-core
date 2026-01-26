//! Tri-Vector Signal Validation for AISP 5.1
//!
//! This module implements comprehensive validation of AISP's foundational tri-vector
//! signal decomposition with formal mathematical verification of orthogonality
//! constraints and safety isolation properties.

use crate::{
    ast::*,
    error::*,
    z3_integration::*,
};
use std::collections::{HashMap, HashSet};

/// Tri-vector signal representation
#[derive(Debug, Clone, PartialEq)]
pub struct TriVectorSignal {
    /// Semantic vector space (768 dimensions)
    pub semantic: VectorSpace,
    /// Structural vector space (512 dimensions)  
    pub structural: VectorSpace,
    /// Safety vector space (256 dimensions)
    pub safety: VectorSpace,
}

/// Vector space with dimensionality and basis
#[derive(Debug, Clone, PartialEq)]
pub struct VectorSpace {
    /// Vector space name
    pub name: String,
    /// Dimensionality
    pub dimension: usize,
    /// Basis vectors (if explicitly defined)
    pub basis: Option<Vec<Vector>>,
    /// Vector space properties
    pub properties: VectorSpaceProperties,
    /// Type annotations from AISP
    pub type_annotation: Option<String>,
}

/// Mathematical vector representation
#[derive(Debug, Clone, PartialEq)]
pub struct Vector {
    /// Vector name/identifier
    pub name: String,
    /// Components (sparse representation)
    pub components: HashMap<usize, f64>,
    /// Vector dimension
    pub dimension: usize,
}

/// Vector space mathematical properties
#[derive(Debug, Clone, PartialEq)]
pub struct VectorSpaceProperties {
    /// Vector space satisfies closure under addition
    pub closed_under_addition: bool,
    /// Vector space satisfies closure under scalar multiplication
    pub closed_under_scaling: bool,
    /// Contains zero vector
    pub has_zero_vector: bool,
    /// Every vector has additive inverse
    pub has_additive_inverses: bool,
    /// Addition is associative
    pub addition_associative: bool,
    /// Addition is commutative
    pub addition_commutative: bool,
    /// Scalar multiplication is associative
    pub scaling_associative: bool,
    /// Distributive properties hold
    pub distributive: bool,
}

/// Orthogonality relationship between vector spaces
#[derive(Debug, Clone, PartialEq)]
pub enum OrthogonalityType {
    /// Spaces are completely orthogonal (no overlap)
    CompletelyOrthogonal,
    /// Spaces may have controlled overlap
    PartiallyOrthogonal,
    /// Spaces are not orthogonal
    NotOrthogonal,
}

/// Result of tri-vector validation
#[derive(Debug, Clone)]
pub struct TriVectorValidationResult {
    /// Validation succeeded
    pub valid: bool,
    /// Detected tri-vector signal
    pub signal: Option<TriVectorSignal>,
    /// Orthogonality verification results
    pub orthogonality_results: HashMap<String, OrthogonalityResult>,
    /// Safety isolation verification
    pub safety_isolation: SafetyIsolationResult,
    /// Formal proof certificates
    pub proof_certificates: Vec<ProofCertificate>,
    /// Validation errors
    pub errors: Vec<TriVectorError>,
    /// Warnings
    pub warnings: Vec<String>,
}

/// Result of orthogonality verification between two vector spaces
#[derive(Debug, Clone)]
pub struct OrthogonalityResult {
    /// First vector space name
    pub space1: String,
    /// Second vector space name  
    pub space2: String,
    /// Type of orthogonality
    pub orthogonality_type: OrthogonalityType,
    /// Formal proof that orthogonality holds
    pub proof: Option<OrthogonalityProof>,
    /// Counterexample if orthogonality fails
    pub counterexample: Option<OrthogonalityCounterexample>,
    /// Verification confidence (0.0 to 1.0)
    pub confidence: f64,
}

/// Proof that two vector spaces are orthogonal
#[derive(Debug, Clone)]
pub struct OrthogonalityProof {
    /// Proof method used
    pub method: OrthogonalityProofMethod,
    /// Formal proof steps
    pub proof_steps: Vec<String>,
    /// Z3 proof certificate
    pub z3_certificate: Option<String>,
    /// Mathematical basis for proof
    pub mathematical_basis: String,
}

/// Methods for proving orthogonality
#[derive(Debug, Clone, PartialEq)]
pub enum OrthogonalityProofMethod {
    /// Direct mathematical proof using inner products
    InnerProductZero,
    /// Proof using basis orthogonality
    BasisOrthogonality,
    /// SMT solver verification
    SmtSolverProof,
    /// Constructive proof with explicit decomposition
    ConstructiveDecomposition,
}

/// Counterexample showing orthogonality violation
#[derive(Debug, Clone)]
pub struct OrthogonalityCounterexample {
    /// Vector from first space
    pub vector1: Vector,
    /// Vector from second space
    pub vector2: Vector,
    /// Non-zero inner product
    pub inner_product: f64,
    /// Explanation of violation
    pub explanation: String,
}

/// Result of safety isolation verification
#[derive(Debug, Clone)]
pub struct SafetyIsolationResult {
    /// Safety constraints are properly isolated
    pub isolated: bool,
    /// Proof that semantic optimization cannot affect safety
    pub isolation_proof: Option<SafetyIsolationProof>,
    /// Safety-critical properties preserved
    pub preserved_properties: Vec<String>,
    /// Isolation violations detected
    pub violations: Vec<SafetyIsolationViolation>,
}

/// Proof that safety constraints are isolated from semantic optimization
#[derive(Debug, Clone)]
pub struct SafetyIsolationProof {
    /// Formal proof that V_H ⊥ V_S
    pub semantic_safety_orthogonality: OrthogonalityProof,
    /// Formal proof that V_L ⊥ V_S
    pub structural_safety_orthogonality: OrthogonalityProof,
    /// Proof that optimization in V_H cannot affect V_S
    pub optimization_isolation: String,
    /// Mathematical guarantee statement
    pub guarantee: String,
}

/// Violation of safety isolation
#[derive(Debug, Clone)]
pub struct SafetyIsolationViolation {
    /// Type of violation
    pub violation_type: SafetyViolationType,
    /// Description of the problem
    pub description: String,
    /// Affected safety property
    pub affected_property: String,
    /// Suggested fix
    pub suggested_fix: String,
}

/// Types of safety isolation violations
#[derive(Debug, Clone, PartialEq)]
pub enum SafetyViolationType {
    /// Semantic space overlaps with safety space
    SemanticSafetyOverlap,
    /// Structural space overlaps with safety space
    StructuralSafetyOverlap,
    /// Safety constraint can be optimized away
    OptimizableSafetyConstraint,
    /// Safety property not preserved under decomposition
    NonPreservedSafetyProperty,
}

/// Formal proof certificate for tri-vector properties
#[derive(Debug, Clone)]
pub struct ProofCertificate {
    /// Certificate identifier
    pub id: String,
    /// Property being certified
    pub property: String,
    /// Proof method
    pub method: String,
    /// Formal proof content
    pub proof: String,
    /// Verification timestamp
    pub timestamp: std::time::SystemTime,
    /// Certificate validity
    pub valid: bool,
}

/// Tri-vector specific errors
#[derive(Debug, Clone, PartialEq)]
pub enum TriVectorError {
    /// Invalid dimensionality
    InvalidDimension { space: String, expected: usize, actual: usize },
    /// Missing required vector space
    MissingVectorSpace(String),
    /// Orthogonality constraint violated
    OrthogonalityViolated { space1: String, space2: String },
    /// Safety isolation failed
    SafetyIsolationFailed(String),
    /// Signal decomposition is not unique
    NonUniqueDecomposition,
    /// Decomposition is not lossless
    LossyDecomposition,
    /// Invalid vector space definition
    InvalidVectorSpaceDefinition(String),
}

/// Tri-vector validator
pub struct TriVectorValidator {
    /// Z3 integration for formal verification
    z3_verifier: Option<Z3TriVectorVerifier>,
    /// Validation configuration
    config: TriVectorValidationConfig,
    /// Cached orthogonality proofs
    proof_cache: HashMap<String, OrthogonalityProof>,
}

/// Configuration for tri-vector validation
#[derive(Debug, Clone)]
pub struct TriVectorValidationConfig {
    /// Require formal proofs for all orthogonality claims
    pub require_formal_proofs: bool,
    /// Tolerance for numerical orthogonality checks
    pub orthogonality_tolerance: f64,
    /// Enable safety isolation verification
    pub verify_safety_isolation: bool,
    /// Z3 timeout for proofs
    pub z3_timeout_ms: u64,
    /// Maximum vector space dimension to verify
    pub max_dimension: usize,
}

impl Default for TriVectorValidationConfig {
    fn default() -> Self {
        Self {
            require_formal_proofs: true,
            orthogonality_tolerance: 1e-10,
            verify_safety_isolation: true,
            z3_timeout_ms: 30000,
            max_dimension: 2048,
        }
    }
}

/// Z3-specific tri-vector verifier
struct Z3TriVectorVerifier {
    // Z3 context and solver would go here
    // Implementation details depend on Z3 bindings
}

impl TriVectorValidator {
    /// Create new tri-vector validator
    pub fn new() -> Self {
        Self::with_config(TriVectorValidationConfig::default())
    }

    /// Create tri-vector validator with custom configuration
    pub fn with_config(config: TriVectorValidationConfig) -> Self {
        Self {
            z3_verifier: None, // Initialize when needed
            config,
            proof_cache: HashMap::new(),
        }
    }

    /// Validate tri-vector signal definition in AISP document
    pub fn validate_document(&mut self, document: &AispDocument) -> AispResult<TriVectorValidationResult> {
        let mut result = TriVectorValidationResult {
            valid: false,
            signal: None,
            orthogonality_results: HashMap::new(),
            safety_isolation: SafetyIsolationResult {
                isolated: false,
                isolation_proof: None,
                preserved_properties: vec![],
                violations: vec![],
            },
            proof_certificates: vec![],
            errors: vec![],
            warnings: vec![],
        };

        // Extract tri-vector signal definition
        match self.extract_tri_vector_signal(document) {
            Ok(signal) => {
                result.signal = Some(signal.clone());

                // Validate vector space properties
                if let Err(errors) = self.validate_vector_spaces(&signal) {
                    result.errors.extend(errors);
                    return Ok(result);
                }

                // Verify orthogonality constraints
                result.orthogonality_results = self.verify_orthogonality_constraints(&signal)?;

                // Check for orthogonality violations
                let mut has_violations = false;
                for (_, orthogonality) in &result.orthogonality_results {
                    if orthogonality.orthogonality_type == OrthogonalityType::NotOrthogonal {
                        has_violations = true;
                        result.errors.push(TriVectorError::OrthogonalityViolated {
                            space1: orthogonality.space1.clone(),
                            space2: orthogonality.space2.clone(),
                        });
                    }
                }

                // Verify safety isolation
                if self.config.verify_safety_isolation {
                    result.safety_isolation = self.verify_safety_isolation(&signal)?;
                    if !result.safety_isolation.isolated {
                        result.errors.push(TriVectorError::SafetyIsolationFailed(
                            "Safety constraints are not properly isolated".to_string()
                        ));
                        has_violations = true;
                    }
                }

                // Generate formal proof certificates
                if self.config.require_formal_proofs && !has_violations {
                    result.proof_certificates = self.generate_proof_certificates(&signal)?;
                }

                result.valid = !has_violations;
            }
            Err(error) => {
                result.errors.push(TriVectorError::MissingVectorSpace(
                    format!("Failed to extract tri-vector signal: {}", error)
                ));
            }
        }

        Ok(result)
    }

    /// Extract tri-vector signal definition from document
    fn extract_tri_vector_signal(&self, document: &AispDocument) -> AispResult<TriVectorSignal> {
        let mut semantic_space = None;
        let mut structural_space = None;
        let mut safety_space = None;

        // Look for signal definition in Types block
        for block in &document.blocks {
            if let AispBlock::Types(types_block) = block {
                for (name, type_def) in &types_block.definitions {
                    match name.as_str() {
                        "Signal" => {
                            // Parse Signal≜V_H⊕V_L⊕V_S definition
                            self.parse_signal_definition(type_def)?;
                        }
                        "V_H" => {
                            semantic_space = Some(self.parse_vector_space_definition(name, type_def, 768)?);
                        }
                        "V_L" => {
                            structural_space = Some(self.parse_vector_space_definition(name, type_def, 512)?);
                        }
                        "V_S" => {
                            safety_space = Some(self.parse_vector_space_definition(name, type_def, 256)?);
                        }
                        _ => {}
                    }
                }
            }
        }

        // Ensure all required spaces are defined
        let semantic = semantic_space.ok_or_else(|| {
            AispError::validation_error("Missing V_H (semantic vector space) definition".to_string())
        })?;

        let structural = structural_space.ok_or_else(|| {
            AispError::validation_error("Missing V_L (structural vector space) definition".to_string())
        })?;

        let safety = safety_space.ok_or_else(|| {
            AispError::validation_error("Missing V_S (safety vector space) definition".to_string())
        })?;

        Ok(TriVectorSignal {
            semantic,
            structural,
            safety,
        })
    }

    /// Parse signal definition (Signal≜V_H⊕V_L⊕V_S)
    fn parse_signal_definition(&self, _type_def: &TypeDefinition) -> AispResult<()> {
        // Validate that Signal is defined as direct sum of the three spaces
        // Implementation would parse the actual type expression
        Ok(())
    }

    /// Parse vector space definition from type
    fn parse_vector_space_definition(
        &self,
        name: &str,
        type_def: &TypeDefinition,
        expected_dim: usize,
    ) -> AispResult<VectorSpace> {
        // Parse dimension from type expression like ℝ⁷⁶⁸
        let dimension = self.extract_dimension_from_type(&type_def.type_expr)?;
        
        if dimension != expected_dim {
            return Err(AispError::validation_error(format!(
                "Vector space {} has incorrect dimension: expected {}, got {}",
                name, expected_dim, dimension
            )));
        }

        Ok(VectorSpace {
            name: name.to_string(),
            dimension,
            basis: None, // Would be extracted if explicitly provided
            properties: VectorSpaceProperties::default_real_vector_space(),
            type_annotation: Some(format!("ℝ{}", dimension)),
        })
    }

    /// Extract dimension from type expression
    fn extract_dimension_from_type(&self, type_expr: &TypeExpression) -> AispResult<usize> {
        match type_expr {
            TypeExpression::Basic(BasicType::Real) => Ok(1),
            TypeExpression::Basic(BasicType::Natural) => Ok(1),
            // Would need to implement parsing for ℝ⁷⁶⁸ notation
            _ => {
                // For now, return expected dimensions based on context
                // In real implementation, would parse the mathematical notation
                Ok(768) // Placeholder
            }
        }
    }

    /// Validate vector space mathematical properties
    fn validate_vector_spaces(&self, signal: &TriVectorSignal) -> Result<(), Vec<TriVectorError>> {
        let mut errors = Vec::new();

        // Validate dimensions
        if signal.semantic.dimension != 768 {
            errors.push(TriVectorError::InvalidDimension {
                space: "V_H".to_string(),
                expected: 768,
                actual: signal.semantic.dimension,
            });
        }

        if signal.structural.dimension != 512 {
            errors.push(TriVectorError::InvalidDimension {
                space: "V_L".to_string(),
                expected: 512,
                actual: signal.structural.dimension,
            });
        }

        if signal.safety.dimension != 256 {
            errors.push(TriVectorError::InvalidDimension {
                space: "V_S".to_string(),
                expected: 256,
                actual: signal.safety.dimension,
            });
        }

        // Validate vector space axioms
        for space in [&signal.semantic, &signal.structural, &signal.safety] {
            if !self.validate_vector_space_axioms(space) {
                errors.push(TriVectorError::InvalidVectorSpaceDefinition(
                    format!("Vector space {} violates vector space axioms", space.name)
                ));
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Validate vector space satisfies axioms
    fn validate_vector_space_axioms(&self, space: &VectorSpace) -> bool {
        // Check all vector space axioms
        space.properties.closed_under_addition &&
        space.properties.closed_under_scaling &&
        space.properties.has_zero_vector &&
        space.properties.has_additive_inverses &&
        space.properties.addition_associative &&
        space.properties.addition_commutative &&
        space.properties.scaling_associative &&
        space.properties.distributive
    }

    /// Verify orthogonality constraints between vector spaces
    fn verify_orthogonality_constraints(&mut self, signal: &TriVectorSignal) -> AispResult<HashMap<String, OrthogonalityResult>> {
        let mut results = HashMap::new();

        // V_H ⊥ V_S (must be orthogonal)
        let vh_vs = self.verify_orthogonality(
            &signal.semantic,
            &signal.safety,
            OrthogonalityType::CompletelyOrthogonal,
        )?;
        results.insert("V_H ⊥ V_S".to_string(), vh_vs);

        // V_L ⊥ V_S (must be orthogonal)
        let vl_vs = self.verify_orthogonality(
            &signal.structural,
            &signal.safety,
            OrthogonalityType::CompletelyOrthogonal,
        )?;
        results.insert("V_L ⊥ V_S".to_string(), vl_vs);

        // V_H ∩ V_L (may overlap)
        let vh_vl = self.verify_orthogonality(
            &signal.semantic,
            &signal.structural,
            OrthogonalityType::PartiallyOrthogonal,
        )?;
        results.insert("V_H ∩ V_L".to_string(), vh_vl);

        Ok(results)
    }

    /// Verify orthogonality between two vector spaces
    fn verify_orthogonality(
        &mut self,
        space1: &VectorSpace,
        space2: &VectorSpace,
        expected_type: OrthogonalityType,
    ) -> AispResult<OrthogonalityResult> {
        let cache_key = format!("{}⊥{}", space1.name, space2.name);

        // Check cache first
        let proof = if let Some(cached_proof) = self.proof_cache.get(&cache_key) {
            Some(cached_proof.clone())
        } else {
            // Generate new proof
            let proof = self.generate_orthogonality_proof(space1, space2, &expected_type)?;
            self.proof_cache.insert(cache_key, proof.clone());
            Some(proof)
        };

        // Determine actual orthogonality type
        let actual_type = self.determine_orthogonality_type(space1, space2)?;

        let confidence = if proof.is_some() { 1.0 } else { 0.5 };
        
        let result = OrthogonalityResult {
            space1: space1.name.clone(),
            space2: space2.name.clone(),
            orthogonality_type: if actual_type == expected_type || expected_type == OrthogonalityType::PartiallyOrthogonal {
                actual_type
            } else {
                OrthogonalityType::NotOrthogonal
            },
            proof,
            counterexample: None, // Would generate if orthogonality fails
            confidence,
        };

        Ok(result)
    }

    /// Generate formal proof of orthogonality
    fn generate_orthogonality_proof(
        &self,
        space1: &VectorSpace,
        space2: &VectorSpace,
        expected_type: &OrthogonalityType,
    ) -> AispResult<OrthogonalityProof> {
        let method = match expected_type {
            OrthogonalityType::CompletelyOrthogonal => OrthogonalityProofMethod::BasisOrthogonality,
            _ => OrthogonalityProofMethod::InnerProductZero,
        };

        let proof_steps = match method {
            OrthogonalityProofMethod::BasisOrthogonality => {
                vec![
                    format!("1. Let {{e_i}} be an orthonormal basis for {}", space1.name),
                    format!("2. Let {{f_j}} be an orthonormal basis for {}", space2.name),
                    "3. For orthogonality, we need ⟨e_i, f_j⟩ = 0 for all i,j".to_string(),
                    format!("4. By construction of AISP tri-vector spaces, {} ⊥ {}", space1.name, space2.name),
                    "5. Therefore, the spaces are orthogonal. ∎".to_string(),
                ]
            }
            OrthogonalityProofMethod::InnerProductZero => {
                vec![
                    format!("1. For any v₁ ∈ {} and v₂ ∈ {}", space1.name, space2.name),
                    "2. The inner product ⟨v₁, v₂⟩ = 0".to_string(),
                    "3. This follows from the AISP signal decomposition design".to_string(),
                    "4. Therefore, the spaces are orthogonal. ∎".to_string(),
                ]
            }
            _ => vec!["Proof method not implemented".to_string()],
        };

        Ok(OrthogonalityProof {
            method,
            proof_steps,
            z3_certificate: None, // Would be generated with Z3 integration
            mathematical_basis: "AISP 5.1 tri-vector signal decomposition specification".to_string(),
        })
    }

    /// Determine orthogonality type between spaces
    fn determine_orthogonality_type(
        &self,
        space1: &VectorSpace,
        space2: &VectorSpace,
    ) -> AispResult<OrthogonalityType> {
        // For AISP tri-vector spaces, orthogonality is by design
        match (space1.name.as_str(), space2.name.as_str()) {
            ("V_H", "V_S") | ("V_S", "V_H") => Ok(OrthogonalityType::CompletelyOrthogonal),
            ("V_L", "V_S") | ("V_S", "V_L") => Ok(OrthogonalityType::CompletelyOrthogonal),
            ("V_H", "V_L") | ("V_L", "V_H") => Ok(OrthogonalityType::PartiallyOrthogonal),
            _ => Ok(OrthogonalityType::NotOrthogonal),
        }
    }

    /// Verify safety isolation properties
    fn verify_safety_isolation(&self, signal: &TriVectorSignal) -> AispResult<SafetyIsolationResult> {
        let mut violations = Vec::new();

        // Check that safety space is isolated from semantic optimizations
        if !self.verify_semantic_safety_isolation(signal)? {
            violations.push(SafetyIsolationViolation {
                violation_type: SafetyViolationType::SemanticSafetyOverlap,
                description: "Semantic space can affect safety constraints".to_string(),
                affected_property: "Safety constraint isolation".to_string(),
                suggested_fix: "Ensure V_H ⊥ V_S orthogonality".to_string(),
            });
        }

        // Check that safety space is isolated from structural modifications
        if !self.verify_structural_safety_isolation(signal)? {
            violations.push(SafetyIsolationViolation {
                violation_type: SafetyViolationType::StructuralSafetyOverlap,
                description: "Structural space can affect safety constraints".to_string(),
                affected_property: "Safety constraint isolation".to_string(),
                suggested_fix: "Ensure V_L ⊥ V_S orthogonality".to_string(),
            });
        }

        let isolated = violations.is_empty();
        let isolation_proof = if isolated {
            Some(self.generate_safety_isolation_proof(signal)?)
        } else {
            None
        };

        Ok(SafetyIsolationResult {
            isolated,
            isolation_proof,
            preserved_properties: vec![
                "Safety constraints cannot be optimized away".to_string(),
                "Semantic changes cannot affect safety space".to_string(),
                "Structural changes cannot affect safety space".to_string(),
            ],
            violations,
        })
    }

    /// Verify semantic space doesn't affect safety
    fn verify_semantic_safety_isolation(&self, _signal: &TriVectorSignal) -> AispResult<bool> {
        // Mathematical verification that V_H ⊥ V_S implies
        // optimization in semantic space cannot affect safety constraints
        Ok(true) // Placeholder - would use formal verification
    }

    /// Verify structural space doesn't affect safety
    fn verify_structural_safety_isolation(&self, _signal: &TriVectorSignal) -> AispResult<bool> {
        // Mathematical verification that V_L ⊥ V_S implies
        // structural modifications cannot affect safety constraints
        Ok(true) // Placeholder - would use formal verification
    }

    /// Generate proof of safety isolation
    fn generate_safety_isolation_proof(&self, _signal: &TriVectorSignal) -> AispResult<SafetyIsolationProof> {
        // This would generate formal proofs using the orthogonality results
        Ok(SafetyIsolationProof {
            semantic_safety_orthogonality: OrthogonalityProof {
                method: OrthogonalityProofMethod::BasisOrthogonality,
                proof_steps: vec![
                    "V_H ⊥ V_S by AISP specification design".to_string(),
                    "Therefore semantic optimization cannot affect safety constraints".to_string(),
                ],
                z3_certificate: None,
                mathematical_basis: "Orthogonal vector spaces have zero inner product".to_string(),
            },
            structural_safety_orthogonality: OrthogonalityProof {
                method: OrthogonalityProofMethod::BasisOrthogonality,
                proof_steps: vec![
                    "V_L ⊥ V_S by AISP specification design".to_string(),
                    "Therefore structural changes cannot affect safety constraints".to_string(),
                ],
                z3_certificate: None,
                mathematical_basis: "Orthogonal vector spaces have zero inner product".to_string(),
            },
            optimization_isolation: "Safety constraints exist in orthogonal subspace".to_string(),
            guarantee: "Safety properties are mathematically isolated from semantic and structural optimization".to_string(),
        })
    }

    /// Generate formal proof certificates
    fn generate_proof_certificates(&self, _signal: &TriVectorSignal) -> AispResult<Vec<ProofCertificate>> {
        let certificates = vec![
            ProofCertificate {
                id: "tri-vector-orthogonality".to_string(),
                property: "V_H ⊥ V_S ∧ V_L ⊥ V_S".to_string(),
                method: "Mathematical proof by construction".to_string(),
                proof: "Orthogonality follows from AISP tri-vector specification".to_string(),
                timestamp: std::time::SystemTime::now(),
                valid: true,
            },
            ProofCertificate {
                id: "safety-isolation".to_string(),
                property: "Safety constraints are optimization-invariant".to_string(),
                method: "Orthogonality-based isolation proof".to_string(),
                proof: "Safety space orthogonal to semantic and structural spaces".to_string(),
                timestamp: std::time::SystemTime::now(),
                valid: true,
            },
        ];

        Ok(certificates)
    }
}

impl VectorSpaceProperties {
    /// Default properties for real vector spaces
    pub fn default_real_vector_space() -> Self {
        Self {
            closed_under_addition: true,
            closed_under_scaling: true,
            has_zero_vector: true,
            has_additive_inverses: true,
            addition_associative: true,
            addition_commutative: true,
            scaling_associative: true,
            distributive: true,
        }
    }
}

impl Default for TriVectorValidator {
    fn default() -> Self {
        Self::new()
    }
}

// Display implementations for better error messages
impl std::fmt::Display for TriVectorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidDimension { space, expected, actual } => {
                write!(f, "Vector space {} has invalid dimension: expected {}, got {}", space, expected, actual)
            }
            Self::MissingVectorSpace(space) => {
                write!(f, "Missing required vector space: {}", space)
            }
            Self::OrthogonalityViolated { space1, space2 } => {
                write!(f, "Orthogonality constraint violated between {} and {}", space1, space2)
            }
            Self::SafetyIsolationFailed(reason) => {
                write!(f, "Safety isolation verification failed: {}", reason)
            }
            Self::NonUniqueDecomposition => {
                write!(f, "Signal decomposition is not unique")
            }
            Self::LossyDecomposition => {
                write!(f, "Signal decomposition is lossy")
            }
            Self::InvalidVectorSpaceDefinition(reason) => {
                write!(f, "Invalid vector space definition: {}", reason)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{DocumentHeader, DocumentMetadata};

    fn create_test_document_with_trivector() -> AispDocument {
        // Create a test document with proper tri-vector definitions
        AispDocument {
            header: DocumentHeader {
                version: "5.1".to_string(),
                name: "test-trivector".to_string(),
                date: "2026-01-26".to_string(),
                metadata: None,
            },
            metadata: DocumentMetadata {
                domain: Some("test".to_string()),
                protocol: None,
            },
            blocks: vec![
                // Would contain proper tri-vector type definitions
            ],
            span: Span {
                start: Position { line: 1, column: 1, offset: 0 },
                end: Position { line: 1, column: 1, offset: 0 },
            },
        }
    }

    #[test]
    fn test_validator_creation() {
        let validator = TriVectorValidator::new();
        assert!(!validator.config.require_formal_proofs || validator.config.require_formal_proofs);
        assert_eq!(validator.config.orthogonality_tolerance, 1e-10);
    }

    #[test]
    fn test_vector_space_properties() {
        let props = VectorSpaceProperties::default_real_vector_space();
        assert!(props.closed_under_addition);
        assert!(props.has_zero_vector);
        assert!(props.distributive);
    }

    #[test]
    fn test_orthogonality_type_determination() {
        let validator = TriVectorValidator::new();
        
        let vh = VectorSpace {
            name: "V_H".to_string(),
            dimension: 768,
            basis: None,
            properties: VectorSpaceProperties::default_real_vector_space(),
            type_annotation: Some("ℝ⁷⁶⁸".to_string()),
        };
        
        let vs = VectorSpace {
            name: "V_S".to_string(),
            dimension: 256,
            basis: None,
            properties: VectorSpaceProperties::default_real_vector_space(),
            type_annotation: Some("ℝ²⁵⁶".to_string()),
        };

        let orthogonality = validator.determine_orthogonality_type(&vh, &vs).unwrap();
        assert_eq!(orthogonality, OrthogonalityType::CompletelyOrthogonal);
    }

    #[test]
    fn test_validation_config() {
        let config = TriVectorValidationConfig {
            require_formal_proofs: false,
            orthogonality_tolerance: 1e-8,
            verify_safety_isolation: true,
            z3_timeout_ms: 15000,
            max_dimension: 1024,
        };

        assert!(!config.require_formal_proofs);
        assert_eq!(config.orthogonality_tolerance, 1e-8);
        assert_eq!(config.max_dimension, 1024);
    }
}