# ADR 006: Garden-Inspired Formal Verification Methodology

## Status
Accepted - 2025-01-26

## Context

After examining the [Garden framework](https://github.com/formal-land/garden) for formally verifying zero-knowledge circuits, we identified several sophisticated patterns that can significantly enhance our AISP formal verification approach. Garden successfully verifies complex cryptographic circuits (Keccak, Blake3, Sha256) with three key properties:

1. **Determinism**: Only one possible execution trace for each input
2. **Functional Correctness**: The system computes the correct output according to specification  
3. **Completeness**: The system never blocks or enters undefined states

Garden's methodology provides excellent patterns for:
- **Modular verification** - Breaking complex proofs into manageable chunks
- **Specification/Implementation separation** - Clear `Spec.t` modules with `implies` lemmas
- **Field arithmetic reasoning** - Proper handling of modular arithmetic with `Prime` typeclass
- **Monadic proof structure** - Using `M.t` monad for proof composition

## Decision

We will adopt **Garden-inspired verification methodology** for AISP validation with:

### 1. Three-Property Verification Framework

Following Garden's model, we will verify AISP documents for:

```rust
/// Core verification properties for AISP documents
pub enum AispVerificationProperty {
    /// Determinism: Only one valid interpretation per document
    Determinism,
    /// Functional Correctness: Validates according to AISP specification
    FunctionalCorrectness, 
    /// Completeness: Validation always terminates with definitive result
    Completeness,
}
```

### 2. Modular Specification Pattern

Adopt Garden's `Module.Spec.t` + `implies` lemma pattern:

```rust
// Generated Rocq code following Garden patterns
pub fn generate_aisp_verification_module(&self, property: &AispProperty) -> String {
    format!(r#"
Module {module_name}.
  Definition eval {{p}} `{{Prime p}} (doc : AispDocument) : M.t unit :=
    {evaluation_logic}

  Module Spec.
    Definition t (doc : AispDocument) : Prop :=
      {property_specification}
  End Spec.

  Lemma implies {{p}} `{{Prime p}} (doc' : AispDocument) :
    let doc := M.map_mod doc' in
    {{{{ eval doc 🔽
      tt,
      Spec.t doc
    }}}}.
  Proof.
    intros.
    unfold eval.
    {{ eapply Run.Implies. {{
        Run.run.
      }}
      unfold Spec.t; cbn.
      hauto l: on.
    }}
  Qed.
End {module_name}.
"#, 
    module_name = property.module_name(),
    evaluation_logic = self.generate_evaluation_logic(property),
    property_specification = self.generate_property_spec(property)
    )
}
```

### 3. AISP-Specific Verification Modules

Create Garden-style verification modules for each AISP block type:

```coq
(* Meta block verification *)
Module MetaBlockVerification.
  Definition eval {p} `{Prime p} (meta : MetaBlock) : M.t unit :=
    M.when (meta.(version_major) =? 5) (
      M.assert (meta.(version_minor) >=? 0)
    ).

  Module Spec.
    Definition t (meta : MetaBlock) : Prop :=
      meta.(version_major) = 5 /\ meta.(version_minor) >= 0.
  End Spec.

  Lemma implies {p} `{Prime p} (meta' : MetaBlock) :
    let meta := M.map_mod meta' in
    {{ eval meta 🔽 tt, Spec.t meta }}.
  (* Proof following Garden patterns *)
End MetaBlockVerification.

(* Types block verification *)  
Module TypesBlockVerification.
  Definition eval {p} `{Prime p} (types : TypesBlock) : M.t unit :=
    M.for_each types.(type_definitions) (fun typedef =>
      M.assert_well_formed_type typedef
    ).

  Module Spec.
    Definition t (types : TypesBlock) : Prop :=
      forall typedef, In typedef types.(type_definitions) ->
        WellFormedType typedef.
  End Spec.
  (* ... implies lemma ... *)
End TypesBlockVerification.

(* Rules block verification *)
Module RulesBlockVerification.
  Definition eval {p} `{Prime p} (rules : RulesBlock) : M.t unit :=
    M.for_each rules.(rule_definitions) (fun rule =>
      M.assert_satisfiable_rule rule
    ).

  Module Spec.  
    Definition t (rules : RulesBlock) : Prop :=
      forall rule, In rule rules.(rule_definitions) ->
        SatisfiableRule rule.
  End Spec.
  (* ... implies lemma ... *)
End RulesBlockVerification.
```

### 4. Monadic Proof Composition

Adopt Garden's monadic approach for composing verification proofs:

```rust
impl AispVerificationFramework {
    pub fn generate_compositional_proof(&self, doc: &AispDocument) -> String {
        r#"
Definition verify_aisp_document {p} `{Prime p} (doc : AispDocument) : M.t unit :=
  match doc with
  | MkAispDocument meta types rules funcs evidence =>
      M.do_unit (MetaBlockVerification.eval meta) ;;
      M.do_unit (TypesBlockVerification.eval types) ;;  
      M.do_unit (RulesBlockVerification.eval rules) ;;
      M.do_unit (FuncsBlockVerification.eval funcs) ;;
      M.do_unit (EvidenceBlockVerification.eval evidence)
  end.

Module DocumentSpec.
  Definition t (doc : AispDocument) : Prop :=
    match doc with  
    | MkAispDocument meta types rules funcs evidence =>
        MetaBlockVerification.Spec.t meta /\
        TypesBlockVerification.Spec.t types /\
        RulesBlockVerification.Spec.t rules /\
        FuncsBlockVerification.Spec.t funcs /\
        EvidenceBlockVerification.Spec.t evidence
    end.
End DocumentSpec.

Theorem aisp_document_correctness {p} `{Prime p} (doc' : AispDocument) :
  let doc := M.map_mod doc' in
  {{ verify_aisp_document doc 🔽 tt, DocumentSpec.t doc }}.
Proof.
  intros.
  destruct doc as [meta types rules funcs evidence].
  unfold verify_aisp_document, DocumentSpec.t.
  
  (* Apply Garden-style sequential verification *)
  eapply Run.Seq. {
    apply MetaBlockVerification.implies.
  }
  eapply Run.Seq. {
    apply TypesBlockVerification.implies.
  }
  eapply Run.Seq. {
    apply RulesBlockVerification.implies.  
  }
  eapply Run.Seq. {
    apply FuncsBlockVerification.implies.
  }
  apply EvidenceBlockVerification.implies.
Qed.
"#
    }
}
```

### 5. Field Arithmetic Integration

Adopt Garden's field arithmetic patterns for AISP numeric validation:

```rust
impl AispFieldArithmetic {
    pub fn generate_field_constraints(&self) -> String {
        r#"
(* AISP numeric constraints with field arithmetic *)
Module AispArithmetic.
  (* Natural number constraints in AISP *)
  Definition verify_natural_constraint {p} `{Prime p} 
      (value : Z) (constraint : NaturalConstraint) : M.t unit :=
    match constraint with
    | Range min_val max_val =>
        M.assert (min_val <=? value) ;;
        M.assert (value <=? max_val)
    | Positive =>
        M.assert (0 <? value)
    | NonZero =>
        M.assert (value <>? 0)
    end.

  (* Set cardinality constraints *)
  Definition verify_set_constraint {p} `{Prime p}
      (set_size : Z) (constraint : SetConstraint) : M.t unit :=
    match constraint with  
    | MinSize min_size =>
        M.assert (min_size <=? set_size)
    | MaxSize max_size =>
        M.assert (set_size <=? max_size)
    | ExactSize exact_size =>
        M.assert (set_size =? exact_size)
    end.
End AispArithmetic.
"#
    }
}
```

### 6. Determinism Verification Pattern

Following Garden's determinism verification approach:

```rust
pub fn generate_determinism_verification(&self) -> String {
    r#"
Module DeterminismVerification.
  (* Two parsing results of the same document must be identical *)
  Definition determinism_property (doc_text : string) : Prop :=
    forall (result1 result2 : AispDocument + ParseError),
      parse_aisp doc_text = result1 ->
      parse_aisp doc_text = result2 ->
      result1 = result2.

  (* Verification that parsing is deterministic *)
  Theorem aisp_parsing_deterministic (doc_text : string) :
    determinism_property doc_text.
  Proof.
    unfold determinism_property.
    intros result1 result2 H1 H2.
    rewrite H1 in H2.
    exact H2.
  Qed.

  (* Semantic interpretation determinism *)  
  Definition semantic_determinism (doc : AispDocument) : Prop :=
    forall (sem1 sem2 : SemanticValue),
      interpret_document doc = sem1 ->
      interpret_document doc = sem2 ->
      sem1 = sem2.

  Theorem aisp_semantics_deterministic (doc : AispDocument) :
    semantic_determinism doc.
  (* Proof using Garden-style reasoning *)
End DeterminismVerification.
"#
}
```

## Architecture Integration

```
┌─────────────────────────────────────────────────────────────┐
│              Garden-Inspired AISP Verification             │
│                                                             │
│ ┌─────────────────────────────────────────────────────────┐ │
│ │                Three-Property Framework                 │ │
│ │                                                         │ │
│ │  Determinism  │  Functional Correctness  │ Completeness │ │
│ │      ↓        │          ↓               │      ↓       │ │
│ │ Parse unique  │   Spec compliance        │ Always halts │ │
│ └─────────────────────────────────────────────────────────┘ │
│                                │                            │
│                                ▼                            │
│ ┌─────────────────────────────────────────────────────────┐ │
│ │            Modular Verification Modules                 │ │
│ │                                                         │ │
│ │ MetaBlock    TypesBlock    RulesBlock    EvidenceBlock  │ │
│ │ Verification Verification  Verification  Verification   │ │
│ │     │            │             │            │           │ │
│ │     ▼            ▼             ▼            ▼           │ │
│ │ Spec.t +     Spec.t +      Spec.t +     Spec.t +       │ │
│ │ implies      implies       implies      implies         │ │
│ └─────────────────────────────────────────────────────────┘ │
│                                │                            │
│                                ▼                            │
│ ┌─────────────────────────────────────────────────────────┐ │
│ │              Monadic Proof Composition                  │ │
│ │                                                         │ │
│ │ M.do_unit verification1 ;;                              │ │
│ │ M.do_unit verification2 ;;                              │ │
│ │ M.do_unit verification3                                 │ │
│ └─────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────┘
```

## Consequences

### Positive
- **Proven Methodology**: Garden successfully verifies complex cryptographic systems
- **Modular Proofs**: Each AISP block verified independently with clear specifications
- **Compositional Reasoning**: Monadic structure allows combining verification modules
- **Field Arithmetic**: Proper mathematical reasoning for numeric constraints
- **Clear Separation**: `Spec.t` modules separate specification from implementation
- **Reusable Patterns**: Garden patterns can be adapted across AISP validation domains

### Negative  
- **Learning Curve**: Team needs familiarity with Garden's Coq/Rocq patterns
- **Proof Complexity**: Garden-style proofs are more sophisticated than basic verification
- **Build Dependencies**: Requires Coq/Rocq environment with Garden-style tooling
- **Maintenance Overhead**: Formal specifications require careful maintenance

## Implementation Plan

### Phase 1: Core Verification Framework
- Implement three-property verification (determinism, correctness, completeness)
- Create modular verification structure with `Spec.t` + `implies` patterns
- Establish field arithmetic foundations for numeric constraints

### Phase 2: AISP Block Verification  
- Develop verification modules for each AISP block type
- Implement Garden-style monadic proof composition
- Create comprehensive test suite following Garden patterns

### Phase 3: Advanced Properties
- Add determinism verification for parsing and semantic interpretation
- Implement completeness verification ensuring termination
- Develop functional correctness proofs against AISP specification

## Garden Pattern Adoption Examples

### From Garden's Keccak Verification:
```coq
(* Garden pattern *)
Module preimage_a.
  Definition eval {p} `{Prime p} (local : KeccakCols.t) : M.t unit :=
    (* verification logic *)

  Module Spec.  
    Definition t (local : KeccakCols.t) : Prop :=
      (* specification *)
  End Spec.

  Lemma implies {p} `{Prime p} (local' : KeccakCols.t) :
    (* proof that eval implies Spec.t *)
End preimage_a.
```

### Adapted for AISP:
```coq  
(* AISP adaptation *)
Module meta_version.
  Definition eval {p} `{Prime p} (meta : MetaBlock) : M.t unit :=
    M.assert (meta.(version_major) =? 5).

  Module Spec.
    Definition t (meta : MetaBlock) : Prop :=
      meta.(version_major) = 5.
  End Spec.

  Lemma implies {p} `{Prime p} (meta' : MetaBlock) :
    let meta := M.map_mod meta' in
    {{ eval meta 🔽 tt, Spec.t meta }}.
End meta_version.
```

## Related Decisions

- [ADR 002](002-formal-methods-framework.md): Formal Methods Framework  
- [ADR 003](003-rocq-integration.md): Rocq-of-Rust Integration
- [ADR 001](001-pure-rust-architecture.md): Pure Rust Architecture