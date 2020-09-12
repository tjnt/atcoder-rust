use proconio::{input, fastout};
use proconio::marker::Usize1;

pub struct UnionFind {
    p: Vec<usize>,
    rank: Vec<usize>,
}
impl UnionFind {
    pub fn new(n: usize) -> Self {
        UnionFind {
            p: (0..n).collect(),
            rank: vec![1; n],
        }
    }
    pub fn find(&mut self, x: usize) -> usize {
        let p = self.p[x];
        if x != p {
            self.p[x] = self.find(p);
        }
        self.p[x]
    }
    pub fn unite(&mut self, x: usize, y: usize) -> bool {
        let mut x = self.find(x);
        let mut y = self.find(y);
        if x == y {
            return false;
        }
        if self.rank[x] < self.rank[y] {
            std::mem::swap(&mut x, &mut y);
        }
        self.rank[x] += self.rank[y];
        self.p[y] = x;
        true
    }
    pub fn same(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
    pub fn size(&mut self, x: usize) -> usize {
        let p = self.find(x);
        self.rank[p]
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        v: [(Usize1,Usize1);m]
    }
    let mut u = UnionFind::new(n);
    for (a,b) in v {
        u.unite(a,b);
    }
    let res = (0..n).map(|x| u.size(x)).max().unwrap();
    println!("{}", res);
}
