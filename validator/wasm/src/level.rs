//! Universe levels for predicative type theory
//!
//! Implements the universe hierarchy: Prop : Type₀ : Type₁ : ...
//! Based on lean-agentic's level module.

use crate::symbol::SymbolId;

/// Level identifier (2 bytes)
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct LevelId(pub u16);

impl LevelId {
    pub const ZERO: Self = Self(0);
    pub const ONE: Self = Self(1);

    #[inline]
    pub const fn new(idx: u16) -> Self {
        Self(idx)
    }

    #[inline]
    pub const fn index(self) -> usize {
        self.0 as usize
    }
}

/// Level kind discriminant
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum LevelKind {
    /// Zero level
    Zero = 0,
    /// Successor level
    Succ = 1,
    /// Maximum of two levels
    Max = 2,
    /// Impredicative maximum
    IMax = 3,
    /// Universe parameter
    Param = 4,
}

/// Level representation (8 bytes)
#[repr(C)]
pub struct Level {
    /// Level kind
    pub kind: LevelKind,
    /// Padding
    _pad: [u8; 3],
    /// Kind-specific data
    pub data: LevelData,
}

impl Level {
    pub const ZERO: Self = Self {
        kind: LevelKind::Zero,
        _pad: [0; 3],
        data: LevelData { zero: () },
    };
}

/// Level data union (4 bytes)
#[repr(C)]
#[derive(Clone, Copy)]
pub union LevelData {
    /// Zero: no data
    pub zero: (),
    /// Succ: predecessor level
    pub succ: LevelId,
    /// Max: (left, right)
    pub max: [LevelId; 2],
    /// IMax: (left, right)
    pub imax: [LevelId; 2],
    /// Param: parameter name
    pub param: SymbolId,
}

/// Level arena for storage
pub struct LevelArena {
    levels: [Level; 32],
    len: u8,
}

impl LevelArena {
    pub const fn new() -> Self {
        Self {
            levels: [Level::ZERO; 32],
            len: 2, // Reserve 0=Zero, 1=Succ(Zero)
        }
    }

    /// Create successor level
    pub fn mk_succ(&mut self, pred: LevelId) -> Option<LevelId> {
        if self.len >= 32 {
            return None;
        }
        let id = LevelId(self.len as u16);
        self.levels[self.len as usize] = Level {
            kind: LevelKind::Succ,
            _pad: [0; 3],
            data: LevelData { succ: pred },
        };
        self.len += 1;
        Some(id)
    }

    /// Create max level
    pub fn mk_max(&mut self, l1: LevelId, l2: LevelId) -> Option<LevelId> {
        if self.len >= 32 {
            return None;
        }
        let id = LevelId(self.len as u16);
        self.levels[self.len as usize] = Level {
            kind: LevelKind::Max,
            _pad: [0; 3],
            data: LevelData { max: [l1, l2] },
        };
        self.len += 1;
        Some(id)
    }

    /// Create imax level
    pub fn mk_imax(&mut self, l1: LevelId, l2: LevelId) -> Option<LevelId> {
        if self.len >= 32 {
            return None;
        }
        let id = LevelId(self.len as u16);
        self.levels[self.len as usize] = Level {
            kind: LevelKind::IMax,
            _pad: [0; 3],
            data: LevelData { imax: [l1, l2] },
        };
        self.len += 1;
        Some(id)
    }

    /// Get level by ID
    pub fn get(&self, id: LevelId) -> &Level {
        &self.levels[id.index()]
    }

    /// Reset arena
    pub fn reset(&mut self) {
        self.len = 2;
    }
}

/// Check if two levels are equal
pub fn level_eq(arena: &LevelArena, l1: LevelId, l2: LevelId) -> bool {
    if l1 == l2 {
        return true;
    }

    let lv1 = arena.get(l1);
    let lv2 = arena.get(l2);

    if lv1.kind != lv2.kind {
        return false;
    }

    match lv1.kind {
        LevelKind::Zero => true,
        LevelKind::Succ => unsafe {
            level_eq(arena, lv1.data.succ, lv2.data.succ)
        },
        LevelKind::Max | LevelKind::IMax => unsafe {
            let d1 = if lv1.kind == LevelKind::Max { lv1.data.max } else { lv1.data.imax };
            let d2 = if lv2.kind == LevelKind::Max { lv2.data.max } else { lv2.data.imax };
            level_eq(arena, d1[0], d2[0]) && level_eq(arena, d1[1], d2[1])
        },
        LevelKind::Param => unsafe {
            lv1.data.param == lv2.data.param
        },
    }
}

// Compile-time size assertions
const _: () = assert!(core::mem::size_of::<Level>() == 8);
const _: () = assert!(core::mem::size_of::<LevelId>() == 2);
