// leetcode 1719
impl Solution {
    pub fn check_ways(pairs: Vec<Vec<i32>>) -> i32 {
        use std::collections::{HashSet, HashMap};
        // recursive credit to @DBabichev, https://leetcode.com/DBabichev/

        // 1. find the root, if can not find one return 0
        // 2. remove the edges from root to others, form connected components for the rest of the graph
        // 3. for each component, run check_ways recursively, and check the aggregate result as follows
        //    1. if any components return 0, the overall result is 0
        //    2. if all components return 1, then return 1
        //    3. if some return 2 and others return 1, return 2
        fn rec(graph: &mut HashMap<i32, HashSet<i32>>, mut sub_vertices: Vec<i32>)->i32 {
            // first find the root, by collect the length of adjacent list, root should have its adj list of |V| - 1
            let v = sub_vertices.len();
            sub_vertices.sort_by_key(|node| graph.get(node).map(|x| x.len()).unwrap());

            let root = sub_vertices[v-1]; // find the root
            let mut candidate_root = if v >= 2 {sub_vertices.get(v-2)} else {None};
            // eprintln!("root = {} \t graph.get(&root).map(|x| x.len()).unwrap_or_default() != v-1:{}, {}",root,graph.get(&root).map(|x| x.len()).unwrap_or_default() , v-1 );
            if graph.get(&root).map(|x| x.len()).unwrap_or_default() != v-1 {return 0;} // if we can't, return 0
            if let Some(cand) = candidate_root.take() {
                if graph.get(&cand).map(|x| x.len()).unwrap_or_default() == v-1 {
                    candidate_root = Some(cand); //resume
                }
            }
            for nei in graph.get(&root).unwrap().clone() {graph.get_mut(&nei).map(|x| x.remove(&root));} // remove edges from root to others
            let mut seen = HashSet::new();
            fn dfs (node:i32, graph: &HashMap<i32, HashSet<i32>>, ret:&mut Vec<i32>, seen:&mut HashSet<i32>){
                for nei in graph.get(&node).unwrap() {
                    if !seen.contains(nei) {
                        seen.insert(*nei);
                        dfs(*nei, graph, ret, seen);
                    }
                }
                ret.push(node)
            }
            let mut componentents = vec![];
            for &node in &sub_vertices {
                if node == root || seen.contains(&node) {continue}
                seen.insert(node);
                let mut ret = vec![];
                dfs(node, graph, &mut ret, &mut seen);
                componentents.push(ret);
            }
            let mut subres = vec![];
            for component in componentents {
                subres.push(rec(graph, component));
            }
            if subres.iter().any(|x| *x==0) {
                0
            } else if subres.iter().any(|x| *x==2) {
                // eprintln!("mult comp");
                2
            } else if candidate_root.is_some() {
                // eprintln!("mult root");
                2
            } else {
                1
            }
        }
        let mut graph:HashMap<i32, HashSet<i32>> = HashMap::new();
        let mut all_nodes = vec![];
        for pair in pairs {
            let (a,b) = (pair[0], pair[1]);
            graph.entry(a).and_modify(|x| {x.insert(b);}).or_insert_with(|| {
                let mut ret = HashSet::new();
                ret.insert(b); ret
            });
            graph.entry(b).and_modify(|x| {x.insert(a);}).or_insert_with(|| {
                let mut ret = HashSet::new();
                ret.insert(a); ret
            });
        }
        for n in graph.keys() {
            all_nodes.push(*n);
        }
        rec(&mut graph, all_nodes)
    }
}
fn main() {}
mod test;
struct Solution;
