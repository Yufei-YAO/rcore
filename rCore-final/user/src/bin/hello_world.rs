#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;


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

