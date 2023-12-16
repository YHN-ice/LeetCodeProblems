
// leetcode 1719
impl Solution {
    pub fn check_ways(pairs: Vec<Vec<i32>>) -> i32 {
        // set relation

        // for a node v denote its connected nodes as in pairs as S(v) = adj(v) \cup {v}
        // for node u,v, if u MUST be ancestor of v, then S(v) is subset of S(u)
        //               if u and v can be mutual ancestor/replaced, S(u) = S(v)
        //               if u and v are not connected by pairs, i.e., in different subtrees, S(u) \notsubseteq S(v) \land S(v) \notsubseteq S(u)
        // thus for a valid tree, we can construct a linear order for all distinct S(.), additionally we return 2 if
        //      for any S(v) there are more than 1 corresponding v
        // So if we can construct a tree from pairs, then each u,v in pair should satisfy
        //      S(u) \subseteq S(v) \lor S(v) \subseteq S(u), \forall u,v in pairs
        // furthur if both sides holds, S(u)=S(v), then we could return 2
        // otherwise, conflict encountered and return 0
        let mut bm = [[0_u128;4]; 501];
        for pair in &pairs {
            let (a,b) = (pair[0] as usize, pair[1] as usize);
            bm[a][b / 128] |= 1<<(b % 128);
            bm[b][a / 128] |= 1<<(a % 128);

            bm[b][b / 128] |= 1<<(b % 128);
            bm[a][a / 128] |= 1<<(a % 128);
        }
        let cnt_bit = |x:&[u128;4]| {x.iter().map(|y| y.count_ones()).sum()};
        let v_cnt = bm.iter().filter(|x| cnt_bit(x) != 0).count() as u32;
        if bm.iter().map(cnt_bit).max() != Some(v_cnt) {return 0}
        let subseteq = |a:[u128;4], b:[u128;4]| {
            let res = a.iter().zip(b.iter()).map(|(x,y)| x & y);
            res.eq(a.into_iter())
        };
        let mut mult = false;
        for pair in &pairs {
            let (a,b) = (pair[0] as usize, pair[1] as usize);
            if !(subseteq(bm[a],bm[b])||subseteq(bm[b],bm[a])) {
                return 0;
            }
            if subseteq(bm[a],bm[b])&&subseteq(bm[b],bm[a]) {
                eprintln!("{:?}, {:?}", bm[a], bm[b]);
                mult = true
            }
        }
        if mult {2} else {1}
    }
}
fn main() {}
mod test;
struct Solution;
