#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        x: u128,
        m: u128
    }
    let mut sequence = vec![0u128; n];
    sequence[0] = x;
    let mut ans = 0;
    for i in 0..n {
        if sequence[i] == 0 {
            break;
        }
        sequence[i + 1] = (sequence[i] * sequence[i]) % m;
        ans += sequence[i + 1];
    }
    println!("{}", ans);
}
