//! 最高响应比优先算法的调度器 [`CFSScheduler`]
use super::rbtree::RBTree;
use super::Scheduler;
use core::cmp::max;
use core::cmp::Ordering;


#[derive(Copy,Clone,Debug,Eq)]
pub struct VrunCompare(pub usize,pub isize);
impl VrunCompare{
    pub fn get_0(&self)->usize{
        return self.0;
    }
}

impl PartialEq for VrunCompare{
    fn eq(&self, other: &Self) -> bool{
        if (self.0 == other.0 )&&(self.1==other.1){
            return true;
        }
        false
    }
}
impl PartialOrd for VrunCompare{
    fn partial_cmp(&self, other: &VrunCompare) -> Option<Ordering>{
        if self.0 < other.0{
            return Some(Ordering::Less);
        }else if self.1 < other.1{
            return Some(Ordering::Less);
        }else {
            return Some(Ordering::Equal);
        }
    }
}
impl Ord for VrunCompare{
    fn cmp(&self,other:&Self)->Ordering{
        if self.0< other.0{
            return Ordering::Less;
        }else if self.1 < other.1{
            return Ordering::Less;
        }else {
            return Ordering::Equal;
        }
    }
}
/// 将线程和调度信息打包
#[derive(Debug)]
pub struct CFSThread<ThreadType: Clone + Eq> {

    /// 线程数据
    pub id:isize,
    pub vruntime:usize,
    pub nice:usize,
    pub thread: ThreadType,
}

/// 采用 CFS（最高响应比优先算法）的调度器
pub struct CFSScheduler<ThreadType: Clone + Eq > {
    current_thread_num:usize,
    /// 带有调度信息的线程池
    pool: RBTree<VrunCompare,CFSThread<ThreadType>>,
}

/// `Default` 创建一个空的调度器
impl<ThreadType: Clone + Eq  > Default for CFSScheduler<ThreadType> {
    fn default() -> Self {
        Self {
            current_thread_num:0,
            pool: RBTree::new(),
        }
    }
}

impl<ThreadType: Clone + Eq> Scheduler<ThreadType> for CFSScheduler<ThreadType> {
    fn add_thread(&mut self, thread: ThreadType, _priority:usize,id:isize) {
        
        self.current_thread_num+=1;
        if self.current_thread_num > 1{
        let f = self.pool.get_first().unwrap().0.get_0();
        let h =CFSThread {
            id,
            nice:_priority,
            vruntime:f,
            thread,
        };
            //println!("{}",h.nice);

        self.pool.insert(VrunCompare(f,id),h);

            
        }else{
            let h =CFSThread {
                id,
                nice:_priority,
                vruntime:1_usize,
                thread,
            };
            //println!("vrun{}",h.vruntime);
            self.pool.insert(VrunCompare(1,id),h);
        }
        //self.pool.print_tree();
    }

    fn get_next(&mut self) -> Option<ThreadType> {

        
        let mut min_key=VrunCompare(usize::MAX,100);
            let mut key_iter =self.pool.keys();
            while let Some(sth) = key_iter.next().clone(){
                
                let x =(*sth).clone();

                if min_key.0 >x.0 {
                    
                    min_key = x ;
                }
            }
        //println!("{}  {} ",min_key.0,min_key.1);
        if let Some(mut best) =self.pool.remove(&min_key){

            if best.vruntime>= usize::MAX/2{
                best.vruntime=0;
            }
            //println!("thread.id={} thread.vruntime={} thread.nice={}",tid,tvruntime,tnice);
            else {
                best.vruntime=best.vruntime+10000/best.nice;
            }

            let h =CFSThread {
                nice:best.nice,
                vruntime:best.vruntime+10000/best.nice,
                id:best.id,
                thread:best.thread.clone(),
            };

            self.pool.insert(VrunCompare(h.vruntime,h.id),h);
            Some(best.thread)
        }else{
            None
        }
    }
    fn remove_thread(&mut self, thread: &ThreadType) {
        // 移除相应的线程并且确认恰移除一个线程
        self.current_thread_num-=1;
        for (key,value) in self.pool.iter_mut(){
            if value.thread == *thread{
                let h = VrunCompare(key.0,key.1);
                self.pool.remove(&h);
                return ;
            }
        }
        //let mut removed = self.pool.remove(VrunCompare(thread.inner.lock().total_runtime,thread.id));
        //assert!(removed.next().is_some() && removed.next().is_none());
    }
    fn set_priority(&mut self, _thread: ThreadType, _priority: usize) {}
}
