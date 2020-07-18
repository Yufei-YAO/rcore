
use spin::{Mutex, RwLock};
mod fifo_replacer;
use crate::memory::frame::FrameTracker;
use crate::Process;
use alloc::sync::Arc;
pub use fifo_replacer::FIFOReplacer;
/// 分配器：固定容量，每次分配 / 回收一个元素
pub trait Replacer {

    fn new(capacity: usize) -> Self;

    fn pop_first(&mut self)->(Arc<RwLock<Process>>,usize,FrameTracker);

    fn get_free_index(&mut self)->Option<usize>;

    fn put_free_index(&mut self,index:usize);

    fn put_in_memory_record(&mut self,pro:Arc<RwLock<Process>>,virtual_page:usize,frame:FrameTracker);
}


pub type ReplacerImpl = FIFOReplacer;