use std::cmp;
struct Solution;
impl Solution {
    pub fn max_score(s: String) -> i32 {

        let mut total_ones = 0;
        for c in s.chars() {
            if c == '1' {
                total_ones += 1;
            }
        }
        let mut max_score = 0;
        let mut left_score = 0;
        let mut right_score = total_ones;
        let mut curr_score = 0;
        for i in 0..s.len()-1 {
            if s.as_bytes()[i] as char == '0' {
                left_score += 1;
            } else {
                right_score -= 1;
            }
            curr_score =  right_score + left_score;
            max_score = cmp::max(max_score, curr_score);
        }
        return max_score;
    }
}

fn main() {
    let s = "011101".to_string();
    let result = Solution::max_score(s);
    println!("{}", result);
}