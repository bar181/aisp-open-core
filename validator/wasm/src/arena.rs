//! Arena allocator for zero-allocation memory management
//!
//! Provides static, bump-allocated memory for term storage
//! without requiring a heap allocator.

/// Fixed-size arena allocator
#[repr(C, align(8))]
pub struct Arena<const N: usize> {
    /// Storage buffer
    buffer: [u8; N],
    /// Current allocation head
    head: usize,
}

impl<const N: usize> Arena<N> {
    /// Create new empty arena
    pub const fn new() -> Self {
        Self {
            buffer: [0; N],
            head: 0,
        }
    }

    /// Allocate bytes with alignment
    #[inline]
    pub fn alloc_bytes(&mut self, size: usize, align: usize) -> Option<*mut u8> {
        let aligned = (self.head + align - 1) & !(align - 1);
        let new_head = aligned + size;

        if new_head > N {
            return None;
        }

        self.head = new_head;
        Some(unsafe { self.buffer.as_mut_ptr().add(aligned) })
    }

    /// Allocate typed value
    #[inline]
    pub fn alloc<T>(&mut self) -> Option<&mut T> {
        let ptr = self.alloc_bytes(
            core::mem::size_of::<T>(),
            core::mem::align_of::<T>(),
        )?;
        Some(unsafe { &mut *(ptr as *mut T) })
    }

    /// Get current usage in bytes
    #[inline]
    pub fn used(&self) -> usize {
        self.head
    }

    /// Get remaining capacity
    #[inline]
    pub fn remaining(&self) -> usize {
        N - self.head
    }

    /// Reset arena (deallocate all)
    #[inline]
    pub fn reset(&mut self) {
        self.head = 0;
        // Zero out for security
        self.buffer.iter_mut().for_each(|b| *b = 0);
    }

    /// Get slice of allocated data
    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        &self.buffer[..self.head]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arena_alloc() {
        let mut arena: Arena<256> = Arena::new();

        let p1 = arena.alloc::<u32>().unwrap();
        *p1 = 42;

        let p2 = arena.alloc::<u64>().unwrap();
        *p2 = 0xDEADBEEF;

        assert_eq!(*p1, 42);
        assert_eq!(*p2, 0xDEADBEEF);
    }

    #[test]
    fn test_arena_full() {
        let mut arena: Arena<8> = Arena::new();

        let _ = arena.alloc::<u64>().unwrap();
        assert!(arena.alloc::<u64>().is_none());
    }
}
