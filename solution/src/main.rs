mod test;

pub struct Solution;
impl Solution {
    pub fn max_array_value(nums: Vec<i32>) -> i64 {
        let mut acc = 0_i64;
        for n in (0..nums.len()).rev() {
            let var = nums[n] as i64;
            if acc >= var {
                acc+=var;
            } else {
                acc = var;
            }
        }
        acc
    }
}


fn main() {
}
