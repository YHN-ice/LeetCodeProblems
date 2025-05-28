struct Solution;

impl Solution {
    pub fn lexicographically_smallest_string(s: String) -> String {
        if s.len() == 1 {
            return s;
        }
        let dist = |b1: u8, b2: u8| -> u8 { b1.max(b2) - b1.min(b2) };
        let ss = s.clone();
        let s = s.as_bytes();
        let n = s.len();

        let mut dp = vec![vec![false; n]; n];
        for r in 0..n {
            for l in 0..=r {
                // s[l..=r]
                let range_len = r - l + 1;
                let ends_meet = dist(s[l], s[r]) == 1;
                match range_len {
                    1 => (),
                    2 => dp[l][r] = ends_meet,
                    _ => {
                        dp[l][r] = dp[l + 1][r - 1] && ends_meet;
                        for m in l + 1..r - 1 {
                            dp[l][r] = dp[l][r] || (dp[l][m] && dp[m + 1][r]);
                        }
                    }
                }
            }
        }

        let mut f = vec![String::new(); n + 1];
        f[1] = ss[n - 1..].to_string();

        for i in 2..=s.len() {
            f[i] = ss[n - i..n - i + 1].to_string() + f[i - 1].as_str();
            for j in 0..i - 2 {
                let (l, r) = (n - i, n - j - 1);
                if dp[l][r] {
                    f[i] = f[i].clone().min(f[j].clone());
                }
            }
        }

        f[n].clone()
    }
}

mod test {
    use super::Solution;
    #[test]
    fn test1() {
        assert_eq!(
            "zdce".to_string(),
            Solution::lexicographically_smallest_string("zdce".to_string())
        );
    }
    #[test]
    fn test2() {
        assert_eq!(
            "".to_string(),
            Solution::lexicographically_smallest_string("bcda".to_string())
        );
    }
}
