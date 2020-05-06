/* {{{ */
macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
    ($($r:tt)*) => {
        let s = {
            use std::io::Read;
            let mut s = String::new();
            std::io::stdin().read_to_string(&mut s).unwrap();
            s
        };
        let mut iter = s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
}

macro_rules! input_inner {
    ($iter:expr) => {};
    ($iter:expr, ) => {};
    ($iter:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($iter, $t);
        input_inner!{$iter $($r)*}
    };
}

macro_rules! read_value {
    ($iter:expr, ( $($t:tt),* )) => {
        ( $(read_value!($iter, $t)),* )
    };
    ($iter:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($iter, $t)).collect::<Vec<_>>()
    };
    ($iter:expr, chars) => {
        read_value!($iter, String).chars().collect::<Vec<char>>()
    };
    ($iter:expr, usize1) => {
        read_value!($iter, usize) - 1
    };
    ($iter:expr, $t:ty) => {
        $iter.next().unwrap().parse::<$t>().expect("Parse error")
    };
}
/* }}} */

fn main() {
    input! {
        s: chars
    }
    let mut b = true;
    let mut v: Vec<String> = Vec::new();
    for c in s {
        if c.is_uppercase() {
            if b {
                b = false;
                v.push(String::new());
            } else {
                b = true;
            }
        }
        let s = v.last_mut().unwrap();
        s.push(c);
    }

    v.sort_by(|ls: &String, rs: &String| {
        use std::cmp::Ordering;
        for (lc,rc) in ls.chars().zip(rs.chars()) {
            if lc == rc { continue; }
            let res = lc.to_lowercase().cmp(rc.to_lowercase());
            if res == Ordering::Equal {
                if lc.is_uppercase() {
                    return Ordering::Less;
                }
                if lc.is_lowercase() {
                    return Ordering::Greater;
                }
            } else {
                return res;
            }
        }
        ls.len().cmp(&rs.len())
    });

    println!("{}", v.join(""));
}
/* vim:set foldmethod=marker: */
