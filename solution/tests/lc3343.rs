struct Solution;

use solution::numeric::{factorial, inv_factorial, LongLongModulo, LL};

use solution::{ll, nil, one, zero};

fn dp(i: LL, left: LL, s: LL, cnt: &Vec<LL>, pre: &Vec<LL>, memo: &mut Vec<Vec<Vec<LL>>>) -> LL {
    match i {
        nil!() => match (left, s) {
            (zero!(), zero!()) => 1.into(),
            _ => zero!(),
        },
        _ => {
            let i_usize: usize = i.try_into().expect("i >= 0");
            let left_usize: usize = left.try_into().expect("k >= 0");
            let s_usize: usize = s.try_into().expect("s >= 0");
            match memo[i_usize][left_usize][s_usize] {
                nil!() => {
                    let mut res = zero!();
                    let left2 = pre[i_usize] - left;
                    let lb = (cnt[i_usize] - left2).max(zero!());
                    let ub = left.min(cnt[i_usize]);
                    for k in lb.0..=ub.0 {
                        let k = ll!(k);
                        if k * i > s {
                            break;
                        }
                        let r = dp(i - 1, left - k, s - k * i, cnt, pre, memo);
                        res = res + r * inv_factorial(k) * inv_factorial(cnt[i_usize] - k);
                    }
                    memo[i_usize][left_usize][s_usize] = res;
                    res
                }
                v => v.into(),
            }
        }
    }
}

impl Solution {
    pub fn count_balanced_permutations(num: String) -> i32 {
        let total: i64 = num.bytes().map(|x| (x - b'0') as i64).sum();
        if total & 1 == 1 {
            return 0;
        }
        let total = total / 2;
        let cnt = num.bytes().fold(vec![zero!(); 10], |acc, x| -> Vec<_> {
            let mut acc = acc;
            let n = x - b'0';
            acc[n as usize] += one!();
            acc
        });
        let pre: Vec<_> = cnt
            .iter()
            .scan(zero!(), |acc, &elt| {
                *acc += elt;
                Some(*acc)
            })
            .collect();

        let n = num.len();
        let n1 = n / 2;
        let n1_i64 = n1 as i64;
        let mut memo = vec![vec![vec![nil!(); 1 + total as usize]; n1 + 1]; 10];

        ((factorial(ll!(n1_i64)) * factorial(ll!((n - n1) as i64)))
            * dp(ll!(9), ll!(n1_i64), ll!(total), &cnt, &pre, &mut memo))
        .0 as i32
    }
}

mod test {
    use super::Solution;
    #[test]
    fn test1() {
        assert_eq!(2, Solution::count_balanced_permutations("123".into()));
    }
}
