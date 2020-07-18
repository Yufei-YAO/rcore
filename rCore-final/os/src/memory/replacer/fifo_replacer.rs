
use spin::{Mutex, RwLock};
use alloc::sync::Arc;
use crate::Process;
use alloc::collections::VecDeque;
use super::Replacer;
use crate::memory::frame::FrameTracker;
pub struct FIFOReplacer{
    pub memory_record: VecDeque<(Arc<RwLock<Process>>,usize,FrameTracker)>,
    list: VecDeque<(usize, usize)>,
}

impl Replacer for FIFOReplacer{
    fn new(capacity: usize) -> Self{
        let mut list =VecDeque::new();
        list.push_front((0,capacity));
        Self{
            memory_record:VecDeque::new(),
            list:list ,
        }
    }
    fn pop_first(&mut self)->(Arc<RwLock<Process>>,usize,FrameTracker){

        self.memory_record.pop_front().unwrap()
    }
    

    fn get_free_index(&mut self)->Option<usize>{

        if let Some((start, end)) = self.list.pop_front() {
            if end - start > 8 {
                self.list.push_front((start + 8, end));
            }
            Some(start)
        } else {
            None
        }
    }
    fn put_free_index(&mut self,index:usize){
        print!("free index");
        self.list.push_front((index, index + 8));
    }
    fn put_in_memory_record(&mut self,pro:Arc<RwLock<Process>>,virtual_page:usize,frame:FrameTracker){
        if(virtual_page ==3){
            println!("page = {} ",3);
        }
        
        self.memory_record.push_back((pro,virtual_page,frame));
        //println!("record {}",virtual_page);
    }
}
