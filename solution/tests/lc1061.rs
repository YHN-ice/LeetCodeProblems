struct Solution;

impl Solution {
    pub fn smallest_equivalent_string(s1: String, s2: String, base_str: String) -> String {
        let mut fa = [0; 26];
        (0..26).for_each(|x| fa[x] = x);
        let find = |fa: &[usize; 26], mut node: usize| -> usize {
            while fa[node] != node {
                node = fa[node];
            }
            node
        };
        for (c1, c2) in s1.into_bytes().into_iter().zip(s2.into_bytes().into_iter()) {
            let c1 = (c1 - b'a') as usize;
            let c2 = (c2 - b'a') as usize;
            let r1 = find(&fa, c1);
            let r2 = find(&fa, c2);
            match r1.cmp(&r2) {
                std::cmp::Ordering::Less => fa[r2] = r1,
                _ => fa[r1] = r2,
            }
        }
        // println!("dict: {:?}", dict);
        base_str
            .into_bytes()
            .into_iter()
            .map(|b| (find(&fa, (b - b'a') as usize) as u8 + b'a') as char)
            .collect::<String>()
    }
}

mod test {

    macro_rules! str {
        ( $s:literal ) => {
            String::from($s)
        };
    }

    use super::Solution;

    #[test]
    // Input: s1 = "parker", s2 = "morris", baseStr = "parser"
    // Output: "makkek"
    fn test1() {
        assert_eq!(
            str!("makkek"),
            Solution::smallest_equivalent_string(str!("parker"), str!("morris"), str!("parser"))
        )
    }

    #[test]
    // Input: s1 = "hello", s2 = "world", baseStr = "hold"
    // Output: "hdld"
    fn test2() {
        assert_eq!(
            str!("hdld"),
            Solution::smallest_equivalent_string(str!("hello"), str!("world"), str!("hold"))
        )
    }

    #[test]
    // Input: s1 = "leetcode", s2 = "programs", baseStr = "sourcecode"
    // Output: "aauaaaaada"
    fn test3() {
        assert_eq!(
            str!("aauaaaaada"),
            Solution::smallest_equivalent_string(
                str!("leetcode"),
                str!("programs"),
                str!("sourcecode")
            )
        )
    }
}
