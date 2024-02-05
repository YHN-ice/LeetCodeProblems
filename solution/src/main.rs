use std::{collections::{BTreeMap, btree_map::Iter, HashMap, BinaryHeap, hash_map::Entry}, ops::{AddAssign, SubAssign}, iter::Sum, hash::Hash, fmt::Debug, cmp::Reverse};
use core::iter::FromIterator;
#[derive(Debug)]
struct SortedList<T:Ord+Copy+Default+AddAssign+SubAssign> {
    data:BTreeMap<T, usize>,
}

impl<T:Ord+Copy+Default+AddAssign+SubAssign> SortedList<T> {
    fn new()->Self{
        Self{data:BTreeMap::new()}
    }

    fn add(&mut self, e: T) {
        self.data.entry(e).and_modify(|x| *x+=1).or_insert(1);
    }
    fn remove(&mut self, e: T) {
        self.data.entry(e).and_modify(|x| *x-=1);
        if let Some(&0) = self.data.get(&e) {
            self.data.remove(&e);
        }
    }
    fn max(&self) -> T {
        self.data.last_key_value().map(|x| *x.0).unwrap_or_default()
    }
    fn min(&self) -> T {
        self.data.first_key_value().map(|x| *x.0).unwrap_or_default()
    }
}

struct RefIter<'a, T:Ord+Copy+Default+AddAssign+SubAssign> {
    k:T,
    cnt: usize,
    i1: Iter<'a, T, usize>,
}

impl<'a, T:Ord+Copy+Default+AddAssign+SubAssign> Iterator for RefIter<'a, T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        match self.cnt {
            0=> match self.i1.next() {
                Some((&k,&v)) => {
                    self.k = k;
                    self.cnt = v-1;
                    Some(k)
                }
                None=> None,
            },
            _=> {
                self.cnt-=1;
                Some(self.k)
            }
        }
    }
}



impl<T:Ord+Copy+Default+AddAssign+SubAssign> FromIterator<T> for SortedList<T> {
    fn from_iter<A: IntoIterator<Item = T>>(iter: A) -> Self {
        let mut ret = Self::new();
        iter.into_iter().for_each(|x| ret.add(x));
        ret
    }
}

#[derive(Debug)]
struct DualHeap<T:Ord+Copy+Default+AddAssign+SubAssign, U: From<T>+Sum> {
    head:BinaryHeap<T>,
    tail:BinaryHeap<Reverse<T>>,
    k_sum:U,
    todo:HashMap<T,usize>,
}

impl<T:Ord+Copy+Default+AddAssign+SubAssign+Hash+Debug, U:From<T>+Sum+AddAssign+SubAssign+Debug> DualHeap<T,U> {
    fn new(init_slice: &[T])->Self {
        let head = init_slice.iter().map(|x| *x).collect();
        let tail = BinaryHeap::new();
        let k_sum = init_slice.iter().map(|x| (*x).into()).sum::<U>();
        let todo = HashMap::new();
        Self{head, tail, k_sum, todo}
    }

    fn extend(&mut self, augment_slice: &[T]) {
        augment_slice.into_iter().for_each(|x| self.add(*x));
    }

    fn poll(&mut self) {
        self.purge_tail();
        self.purge_head();
        let fill = self.tail.peek().unwrap().0;
        // dbg!(&self);
        self.k_sum += fill.into();
        self.head.push(fill);
        self.tail.pop();
    }
    fn pop(&mut self) {
        self.purge_tail();
        self.purge_head();
        let kick = *self.head.peek().unwrap();
        self.k_sum -= kick.into();
        self.head.pop();
        self.tail.push(Reverse(kick));
    }
    fn add(&mut self, e:T) {
        if e<*self.head.peek().unwrap() {
            self.head.push(e);
            self.k_sum += e.into();
            self.pop();
        } else {
            self.tail.push(Reverse(e));
        }
    }
    fn del(&mut self, e:T) {
        self.todo.entry(e).and_modify(|x| *x+=1).or_insert(1);
        if e<self.tail.peek().unwrap().0 {
            self.k_sum -= e.into();
            self.poll();
        }
    }
    fn k_sum(&mut self) -> &U {
        &self.k_sum
    }


    fn purge_head(&mut self) {
        while let Some(victim_counter) = self.todo.get_mut(&self.head.peek().map(|x| *x).unwrap_or_default()) {
            *victim_counter -= 1;
            if *victim_counter == 0 {self.todo.remove(&self.head.peek().map(|x| *x).unwrap_or_default());}
            self.head.pop();
        }
    }
    fn purge_tail(&mut self) {
        while let Some(victim_counter) = self.todo.get_mut(&self.tail.peek().map(|x| *x).unwrap_or_default().0) {
            *victim_counter -= 1;
            if *victim_counter == 0 {self.todo.remove(&self.tail.peek().map(|x| *x).unwrap_or_default().0);}
            self.tail.pop();
        }
    }
}


impl Solution {
    pub fn minimum_cost(nums: Vec<i32>, k: i32, dist: i32) -> i64 {
        let k = k as usize; let dist = dist as usize;
        let mut dual:DualHeap<i32, i64> = DualHeap::new(&nums[1..k]);
        dual.extend(&nums[k..2+dist]);
        let init = *dual.k_sum();
        // println!("initial, h:{:?}, t:{:?}, del:{:?} k_sum{}", dual.head, dual.tail, dual.todo, dual.k_sum);
        nums[0] as i64 + ((2+dist .. nums.len()).scan(dual, |acc, i| {
            // println!("add:{}, del:{}", nums[i], nums[i-dist-1]);
            acc.add(nums[i]);
            // println!("after add, h:{:?}, t:{:?}, del:{:?} k_sum{}", acc.head, acc.tail, acc.todo, acc.k_sum);
            acc.del(nums[i-dist-1]);
            // println!("after add del, h:{:?}, t:{:?}, del:{:?} k_sum{}", acc.head, acc.tail, acc.todo, acc.k_sum);
            // let mut range:Vec<i32> = (&nums[i-dist..=i]).iter().map(|x| *x).collect();
            // range.sort();
            Some(*acc.k_sum())

        }).min().unwrap_or(i64::MAX)).min(init)
    }
}

struct Solution;


fn main() {
    let __gcd__ = |mut a,mut b| {
        if a<b {std::mem::swap(&mut a, &mut b)};
        while a%b!=0 {
            let rem = a%b;
            a = b;
            b = rem
        }
        b
    };
    let mut m = HashMap::new();
    m.insert(1, 'b');
    let c:Entry<_,_> = m.entry(1);
    enum demo {
        A,B
    }
    impl demo {
        fn call_me(&self) {
            match self {
                &Self::A => println!("I am A"),
                &Self::B => println!("I am B"),
            }
        }
    }
    let acc = demo::A;
    acc.call_me();
    match acc {
        demo::A=>println!("i am A"),
        demo::B=>println!("i am B"),
    }
    assert_eq!(1, __gcd__(17,63));
    // assert_eq!(0, Solution::minimum_cost(vec![43,44,41,45,42,41,45,47,47,50,47,45,42,42,47,46,45,50,44,42,48,46,49,45,47,47,45,44,40,42,44,42,43,46,46,47,49,44,43,47,48,48,41,45,50,50,44,42,41,50], 7, 9));
    assert_eq!(0, Solution::minimum_cost(vec![41,50,42,48,50,41,48,50,40,41,43,44,42,42,41,43,45,50,44,43,44,44,49,48,47,44,43,49,42,42,50,45,40,49,46,46,43,48,42,45,41,44,47,40,48,41,48,41,41,49], 9, 34));

}
