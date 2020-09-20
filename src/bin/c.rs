#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut ans = 0;
    for i in 1..n {
        ans += (n - 1) / i;
    }
    println!("{}", ans);
}
