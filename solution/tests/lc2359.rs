struct Solution;

impl Solution {
    pub fn closest_meeting_node(edges: Vec<i32>, node1: i32, node2: i32) -> i32 {
        use std::collections::VecDeque;
        let bfs = |node: usize| {
            let mut dist = vec![-1; edges.len()];
            let mut q = VecDeque::new();
            q.push_front((node, 0));
            while let Some((front, distance)) = q.pop_back() {
                if dist[front] >= 0 {
                    break;
                }
                dist[front] = distance;

                if edges[front] == -1 {
                    break;
                }
                q.push_front((edges[front].try_into().unwrap(), distance + 1))
            }
            dist
        };
        // println!("bfs 1 {:?}", bfs(node1.try_into().unwrap()));
        // println!("bfs 2 {:?}", bfs(node2.try_into().unwrap()));
        bfs(node1.try_into().unwrap())
            .into_iter()
            .zip(bfs(node2.try_into().unwrap()).into_iter())
            .enumerate()
            .filter_map(|(i, dpair)| match dpair {
                (-1, _) | (_, -1) => None,
                (a, b) => Some((i, a.max(b))),
            })
            .min_by_key(|x| x.1)
            .map(|x| (x.0.try_into().unwrap(), x.1))
            .unwrap_or((-1, -1))
            .0
    }
}

mod test {
    use super::Solution;
    #[test]
    fn test1() {
        assert_eq!(2, Solution::closest_meeting_node(vec![2, 2, 3, -1], 0, 1))
    }
    #[test]
    fn test2() {
        assert_eq!(2, Solution::closest_meeting_node(vec![1, 2, -1], 0, 2))
    }
    #[test]
    fn test3() {
        assert_eq!(
            1,
            Solution::closest_meeting_node(vec![4, 4, 4, 5, 1, 2, 2,], 1, 1)
        )
    }
}
