/*

leetcode_3  无重复字符的最长子串
给定一个字符串，请你找出其中不含有重复字符的 最长子串 的长度
*/


use std::cmp::max;
use std::collections::HashSet;
pub fn length_of_longest_substring(s: String) -> i32 {
    let mut set_map = HashSet::<u8>::new();
    let s_u =  s.as_bytes();
    let mut ans =0;
    let mut r =0;
    for i in 0..s_u.len(){
        if i>0 {
            set_map.remove(&(s_u[i-1] as u8));
        }

        while r <s_u.len() &&  !set_map.contains(&(s_u[r] as u8)){
            set_map.insert(s_u[r] as u8);
            r+=1;
        }
        ans =max(ans,r-i);
    }
    ans as i32
}

fn main(){
    println!("{}",length_of_longest_substring("dvdf".to_string()));
}