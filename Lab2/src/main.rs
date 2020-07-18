//! # 全局属性
//! - `#![no_std]`  
//!   禁用标准库
#![no_std]
//!
//! - `#![no_main]`  
//!   不使用 `main` 函数等全部 Rust-level 入口点来作为程序入口
#![no_main]
//!
//! - `#![deny(missing_docs)]`  
//!   任何没有注释的地方都会产生警告：这个属性用来压榨写实验指导的学长，同学可以删掉了
#![warn(missing_docs)]
#![feature(llvm_asm)]
//! # 一些 unstable 的功能需要在 crate 层级声明后才可以使用
//! - `#![feature(asm)]`  
//!   内嵌汇编
#![feature(asm)]
//!
//! - `#![feature(global_asm)]`  
//!   内嵌整个汇编文件
#![feature(global_asm)]
//!
//! - `#![feature(panic_info_message)]`  
//!   panic! 时，获取其中的信息并打印
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]

#[macro_use]
mod console;
mod panic;
mod sbi;
mod interrupt;
mod memory;
mod data_structure;
#[macro_use]
extern crate lazy_static;




extern crate alloc;


// 汇编编写的程序入口，具体见该文件
global_asm!(include_str!("asm/entry.asm"));

/// Rust 的入口函数
/// 
/// 
///
///
/// 在 `_start` 为我们进行了一系列准备之后，这是第一个被调用的 Rust 函数
#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    extern "C" {
        fn kernel_end();
    }
    println!("kernel end vaddr = {:#x}", kernel_end as usize);

    crate::interrupt::init();

    crate::memory::init();
    frame_allocating_test();


    panic!("end of rust_main");
    loop {}
}

fn frame_allocating_test() {
    let frame_0 = match memory::frame::FRAME_ALLOCATOR.lock().alloc() {
        Result::Ok(frame_tracker) => frame_tracker,
        Result::Err(err) => panic!("{}", err)
    };
    println!("alloc {:x?}", frame_0.address());
    let mut f = match memory::frame::FRAME_ALLOCATOR.lock().alloc_frames(5) {
        Result::Ok(frame_tracker) => frame_tracker,
        Result::Err(err) => panic!("{}", err)
    };
    println!("alloc {:x?}", f.address());
    let mut t = match memory::frame::FRAME_ALLOCATOR.lock().alloc_frames(5) {
        Result::Ok(frame_tracker) => frame_tracker,
        Result::Err(err) => panic!("{}", err)
    };
    println!("alloc {:x?}", t.address());

    println!("dealloc {:x?}", f.address());
    drop(f);

    let frame_2 = match memory::frame::FRAME_ALLOCATOR.lock().alloc_frames(2) {
        Result::Ok(frame_tracker) => frame_tracker,
        Result::Err(err) => panic!("{}", err)
    };
    println!("alloc {:x?}", frame_2.address());
    let frame_3 = match memory::frame::FRAME_ALLOCATOR.lock().alloc_frames(3) {
        Result::Ok(frame_tracker) => frame_tracker,
        Result::Err(err) => panic!("{}", err)
    };
    println!("alloc {:x?}", frame_3.address());

}