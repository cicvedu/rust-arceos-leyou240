//! Simple memory allocation.
//!
//! TODO: more efficient

use core::alloc::Layout;
use core::num::NonZeroUsize;

use crate::{AllocResult, BaseAllocator, ByteAllocator};

pub struct SimpleByteAllocator {
    start: usize,
    next: usize,
    allocations: usize,
    end: usize,
}

impl SimpleByteAllocator {
    pub const fn new() -> Self {
        Self {
            start: 0,
            next: 0,
            allocations: 0,
            end: 0,
        }
    }
}

impl BaseAllocator for SimpleByteAllocator {
    fn init(&mut self, _start: usize, _size: usize) {
        self.start = _start;
        self.next = self.start;
        self.end = self.start + _size;
        self.allocations = 0;
    }

    fn add_memory(&mut self, _start: usize, _size: usize) -> AllocResult {
        todo!();
    }
}

impl ByteAllocator for SimpleByteAllocator {
    fn alloc(&mut self, layout: Layout) -> AllocResult<NonZeroUsize> {
        let size = layout.size();
        let align = layout.align();
        let align_mask = !(align - 1);

        let start = (self.next + align - 1) & align_mask;

        if start + size > self.end {
            Err(crate::AllocError::NoMemory)
        } else {
            self.allocations += 1;
            self.next = start + size;
            Ok(NonZeroUsize::new(start).unwrap())
        }
    }

    fn dealloc(&mut self, _: NonZeroUsize, _: Layout) {
        self.allocations -= 1;
        if self.allocations == 0 {
            self.next = self.start;
        }
    }

    fn total_bytes(&self) -> usize {
        self.end - self.start
    }

    fn used_bytes(&self) -> usize {
        self.next - self.start
    }

    fn available_bytes(&self) -> usize {
        self.end - self.next
    }
}
