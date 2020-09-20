#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        mut s: Chars,
    }
    if s[s.len() - 1] != 's' {
        s.push('s');
    } else {
        s.push('e');
        s.push('s');
    }
    for i in s.iter() {
        print!("{}", i);
    }
}
