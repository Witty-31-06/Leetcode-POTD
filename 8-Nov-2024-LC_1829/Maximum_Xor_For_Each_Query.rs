// https://leetcode.com/problems/maximum-xor-for-each-query/
struct Solution;
fn main()
{
    let nums = vec![0,1,2,2,5,7];
    let maximum_bit = 3;
    Solution::get_maximum_xor(nums, maximum_bit);
}

impl Solution {
    pub fn get_maximum_xor(nums: Vec<i32>, maximum_bit: i32) -> Vec<i32> {
        let n: usize = nums.len();
        let mut ans = Vec::new();
        let mut xor_value = 0;
        for i in 0..n {
            xor_value = xor_value ^ nums[i];
        }
        // println!("{xor_value}");
        let mask = (1<<maximum_bit)-1;
        for i in (0..n).rev() {
            let element = nums[i];
            
            let k = (!xor_value) & mask;
            ans.push(k);
            // println!("{mask}, {xor_value}, {k}");
            xor_value = xor_value ^ element;
            
        }
        println!("{:?}", ans);
        return ans;
    }
}