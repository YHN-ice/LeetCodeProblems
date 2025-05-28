struct Solution;

impl Solution {
    pub fn minimum_moves(nums: Vec<i32>, k: i32, max_changes: i32) -> i64 {
        (|| {
            let (k, max_changes): (i64, i64) = (k.try_into().ok()?, max_changes.try_into().ok()?);
            let ones: Vec<i64> = nums
                .into_iter()
                .enumerate()
                .filter_map(|(idx, elem)| match elem {
                    1 => Some(idx.try_into().ok()?),
                    _ => None,
                })
                .collect();
            let l_end = ones.len().try_into().ok()?;
            let presum: Vec<_> = ones
                .iter()
                .scan(0_i64, |state: &mut i64, v: &i64| {
                    *state += *v;
                    Some(*state)
                })
                .collect();
            (0..=(3_i64).min(k).min(l_end))
                .map_while(|save_for_native| {
                    let range_len = k - max_changes.min(k - save_for_native);
                    match range_len {
                        0 => Some((k - range_len) * 2),
                        _ => (0..l_end)
                            .zip(range_len..)
                            .map_while(|(l, r)| {
                                if r > l_end {
                                    None
                                } else {
                                    let mid = l + ((r - l) >> 1);
                                    let r_usize: usize = r.try_into().ok()?;
                                    let l_usize: usize = l.try_into().ok()?;
                                    let mid_usize: usize = mid.try_into().ok()?;
                                    let distance_sum = presum[r_usize - 1_usize]
                                        - presum[mid_usize]
                                        - (r - 1 - mid) * ones[mid_usize]
                                        + (mid - l + 1) * ones[mid_usize]
                                        - (presum[mid_usize]
                                            - (match l {
                                                0 => 0,
                                                _ => presum[l_usize - 1],
                                            }));
                                    Some(distance_sum + (k - range_len) * 2)
                                }
                            })
                            .min(),
                    }
                })
                .min()
        })()
        .unwrap()
    }
}

mod test {
    use super::Solution;

    #[test]
    fn test1() {
        assert_eq!(
            3,
            Solution::minimum_moves(vec![1, 1, 0, 0, 0, 1, 1, 0, 0, 1], 3, 1)
        )
    }

    #[test]
    fn test2() {
        assert_eq!(4, Solution::minimum_moves(vec![0, 0, 0, 0], 2, 3))
    }

    #[test]
    fn test3() {
        assert_eq!(4, Solution::minimum_moves(vec![1, 0, 1, 0, 1], 3, 0))
    }

    #[test]
    fn test4() {
        assert_eq!(0, Solution::minimum_moves(vec![0, 1], 1, 0))
    }

    #[test]
    fn test5() {
        assert_eq!(1, Solution::minimum_moves(vec![1, 1], 2, 0))
    }

    #[test]
    fn test6() {
        assert_eq!(0, Solution::minimum_moves(vec![1, 1], 1, 2))
    }
    #[test]
    fn test7() {
        assert_eq!(1, Solution::minimum_moves(vec![1, 1, 1], 2, 0))
    }
}
