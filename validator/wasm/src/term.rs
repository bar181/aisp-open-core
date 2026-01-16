//! Term representation for dependent type theory
//!
//! Based on lean-agentic's term module, optimized for size.
//! Uses packed 16-byte representation with hash-consing.

use crate::level::LevelId;
use crate::symbol::SymbolId;

/// Term identifier (2 bytes, max 65535 terms)
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct TermId(pub u16);

impl TermId {
    pub const NULL: Self = Self(0xFFFF);
    pub const PROP: Self = Self(0);  // Sort(Level::Zero)
    pub const TYPE: Self = Self(1);  // Sort(Level::Succ(Zero))

    #[inline]
    pub const fn new(idx: u16) -> Self {
        Self(idx)
    }

    #[inline]
    pub const fn index(self) -> usize {
        self.0 as usize
    }

    #[inline]
    pub const fn is_null(self) -> bool {
        self.0 == 0xFFFF
    }
}

/// Term kind discriminant (1 byte)
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TermKind {
    /// Type universe: Sort(Level)
    Sort = 0,
    /// Bound variable: Var(de_bruijn_index)
    Var = 1,
    /// Global constant: Const(name, levels)
    Const = 2,
    /// Application: App(fn, arg)
    App = 3,
    /// Lambda: Lam(binder, body)
    Lam = 4,
    /// Dependent function type: Pi(binder, body)
    Pi = 5,
    /// Let binding: Let(binder, value, body)
    Let = 6,
    /// Metavariable (for unification)
    Meta = 7,
}

/// Binder info (1 byte)
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BinderInfo {
    /// Explicit argument
    Default = 0,
    /// Implicit argument {}
    Implicit = 1,
    /// Instance argument []
    Instance = 2,
    /// Strict implicit {{}}
    StrictImplicit = 3,
}

/// Term flags (1 byte)
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct TermFlags(pub u8);

impl TermFlags {
    pub const NONE: Self = Self(0);
    pub const HAS_FVARS: Self = Self(1 << 0);
    pub const HAS_MVARS: Self = Self(1 << 1);
    pub const IS_CLOSED: Self = Self(1 << 2);
    pub const IS_WHNF: Self = Self(1 << 3);
}

/// Core term representation (16 bytes)
#[repr(C, align(8))]
pub struct Term {
    /// Term kind
    pub kind: TermKind,
    /// Flags
    pub flags: TermFlags,
    /// Padding
    _pad: [u8; 2],
    /// Kind-specific data
    pub data: TermData,
}

impl Term {
    pub const UNINIT: Self = Self {
        kind: TermKind::Sort,
        flags: TermFlags::NONE,
        _pad: [0; 2],
        data: TermData { level: LevelId(0) },
    };
}

/// Term data union (12 bytes)
#[repr(C)]
#[derive(Clone, Copy)]
pub union TermData {
    /// Sort: universe level
    pub level: LevelId,
    /// Var: de Bruijn index
    pub var_idx: u16,
    /// Const: name symbol
    pub const_name: SymbolId,
    /// App: (function, argument)
    pub app: [TermId; 2],
    /// Lam/Pi: binder data
    pub binder: BinderData,
    /// Let: let data
    pub let_data: LetData,
    /// Meta: metavariable id
    pub meta_id: u16,
}

/// Binder data (8 bytes)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct BinderData {
    /// Binder name
    pub name: SymbolId,
    /// Binder info
    pub info: BinderInfo,
    /// Padding
    _pad: u8,
    /// Binder type
    pub ty: TermId,
    /// Body term
    pub body: TermId,
}

/// Let binding data (8 bytes)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct LetData {
    /// Variable name
    pub name: SymbolId,
    /// Variable type
    pub ty: TermId,
    /// Bound value
    pub val: TermId,
    /// Body
    pub body: TermId,
}

// Compile-time size assertions
const _: () = assert!(core::mem::size_of::<Term>() == 16);
const _: () = assert!(core::mem::size_of::<TermId>() == 2);
const _: () = assert!(core::mem::size_of::<BinderData>() == 8);
