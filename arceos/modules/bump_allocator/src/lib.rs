#![no_std]

use core::ptr::NonNull;

use allocator::{AllocError, BaseAllocator, ByteAllocator, PageAllocator};

const PAGE_SIZE: usize = 0x1000;

/// Early memory allocator
/// Use it before formal bytes-allocator and pages-allocator can work!
/// This is a double-end memory range:
/// - Alloc bytes forward
/// - Alloc pages backward
///
/// [ bytes-used | avail-area | pages-used ]
/// |            | -->    <-- |            |
/// start       b_pos        p_pos       end
///
/// For bytes area, 'count' records number of allocations.
/// When it goes down to ZERO, free bytes-used area.
/// For pages area, it will never be freed!
///
pub struct EarlyAllocator<const SIZE: usize> {
    start: usize,
    end: usize,
    bpos: usize,
    ppos: usize,
    cnt: usize,
}

impl<const SIZE: usize> EarlyAllocator<SIZE> {
    pub const fn new() -> Self {
        Self {
            start: 0,
            end: 0,
            bpos: 0,
            ppos: 0,
            cnt: 0,
        }
    }
}

impl<const SIZE: usize> BaseAllocator for EarlyAllocator<SIZE> {
    fn init(&mut self, start: usize, size: usize) {
        self.start = start;
        self.end = start + size;
        self.bpos = start;
        self.ppos = start + size;
        self.cnt = 0;
    }

    fn add_memory(&mut self, start: usize, size: usize) -> allocator::AllocResult {
        unimplemented!()
    }
}

fn align_up(x: usize, align: usize) -> usize {
    // 检查 align 是否为 2 的幂（可选，用于调试）
    debug_assert!(align.is_power_of_two(), "align must be a power of two");
    debug_assert!(align != 0, "align cannot be zero");

    // 位运算公式：(x + align - 1) & !(align - 1)
    (x + align - 1) & !(align - 1)
}

/// 将 `x` 向下调整到 `align` 的倍数（`align` 必须是 2 的幂且 > 0）。
fn align_down(x: usize, align: usize) -> usize {
    // 检查 align 是否为 2 的幂（可选，用于调试）
    debug_assert!(align.is_power_of_two(), "align must be a power of two");
    debug_assert!(align != 0, "align cannot be zero");

    // 位运算公式：x & !(align - 1)
    x & !(align - 1)
}

impl<const SIZE: usize> ByteAllocator for EarlyAllocator<SIZE> {
    fn alloc(
        &mut self,
        layout: core::alloc::Layout,
    ) -> allocator::AllocResult<core::ptr::NonNull<u8>> {
        let start = align_up(self.bpos, layout.align());
        let next = start + layout.size();
        if next > self.ppos {
            return Err(allocator::AllocError::NoMemory);
        } else {
            self.bpos = next;
            self.cnt += 1;
            return NonNull::new(start as *mut u8).ok_or(AllocError::NoMemory);
        }
    }

    fn dealloc(&mut self, pos: core::ptr::NonNull<u8>, layout: core::alloc::Layout) {
        self.cnt -= 1;
        if self.cnt == 0 {
            self.bpos = self.start;
        }
    }

    fn total_bytes(&self) -> usize {
        self.end - self.start
    }

    fn used_bytes(&self) -> usize {
        self.bpos - self.start
    }

    fn available_bytes(&self) -> usize {
        self.ppos - self.bpos
    }
}

impl<const SIZE: usize> PageAllocator for EarlyAllocator<SIZE> {
    const PAGE_SIZE: usize = SIZE;

    fn alloc_pages(
        &mut self,
        num_pages: usize,
        align_pow2: usize,
    ) -> allocator::AllocResult<usize> {
        let next = align_down(self.end, align_pow2);
        if next <= self.bpos {
            return Err(AllocError::NoMemory);
        }
        self.ppos = next;
        Ok(next)
    }

    fn dealloc_pages(&mut self, pos: usize, num_pages: usize) {}

    fn total_pages(&self) -> usize {
        (self.end - self.start) / PAGE_SIZE
    }

    fn used_pages(&self) -> usize {
        (self.end - self.ppos) / PAGE_SIZE
    }

    fn available_pages(&self) -> usize {
        (self.ppos - self.bpos) / PAGE_SIZE
    }
}

