## Problem [1422. Maximum Score After Splitting a String](https://leetcode.com/problems/maximum-score-after-splitting-a-string/description/)
### Intuition
Since the score is basically summation of number of `0`'s in left partition and number of `1`'s in right partition, we have to just keep the track of their number of occurences as we iterate through the string.
#### Solution
```rust
use std::cmp;
impl Solution {
    pub fn max_score(s: String) -> i32 {
        let mut total_zeros = 0;
        let mut total_ones = 0;
        for c in s.chars() {
            if c == '0' {
                total_zeros += 1;
            } else {
                total_ones += 1;
            }
        }
        let mut max_score = 0;
        let mut left_score = 0;
        let mut right_score = total_ones;
        let mut curr_score = 0;
        for i in 0..s.len()-1 {
            if (s.as_bytes()[i] as char == '0') {
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
```
#### Short Explanation
So first loop is used to basically count the number of `1`s and `0`'s in the string. Second loop is used to solve our problem, as we iterate through the string we divide the string into two non-empty sub strings (note that the index only goes up until `n-1` to ensure right substring is empty.) Then we calculate the score for that index. 