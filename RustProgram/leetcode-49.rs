/*

leetcode_49  字母异位词分组
给你 n 个非负整数 a1，a2，...，an，每个数代表坐标中的一个点 (i, ai) 。在坐标内画 n 条垂直线，垂直线 i 的两个端点分别为 (i, ai) 和 (i, 0)。找出其中的两条线，使得它们与 x 轴共同构成的容器可以容纳最多的水。

给定一个字符串数组，将字母异位词组合在一起。字母异位词指字母相同，但排列不同的字符串。
*/
use std::cmp::min;
use std::cmp::max;
use std::collections::HashMap;
pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut ans = Vec::< Vec<String>>::new();
    let mut hash_map:HashMap::<String,Vec<String>> =HashMap::new();
    for st_r in strs{
        let mut str_so = st_r.clone();
        let mut str_sort =str_so.into_bytes();
        str_sort.sort();
        match hash_map.get_mut(&String::from_utf8(str_sort.to_vec()).unwrap()){
            Some(ve) => ve.push(st_r),
            None =>{
                hash_map.insert(String::from_utf8(str_sort.to_vec()).unwrap(),vec![st_r]);
            }
        }
    }
    for val in hash_map.values(){
        ans.push(val.clone());
    }
    ans
}

fn main(){
    println!("{:?}",group_anagrams(vec!["eat".to_string(), "tea".to_string(), "tan".to_string(), "ate".to_string(), "nat".to_string(), "bat".to_string()]));
}