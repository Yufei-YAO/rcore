/*

leetcode_16  最接近的三数之和
给定一个包括 n 个整数的数组 nums 和 一个目标值 target。找出 nums 中的三个整数，使得它们的和与 target 最接近。
返回这三个数的和。假定每组输入只存在唯一答案。


*/


pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
    nums.sort();
    let mut result = nums[0]+nums[1]+nums[2];
    for i in 0..nums.len()-2{
        let mut left =i+1;
        let mut right =nums.len()-1;
        while left<right{
            let sum =nums[i] +nums[left] +nums[right];
            if (target-sum).abs()< (target-result).abs(){
                result = sum;
            }
            if sum > target{
                right-=1;
            }else{
                left+=1;
            }
        }
    }
    result
}

fn main(){
    println!("{}",three_sum_closest(vec![-1,2,1,-4],1));
}