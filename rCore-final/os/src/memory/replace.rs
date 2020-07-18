
use crate::drivers::driver::Driver;
use alloc::sync::Arc;
use super::*;
use crate::memory::*;
use crate::memory::frame::FrameTracker;
use crate::memory::mapping::PageTableEntry;

use lazy_static::*;
use crate::memory::replacer::{*};
use crate::drivers::*;
use spin::{Mutex, RwLock};
use crate::Process;
use core::ops::DerefMut;
lazy_static! {
    /// 帧分配器
    pub  static  ref  FRAME_REPLACER: Mutex<FrameReplacer<ReplacerImpl>> = Mutex::new(FrameReplacer::new(10000..60000usize));
}

pub struct FrameReplacer<T:Replacer>{
    blk_device: Option<Arc<dyn Driver>>,
    start_blk: usize,
    pub replacer:T,
}
impl <T:Replacer> FrameReplacer<T>{

    pub fn new(range: core::ops::Range<usize>) -> Self {
        FrameReplacer {
            blk_device: None,
            start_blk: range.start,
            replacer: T::new(range.len()),
        }
    }
    pub fn init(&mut self,device: Arc<dyn Driver>){
        println!("replacer connect");
        self.blk_device = Some(device);
    }
    pub fn into_replacer(&mut self,process:Arc<RwLock<Process>>,vpn:VirtualPageNumber,frame:FrameTracker){
        self.replacer.put_in_memory_record(process.clone(),vpn.0,frame);
    }

    pub fn alloc_fault(&mut self){
        //println!("alloc fault ");
        let (process,vpnumber,mut frame) = self.replacer.pop_first();
        let index = self.replacer.get_free_index().ok_or("no available frame to allocate")
                                                    .map(|offset| self.start_blk + offset).unwrap();
        
        let mut memset = &mut process.write().memory_set;

        
        memset.virtual_set.push((VirtualPageNumber(vpnumber),index));
        println!("alloc full -> add {:0x} in virtual set ",vpnumber);

        
        for i in 0..8{
            unsafe{self.blk_device.as_ref().unwrap().write_block(i+index,&(*((frame.deref() as *const _ as usize +i*512 )  as *const[u8;512] )));}
            let entry = memset.mapping.find_entry(VirtualPageNumber(vpnumber)).unwrap();
            *entry = PageTableEntry(0usize);
        }
        //println!("put in blk {}",index);
    }
    pub fn page_invaild(&mut self,process:Arc<RwLock<Process>>,vpa:VirtualAddress,mut frame:FrameTracker) {
        
        let mut memset = &mut process.write().memory_set;
        let vpn = VirtualPageNumber::from(VirtualPageNumber::floor(vpa));

        // if vpn.0==18{
        // println!("page invaild {:0x}",vpn.0);
        // }

        let virtual_set = &mut memset.virtual_set;
        'a:
        for pos in 0..virtual_set.len(){
            let x = virtual_set[pos];
            //println!("pos in virtual set {}" , x.0);
            if(x.0 == vpn){
                for seg in &memset.segments{
                    //println!(" {} {} {} ",vpa ,  seg.range.start, seg.range.end);
                    if vpa >=seg.range.start && vpa<seg.range.end{

                        println!("catch virtual page invaild {:0x} in blk_index {}",vpn.0,x.1);

                        memset.mapping.map_one(vpn, frame.page_number(), seg.flags | Flags::VALID);
                        frame.fill(0);
                        for i in 0..8{
                            unsafe{self.blk_device.as_ref().unwrap().read_block(i+x.1,&mut (*((frame.deref_mut() as *mut _ as usize +i*512) as *mut[u8;512])));}
                            
                        }
                        self.replacer.put_in_memory_record(process.clone(),vpn.0,frame);
                        virtual_set.remove(pos);
                        break 'a;
                    }
                }
            }
        }
        memset.activate();
        
        return 
    }
    
}




