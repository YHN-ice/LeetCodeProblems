struct Solution;

impl Solution {
    pub fn max_target_nodes(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>) -> Vec<i32> {
        let neighbor_at_distance = |edges: Vec<Vec<i32>>| {
            let n = edges.len() + 1;
            let mut next = vec![vec![]; n];

            for uv in edges {
                match uv[..] {
                    [u, v] => {
                        let u_idx: usize = u.try_into().unwrap();
                        let v_idx: usize = v.try_into().unwrap();
                        next[u_idx].push(v);
                        next[v_idx].push(u);
                    }
                    _ => unreachable!("edge is an length 2 vector"),
                }
            }

            // the coloring / bipartite graph idea, creidt to
            // https://leetcode.cn/problems/maximize-the-number-of-target-nodes-after-connecting-trees-ii/solutions/3006331/an-qi-ou-fen-lei-pythonjavacgo-by-endles-dweg
            let mut coloring: Vec<bool> = vec![false; n]; // odd zero, even 1 (self)
            let mut oracle: Vec<(i32, i32)> = vec![(0, 1); n]; // odd zero, even 1 (self)
            let mut stack = vec![(0, 0, false)];
            while let Some(&(top, fa, visited)) = stack.last() {
                let top_idx: usize = top.try_into().unwrap();
                match visited {
                    true => {
                        oracle[top_idx] = next[top_idx]
                            .iter()
                            .map(|&x| -> (i32, i32) {
                                let x_idx: usize = x.try_into().unwrap();
                                if x == fa {
                                    (0, 0)
                                } else {
                                    (oracle[x_idx].1, oracle[x_idx].0)
                                }
                            })
                            .fold(oracle[top_idx], |(acc_a, acc_b), (a, b)| {
                                (acc_a + a, acc_b + b)
                            });
                        stack.pop();
                    }
                    _ => {
                        stack.last_mut().map(|x| x.2 = true);
                        for &child in next[top_idx].iter() {
                            if child == fa {
                                continue;
                            }
                            let child_idx: usize = child.try_into().unwrap();
                            coloring[child_idx] = !coloring[top_idx];
                            stack.push((child, top, false))
                        }
                    }
                }
            }
            (coloring, oracle[0])
        };
        let (_, (s1, s2)) = neighbor_at_distance(edges2);
        let max_2 = s1.max(s2);
        let (coloring, (s0, s1)) = neighbor_at_distance(edges1);
        coloring
            .into_iter()
            .map(|x| {
                max_2
                    + (match x {
                        true => s0,
                        _ => s1,
                    })
            })
            .collect()
    }
}

mod test {
    use super::Solution;

    // Input: edges1 = [[0,1],[0,2],[2,3],[2,4]], edges2 = [[0,1],[0,2],[0,3],[2,7],[1,4],[4,5],[4,6]]
    // Output: [8,7,7,8,8]
    #[test]
    fn test1() {
        assert_eq!(
            vec![8, 7, 7, 8, 8],
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
                ]
            )
        )
    }

    // Input: edges1 = [[0,1],[0,2],[0,3],[0,4]], edges2 = [[0,1],[1,2],[2,3]]
    // Output: [3,6,6,6,6]
    #[test]
    fn test2() {
        assert_eq!(
            vec![3, 6, 6, 6, 6],
            Solution::max_target_nodes(
                vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![0, 4]],
                vec![vec![0, 1], vec![1, 2], vec![2, 3]]
            )
        )
    }
}
