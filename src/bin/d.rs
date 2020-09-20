use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[allow(unused_imports)]
use std::collections::HashMap;
#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        range: [(usize, usize); k],

    }
    let MOD_SAMPLE = 998244353;
    let mut s = vec![false; n];
    let mut ans = 0;
    for (i, j) in range.iter() {
        if i == j {
            s[*i] = true;
        } else {
            for k in *i..=*j {
                s[k] = true;
            }
        }
    }
    for i in 0..n {
        if s[n - 1 - i] == false {
            continue;
        } else {
            s[n - 1 - i] = false;
        }
        let mut list: HashMap<usize, usize> = HashMap::new();
        let mut d = 0;
        let amari = n % (n - 1 - i);
        let shou = n / (n - 1 - i);
        if amari == 0 {
            ans += 1;
        } else {
        }
    }
}
fn rec(n: usize, d: usize, s: Vec<bool>) -> usize {
    0
}
