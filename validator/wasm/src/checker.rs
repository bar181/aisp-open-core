//! Type checker for AISP terms
//!
//! Implements the trusted kernel for type checking based on
//! lean-agentic's type theory.

use crate::term::{Term, TermId, TermKind};
use crate::level::{LevelArena, level_eq};
use crate::symbol::SymbolId;

/// Type checking error codes
#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TypeError {
    /// Success
    Ok = 0,
    /// Type mismatch
    TypeMismatch = -10,
    /// Variable out of scope
    VarOutOfScope = -11,
    /// Expected function type
    ExpectedPi = -12,
    /// Expected sort
    ExpectedSort = -13,
    /// Unknown constant
    UnknownConst = -14,
    /// Context overflow
    ContextOverflow = -15,
}

/// Typing context entry
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ContextEntry {
    /// Variable name
    pub name: SymbolId,
    /// Variable type
    pub ty: TermId,
    /// Optional value (for let-bindings)
    pub value: TermId,
}

impl ContextEntry {
    pub const EMPTY: Self = Self {
        name: SymbolId::ANONYMOUS,
        ty: TermId::NULL,
        value: TermId::NULL,
    };
}

/// Type checking context
pub struct Context {
    /// Local variable entries
    entries: [ContextEntry; 32],
    /// Current depth
    depth: u8,
}

impl Context {
    /// Create empty context
    pub const fn new() -> Self {
        Self {
            entries: [ContextEntry::EMPTY; 32],
            depth: 0,
        }
    }

    /// Extend context with new variable
    pub fn extend(&mut self, name: SymbolId, ty: TermId) -> Result<(), TypeError> {
        if self.depth >= 32 {
            return Err(TypeError::ContextOverflow);
        }
        self.entries[self.depth as usize] = ContextEntry {
            name,
            ty,
            value: TermId::NULL,
        };
        self.depth += 1;
        Ok(())
    }

    /// Extend with let-binding
    pub fn define(&mut self, name: SymbolId, ty: TermId, val: TermId) -> Result<(), TypeError> {
        if self.depth >= 32 {
            return Err(TypeError::ContextOverflow);
        }
        self.entries[self.depth as usize] = ContextEntry { name, ty, value: val };
        self.depth += 1;
        Ok(())
    }

    /// Get type at de Bruijn index
    pub fn get_type(&self, idx: u16) -> Result<TermId, TypeError> {
        let i = self.depth.checked_sub(1 + idx as u8)
            .ok_or(TypeError::VarOutOfScope)?;
        Ok(self.entries[i as usize].ty)
    }

    /// Get value at de Bruijn index (for let-bindings)
    pub fn get_value(&self, idx: u16) -> Option<TermId> {
        let i = self.depth.checked_sub(1 + idx as u8)?;
        let v = self.entries[i as usize].value;
        if v.is_null() { None } else { Some(v) }
    }

    /// Pop from context
    pub fn pop(&mut self) {
        if self.depth > 0 {
            self.depth -= 1;
        }
    }

    /// Reset context
    pub fn reset(&mut self) {
        self.depth = 0;
    }
}

/// Type checker state
pub struct TypeChecker<'a> {
    /// Term storage
    terms: &'a [Term],
    /// Level storage
    levels: &'a LevelArena,
    /// Typing context
    ctx: Context,
}

impl<'a> TypeChecker<'a> {
    /// Create new type checker
    pub fn new(terms: &'a [Term], levels: &'a LevelArena) -> Self {
        Self {
            terms,
            levels,
            ctx: Context::new(),
        }
    }

    /// Get term by ID
    #[inline]
    fn get(&self, id: TermId) -> &Term {
        &self.terms[id.index()]
    }

    /// Infer type of term
    pub fn infer(&mut self, t: TermId) -> Result<TermId, TypeError> {
        let term = self.get(t);

        match term.kind {
            TermKind::Sort => {
                // Sort(l) : Sort(succ(l))
                Ok(TermId::TYPE) // Simplified: all sorts have Type as type
            }

            TermKind::Var => {
                // Var(i) : ctx[i]
                let idx = unsafe { term.data.var_idx };
                self.ctx.get_type(idx)
            }

            TermKind::Const => {
                // Would look up in environment
                // For now, return TYPE
                Ok(TermId::TYPE)
            }

            TermKind::App => {
                // App(f, a) : subst(B, a) where f : Pi(A, B) and a : A
                let [func, arg] = unsafe { term.data.app };
                let fn_ty = self.infer(func)?;

                // fn_ty should be Pi
                let fn_term = self.get(fn_ty);
                if fn_term.kind != TermKind::Pi {
                    return Err(TypeError::ExpectedPi);
                }

                let binder = unsafe { fn_term.data.binder };
                self.check(arg, binder.ty)?;

                // Return body (simplified, no actual substitution)
                Ok(binder.body)
            }

            TermKind::Lam => {
                // Lam(x:A, b) : Pi(x:A, infer(b))
                let binder = unsafe { term.data.binder };
                self.ctx.extend(binder.name, binder.ty)?;
                let body_ty = self.infer(binder.body)?;
                self.ctx.pop();

                // Would construct Pi type here
                // Simplified: return body type
                Ok(body_ty)
            }

            TermKind::Pi => {
                // Pi(x:A, B) : Sort(imax(level(A), level(B)))
                Ok(TermId::TYPE)
            }

            TermKind::Let => {
                // Let(x:A=v, b) : infer(b[x:=v])
                let data = unsafe { term.data.let_data };
                self.check(data.val, data.ty)?;
                self.ctx.define(data.name, data.ty, data.val)?;
                let result = self.infer(data.body);
                self.ctx.pop();
                result
            }

            TermKind::Meta => {
                // Metavariable - would be resolved by unification
                Ok(TermId::TYPE)
            }
        }
    }

    /// Check term has given type
    pub fn check(&mut self, t: TermId, expected: TermId) -> Result<(), TypeError> {
        let inferred = self.infer(t)?;

        if self.conv(inferred, expected) {
            Ok(())
        } else {
            Err(TypeError::TypeMismatch)
        }
    }

    /// Check definitional equality
    pub fn conv(&self, t1: TermId, t2: TermId) -> bool {
        if t1 == t2 {
            return true;
        }

        let term1 = self.get(t1);
        let term2 = self.get(t2);

        if term1.kind != term2.kind {
            return false;
        }

        match term1.kind {
            TermKind::Sort => {
                let l1 = unsafe { term1.data.level };
                let l2 = unsafe { term2.data.level };
                level_eq(self.levels, l1, l2)
            }

            TermKind::Var => {
                let i1 = unsafe { term1.data.var_idx };
                let i2 = unsafe { term2.data.var_idx };
                i1 == i2
            }

            TermKind::Const => {
                let n1 = unsafe { term1.data.const_name };
                let n2 = unsafe { term2.data.const_name };
                n1 == n2
            }

            TermKind::App => {
                let [f1, a1] = unsafe { term1.data.app };
                let [f2, a2] = unsafe { term2.data.app };
                self.conv(f1, f2) && self.conv(a1, a2)
            }

            TermKind::Lam | TermKind::Pi => {
                let b1 = unsafe { term1.data.binder };
                let b2 = unsafe { term2.data.binder };
                self.conv(b1.ty, b2.ty) && self.conv(b1.body, b2.body)
            }

            TermKind::Let => {
                let d1 = unsafe { term1.data.let_data };
                let d2 = unsafe { term2.data.let_data };
                self.conv(d1.ty, d2.ty) &&
                self.conv(d1.val, d2.val) &&
                self.conv(d1.body, d2.body)
            }

            TermKind::Meta => {
                let m1 = unsafe { term1.data.meta_id };
                let m2 = unsafe { term2.data.meta_id };
                m1 == m2
            }
        }
    }
}
