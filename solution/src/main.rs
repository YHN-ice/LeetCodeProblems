#[derive(Debug)]
struct Trie<> {
    ptr: [usize; 26],
    term: Option<String>
}

impl Trie {
    fn new() -> Self {
        Trie{ptr:[0;26], term:None}
    }
    fn build(dict:Vec<String>) -> (Vec<Trie>, usize) {
        let mut storage = vec![Trie::new()];
        let root = 0;
        dict.into_iter().for_each(|x| {
            let mut cur = root;
            x.bytes().map(|x| (x-b'a') as usize).for_each(|ch| {
                if storage[cur].ptr[ch] == 0 {
                    storage[cur].ptr[ch] = storage.len();
                    storage.push(Trie::new());
                }
                cur = storage[cur].ptr[ch];
            });
            storage[cur].term = Some(x)
        });
        (storage,root)
    }
}

fn get_shape(board:&Vec<Vec<char>>) -> (usize, usize) {
    (board.len(), board.first().map(|x| x.len()).unwrap_or(0))
}

fn backtrack(trie:&mut Vec<Trie>, root:usize, board:&Vec<Vec<char>>, pos:(usize,usize)) -> impl Iterator<Item = String> {
    let (n,m) = get_shape(&board);
    let mut vis = vec![vec![false;m];n];
    let mut ret = vec![];
    fn dfs(cur:usize, trie:&mut Vec<Trie>, board:&Vec<Vec<char>>, pos:(usize, usize), vis:&mut Vec<Vec<bool>>, ret:&mut Vec<String>) {
        let (n,m) = get_shape(board);
        let (x,y) = pos;
        let cur_ch = (board[x][y] as u8 - b'a') as usize;
        match trie[cur].ptr[cur_ch] {
            0=>return,
            id => {
                // hit on id
                vis[x][y] = true;
                if let Some(str_ref) = trie[id].term.take() {ret.push(str_ref)}

                if trie[id].ptr.iter().all(|&x| x == 0) {
                    vis[x][y] = false;
                    trie[cur].ptr[cur_ch] = 0;
                    return
                }

                let diff = [-1,0,1,0,-1];
                diff.iter().zip((&diff[1..]).into_iter()).for_each(|(dx, dy)|{
                    let (x,y) = (x as i32+dx, y as i32+dy);
                    if !(0..n as i32).contains(&x) || !(0..m as i32).contains(&y) || vis[x as usize][y as usize]{ return }
                    dfs(id, trie, board, (x as usize, y as usize), vis, ret)
                });
                vis[x][y] = false;
            }
        }
    }
    dfs(root, trie, board, pos, &mut vis, &mut ret);
    ret.into_iter()
}
impl Solution {
    pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let (mut trie, root) = Trie::build(words);
        let (n,m) = get_shape(&board);
        (0..n).flat_map(|x:usize| (0..m).map(move |y:usize| (x,y))).flat_map(|(x,y)| {
            backtrack(&mut trie, root, &board, (x,y))
        }).collect()
    }
}

struct Solution;


fn main() {

    // let board = vec![vec!['o','a','a','n'],vec!['e','t','a','e'],vec!['i','h','k','r'],vec!['i','e','a','t']];
    // let words = vec!["oath","pea","eat","rain"];
    // assert_eq!(vec!["oath","eat"].into_iter().map(|x| x.to_owned()).collect::<Vec<String>>(), Solution::find_words(board, words.into_iter().map(|x| x.to_owned()).collect()));
    let board = vec![vec!['a','b']];
    let words = vec!["a", "b"];
    assert_eq!(vec!["a","b"].into_iter().map(|x| x.to_owned()).collect::<Vec<String>>(), Solution::find_words(board, words.into_iter().map(|x| x.to_owned()).collect()));
}
