use super::*;
use crate::memory::*;
//use algorithm::*;
use lazy_static::*;
use spin::Mutex;

use crate::data_structure::{*};


lazy_static! {
    /// 帧分配器
    pub static ref FRAME_ALLOCATOR: Mutex<FrameAllocator<AllocatorImpl>> = Mutex::new(FrameAllocator::new(Range::from(
            PhysicalPageNumber::ceil(PhysicalAddress::from(PhysicalAddress((*KERNEL_END_ADDRESS).0&0xFFFFF000)))..PhysicalPageNumber::floor(PhysicalAddress(MEMORY_END_ADDRESS.0&0xFFFFF000)))));
}

/// 基于线段树的帧分配 / 回收
pub struct FrameAllocator<T: Allocator> {
    /// 可用区间的起始
    start_ppn: PhysicalPageNumber,
    /// 分配器
    allocator: T,
}

impl<T: Allocator> FrameAllocator<T> {
    /// 创建对象
    pub fn new(range: impl Into<Range<PhysicalPageNumber>> + Copy) -> Self {
        FrameAllocator {
            start_ppn: range.into().start,
            allocator: T::new(range.into().len()),
        }
    }

    /// 分配帧，如果没有剩余则返回 `Err`
    pub fn alloc(&mut self) -> MemoryResult<FrameTracker> {
        self.alloc_frames(1)
    }
    pub fn alloc_frames(&mut self,num:usize) -> MemoryResult<FrameTracker> {
        self.allocator
            .alloc_frames(num)
            .ok_or("no available frame to allocate")
            .map(|offset| FrameTracker::from(FrameTracker(self.start_ppn + offset,num)))
    }

    /// 将被释放的帧添加到空闲列表的尾部
    ///
    /// 这个函数会在 [`FrameTracker`] 被 drop 时自动调用，不应在其他地方调用
    pub(super) fn dealloc(&mut self, frame: &FrameTracker) {
        self.allocator.dealloc((frame.page_number() - self.start_ppn),frame.page_num());
    }
}
