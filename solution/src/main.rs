mod test;

pub struct Solution;

trait Linear<T> {
    fn lget(&self, i:usize) -> &T ;

    fn lset(&mut self, i:usize)-> &mut T ;
}
impl Linear<i32> for Vec<Vec<i32>> {
    fn lget(&self, i:usize) -> &i32 {
        let n = self[0].len();
        &self[i/n][i%n]
    }
    fn lset(&mut self, i:usize)-> &mut i32 {
        let n = self[0].len();
        &mut self[i/n][i%n]
    }
}

const  MOD:i32 = 12345;

impl Solution {
    pub fn construct_product_matrix(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let fold_n = |data,  n:usize| -> Vec<Vec<i32>> {
            let mut ret = vec![];
            let mut buf = vec![];
            for d in data {
                if buf.len()==n {ret.push(buf.clone()); buf.clear();}
                buf.push(d);
            }
            if buf.len()>0 {ret.push(buf);}
            ret
        };
        let nm = grid.len()*grid[0].len();
        let (mut acc, mut racc) = (1, 1);
        let pref = (0..nm).rev().map(|x| {acc = acc*(if x== nm-1 {1} else {grid.lget(x+1)%MOD})%MOD; acc}).collect::<Vec<_>>();
        fold_n(
            (0..nm).map(|i| {
                let ret = (pref[nm-i-1] * racc) %MOD;
                racc = racc * (grid.lget(i) %MOD) %MOD;

                ret
            }), grid[0].len())
    }
}


fn main() {}
