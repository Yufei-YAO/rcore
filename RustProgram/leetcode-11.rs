/*

leetcode_11   盛最多水的容器
给你 n 个非负整数 a1，a2，...，an，每个数代表坐标中的一个点 (i, ai) 。在坐标内画 n 条垂直线，垂直线 i 的两个端点分别为 (i, ai) 和 (i, 0)。找出其中的两条线，使得它们与 x 轴共同构成的容器可以容纳最多的水。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/container-with-most-water
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/
use std::cmp::min;
use std::cmp::max;

pub fn max_area(height: Vec<i32>) -> i32 {
    let len =height.len();
    let mut l:usize =0;
    let mut r:usize = len -1;
    let mut ans:i32 = 0;
    while l<r {
        ans = max (ans,min(height[l],height[r]) * (r as i32 -l as i32 ));
        if( height[l] < height[r]){
            l+=1;
        }else{
            r-=1;
        }
    }
    ans
}

fn main(){
    println!("{}",max_area(vec![1,8,6,2,5,4,8,3,7]));
}