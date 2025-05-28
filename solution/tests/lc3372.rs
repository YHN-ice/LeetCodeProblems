struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn max_target_nodes(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        match k {
            0 => std::iter::repeat(1).take(edges1.len() + 1).collect(),
            _ => {
                let k_neighbors = |edges: Vec<Vec<i32>>, k: i32| -> _ {
                    // O(n^2)
                    let k = k.try_into().unwrap();
                    let n = edges.len() + 1;
                    let mut next = vec![vec![]; n];
                    for uv in edges {
                        let (u, v) = (uv[0] as usize, uv[1] as usize);
                        next[u].push(v);
                        next[v].push(u);
                    }
                    (0..n).into_iter().map(move |v| -> i32 {
                        let mut q = VecDeque::<(usize, usize, usize)>::new();
                        q.push_back((v, 0, v));
                        let mut found = 0;
                        while let Some(top) = q.pop_front() {
                            if top.1 > k {
                                break;
                            }
                            found += 1;
                            for &nx in &next[top.0] {
                                if nx == top.2 {
                                    continue;
                                }
                                q.push_back((nx, top.1 + 1, top.0));
                            }
                        }
                        found
                    })
                };
                let max_2 = k_neighbors(edges2, k - 1).max().unwrap();
                k_neighbors(edges1, k).map(|local| local + max_2).collect()
            }
        }
    }
}

mod test {
    use super::Solution;
    #[test]
    fn test1() {
        assert_eq!(
            vec![9, 7, 9, 8, 8],
            Solution::max_target_nodes(
                vec![vec![0, 1], vec![0, 2], vec![2, 3], vec![2, 4]],
                vec![
                    vec![0, 1],
                    vec![0, 2],
                    vec![0, 3],
                    vec![2, 7],
                    vec![1, 4],
                    vec![4, 5],
                    vec![4, 6]
                ],
                2
            )
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            vec![6, 3, 3, 3, 3],
            Solution::max_target_nodes(
                vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![0, 4]],
                vec![vec![0, 1], vec![1, 2], vec![2, 3]],
                1
            )
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            vec![1, 1],
            Solution::max_target_nodes(vec![vec![0, 1]], vec![vec![0, 1]], 0)
        );
    }
}
