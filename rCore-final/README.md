此为根据实验提供的rCore进行修改的最终版本

功能实现

1.基于优先级的进程调度

/os/src/algorithm/cfs_scheduler
初步设想是想仿照linux的CFS调度器进行设计，但是Rust的collection缺乏许多必要的数据结构如红黑树，所以在进程切换选择的效率上并不高效。
但是可以跟据预先设计的优先级进行类似CFS调度器中基于优先级大小分配时间片的功能。
测试为main.rs 底部加了注释的部分 。

进程5 优先权重为1
进程10 优先权重为5
结果如下
...
thread 5
thread 10
thread 10
thread 10
thread 10
thread 10
thread 5
thread 10
thread 10
thread 10
thread 10
thread 10
thread 5
thread 10
thread 10
thread 10
thread 10
thread 10
...
src/main.rs:154: 'stop'



2.初步的基于虚拟内存的缺页中断设计
/os/src/memory/replacer/*
/os/src/memory/replace.rs

调度策略为FIFO

为了进行测试，限制/os/src/memory/frame/allocator.rs中页面分配器拥有的页面数量为2000页
在main.rs中开始10个下述进程

start_user_thread("hello_world",4);
start_user_thread("hello_world",4);
start_user_thread("hello_world",4);
start_user_thread("hello_world",4);
start_user_thread("hello_world",4);
start_user_thread("hello_world",4);
start_user_thread("hello_world",4);
start_user_thread("hello_world",4);
start_user_thread("hello_world",4);
start_user_thread("hello_world",4);
若使用实验提供的rcore版本

hello_world定义为
#[no_mangle]
pub fn main() -> usize {
    
    println!("Hello world from user mode program!");
    let mut pos =0;
    for i in 0..1000{
            pos+=1;
            //println!("{}", pos);
    }
    println!("usr Hello world end!");

    0
}

会出现错误

mod memory initialized
mod interrupt initialized
mod driver initialized
.
..
hello_world
notebook
mod fs initialized
src/main.rs:102: 'called `Result::unwrap()` on an `Err` value: "no available frame to allocate"'

使用了我设计的添加了页面置换功能的rCore

则进程可以完整运行效果如下
...
alloc full -> add b1 in virtual set 
alloc full -> add b0 in virtual set 
alloc full -> add af in virtual set 
alloc full -> add ae in virtual set 
alloc full -> add ad in virtual set 
alloc full -> add ac in virtual set
...



...
page_fault_handler miss vp : 20
alloc full -> add 1058 in virtual set 
catch virtual page invaild 20 in blk_index 18200
page_fault_handler miss vp : 40
alloc full -> add 1057 in virtual set 
catch virtual page invaild 40 in blk_index 17944
page_fault_handler miss vp : 80
alloc full -> add 1056 in virtual set 
catch virtual page invaild 80 in blk_index 17432
page_fault_handler miss vp : 100
...


最终结果为所有进程运行完毕
usr Hello world end!
Thread 7 exit with code 0
Hello world from user mode program!
usr Hello world end!
Thread 8 exit with code 0
Hello world from user mode program!
usr Hello world end!
Thread 9 exit with code 0
Hello world from user mode program!
usr Hello world end!
Thread 10 exit with code 0
src/process/processor.rs:100: 'all threads terminated, shutting down'




