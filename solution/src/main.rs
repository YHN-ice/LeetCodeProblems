
struct Solution;
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

#[test]
// 输入：nums = [2,3,7,9,3]
// 输出：21
fn testcase_1() {
    assert_eq!(21, Solution::max_array_value(vec![2,3,7,9,3]))
}



#[test]
// 输入：nums = [5,3,3]
// 输出：11
fn testcase_2() {
    assert_eq!(11, Solution::max_array_value(vec![5,3,3]))
}

#[test]
// 输入：nums = [40,15,35,98,77,79,24,62,53,84,97,16,30,22,49]
// 输出：781
fn testcase_3() {
    assert_eq!(781, Solution::max_array_value(vec![40,15,35,98,77,79,24,62,53,84,97,16,30,22,49]))
}

fn main() {
}
