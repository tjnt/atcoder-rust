use proconio::{input, fastout};

pub trait LexicalPermutation {
    fn next_permutation(&mut self) -> bool;
    fn prev_permutation(&mut self) -> bool;
}
impl<T> LexicalPermutation for [T]
where
    T: Ord,
{
    fn next_permutation(&mut self) -> bool {
        if self.len() < 2 {
            return false;
        }
        let mut i = self.len() - 1;
        while i > 0 && self[i - 1] >= self[i] {
            i -= 1;
        }
        if i == 0 {
            return false;
        }
        let mut j = self.len() - 1;
        while j >= i && self[j] <= self[i - 1] {
            j -= 1;
        }
        self.swap(j, i - 1);
        self[i..].reverse();
        true
    }
    fn prev_permutation(&mut self) -> bool {
        if self.len() < 2 {
            return false;
        }
        let mut i = self.len() - 1;
        while i > 0 && self[i - 1] <= self[i] {
            i -= 1;
        }
        if i == 0 {
            return false;
        }
        self[i..].reverse();
        let mut j = self.len() - 1;
        while j >= i && self[j - 1] < self[i - 1] {
            j -= 1;
        }
        self.swap(i - 1, j);
        true
    }
}


#[fastout]
fn main() {
    input! {
        n: usize,
        k: u64,
        tt: [[u64;n];n]
    }
    let mut v = (1..n).collect::<Vec<usize>>();
    let mut res = 0;
    loop {
        let mut u = v.clone();
        u.insert(0, 0);
        u.push(0);
        let mut sum = 0;
        for i in 0..n {
            sum += tt[u[i]][u[i+1]] 
        }
        if sum == k {
            res += 1;
        }
        if !v.next_permutation() {
            break;
        }
    }
    println!("{}", res);
}
