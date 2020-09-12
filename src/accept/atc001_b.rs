use proconio::{input, fastout};

pub struct UnionFind {
    pars: Vec<usize>,
    rank: Vec<usize>,
}
impl UnionFind {
    pub fn new(n: usize) -> Self {
        UnionFind {
            pars: (0..n).collect(),
            rank: vec![1; n],
        }
    }
    pub fn find(&mut self, x: usize) -> usize {
        let p = self.pars[x];
        if x != p {
            self.pars[x] = self.find(p);
        }
        self.pars[x]
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
        self.pars[y] = x;
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
        q: usize,
        v: [(usize, usize, usize); q]
    }
    let mut u = UnionFind::new(n);
    for (p,a,b) in v {
        if p == 0 {
            u.unite(a,b);
        } else {
            println!("{}",
                if u.same(a,b) { "Yes" } else { "No" });
        }
    }
}
