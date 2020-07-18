
use super::VectorAllocator;
use super::list::LinkedList;
use core::cmp::min;
use bit_field::BitField;
const SLABMAP_SIZE:usize = 0x80_0000;
use alloc::vec::Vec;
use core::ptr;
pub struct SlabVectorAllocator {
    list_table:[LinkedList;19]
    
}
impl VectorAllocator for SlabVectorAllocator {
    fn new(sta:usize ,capacity: usize) -> Self {
        unsafe{let mut ta = [LinkedList{head: ptr::null_mut() as *mut usize };19];
        for i in 0..1{
            ta[18].push((sta+i*262144) as *mut usize);
        }
        Self {
            list_table : ta,
        }}
    }
    fn alloc(&mut self, mut size: usize, align: usize) -> Option<usize> {

        let mut len =1;let mut p=0;
        if size<8{
            size = 8;
        }
        while len< size{
            len*=2;
            p+=1;
        }
        let  mut pos =0;
        for mut list_iter in self.list_table[p].iter_mut() {
            let tmp:usize = list_iter.curr as usize;
            //println!("f 0x{:X}",tmp);
            if tmp&((align & (-(align as i64) as usize))-1)==0{
                unsafe{
                    //println!("*list_iter.prev = *list_iter.curr as usize;");
                    *list_iter.prev = *list_iter.curr as usize;
                    self.deal_with_more(p-1,size,len,tmp);
                    
                }
                //println!("return");
                return Some(tmp);
            }
            pos+=1;
        }
        
        let get = self.alloc(len*2,align).unwrap();
        unsafe{
        self.list_table[p].push(get as *mut usize);
        self.list_table[p].push((get+len) as *mut usize);
        }
        println!("{:0x} {:0x}" ,get,get+len);

        let t = self.alloc(size,align);
        return t;
    }
    fn dealloc(&mut self, start: usize, mut size: usize, _align: usize) {
        if(size < 8){
            size =8;
        }
        let mut len =262144;let mut p=18;
        let mut st_art =start;
        let mut s_ize=size;

        unsafe{
        while s_ize>0{
            let mut  flag =true;
            if(s_ize >= len){
                for list_iter in self.list_table[p].iter_mut(){
                    let tmp:usize = list_iter.curr as usize;
                    if (start+len == tmp || start == tmp+len ){
                        *list_iter.prev = *list_iter.curr as usize;
                        self.dealloc(min(start,tmp),len*2, _align);
                        flag =false;
                        break;
                    }
                }
                if flag ==true{
                self.list_table[p].push((start) as *mut usize);
                }
                st_art+=len;
                s_ize-=len;
            }
            p-=1;
            len/=2;
        }
        }

    }
}
impl SlabVectorAllocator{
    unsafe fn deal_with_more(&mut self,p:usize,need:usize,have:usize,last_start:usize){

        if(need==have) {return;}
        else if(need == have/2) {
            self.list_table[p].push((last_start+have/2) as *mut usize)
        }else if need <have/2{
            self.list_table[p].push((last_start+have/2) as *mut usize);
            self.deal_with_more(p-1,need,have/2,last_start+(have/2))
        }else {
            self.deal_with_more(p-1,need-have/2,have/2,last_start+(have/2))
        }
    }

}