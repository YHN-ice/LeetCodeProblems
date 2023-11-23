struct Solution;
impl Solution {
    pub fn count_pairs(nums: Vec<i32>, target: i32) -> i32 {
        let mut ans = 0;
        for (i,v) in nums.iter().enumerate() {
            for j in 0..i {
                if v+nums[j] < target {ans+=1;}
            }
        }
        ans
    }
}


#[test]
// 输入：nums = [-1,1,2,3,1], target = 2
// 输出：3
fn testcase_1() {
    assert_eq!(3, Solution::count_pairs(vec![-1,1,2,3,1], 2))
}



#[test]
// 输入：nums = [-6,2,5,-2,-7,-1,3], target = -2
// 输出：10
fn testcase_2() {
    assert_eq!(10, Solution::count_pairs(vec![-6,2,5,-2,-7,-1,3], -2))
}


#[test]
// 输入：nums = [9,-5,-5,5,-5,-4,-6,6,-6], target = 3
// 输出：27
fn testcase_3() {
    assert_eq!(27, Solution::count_pairs(vec![9,-5,-5,5,-5,-4,-6,6,-6], 3))
}
fn main() {}
