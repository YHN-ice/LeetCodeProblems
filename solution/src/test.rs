#[cfg(test)]
mod test {
    use crate::Solution;
    #[test]
// 输入：nums = [2,3,7,9,3]
// 输出：21
    fn testcase_1() {
        assert_eq!(21, Solution::max_array_value(vec![2, 3, 7, 9, 3]))
    }


    #[test]
// 输入：nums = [5,3,3]
// 输出：11
    fn testcase_2() {
        assert_eq!(11, Solution::max_array_value(vec![5, 3, 3]))
    }

    #[test]
// 输入：nums = [40,15,35,98,77,79,24,62,53,84,97,16,30,22,49]
// 输出：781
    fn testcase_3() {
        assert_eq!(781, Solution::max_array_value(vec![40, 15, 35, 98, 77, 79, 24, 62, 53, 84, 97, 16, 30, 22, 49]))
    }
}