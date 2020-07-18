//! 一些可能用到，而又不好找库的数据结构
//!
//! 以及有多种实现，会留作业的数据结构
#![no_std]
#![feature(linked_list_remove)]
#![feature(llvm_asm)]
extern crate alloc;

#[macro_use]
mod console;

mod allocator;
mod sbi;

pub use allocator::*;

