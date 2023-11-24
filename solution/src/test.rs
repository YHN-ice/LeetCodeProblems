#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn testcase_1() {
        assert_eq!(vec![vec![24,12],vec![8,6]], Solution::construct_product_matrix(vec![vec![1,2],vec![3,4]]));
    }


    #[test]
    fn testcase_2() {
        assert_eq!(vec![vec![2],vec![0],vec![0]], Solution::construct_product_matrix(vec![vec![12345],vec![2],vec![1]]));
    }

    #[test]
    fn testcase_3() {
        assert_eq!(vec![vec![1462],vec![3103],vec![9436]], Solution::construct_product_matrix(vec![vec![414750857],vec![449145368],vec![767292749]]));
    }
}