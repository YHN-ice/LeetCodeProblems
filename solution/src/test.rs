#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn tc1 () {
        assert_eq!(2,  Solution::check_ways(vec![vec![1,2],vec![2,3],vec![1,3]]))
    }
    #[test]
    fn tc2 () {
        assert_eq!(1,  Solution::check_ways(vec![vec![1,2],vec![2,3]]))
    }
    #[test]
    fn tc3 () {
        assert_eq!(0,  Solution::check_ways(vec![vec![3,4],vec![2,3],vec![4,5],vec![2,4],vec![2,5],vec![1,5],vec![1,4]]))
    }
}
