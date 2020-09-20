#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        d: [(u8, u8); n],
    }
    let mut ans = false;
    let mut count = 0;
    for i in d.iter() {
        if i.0 == i.1 {
            count += 1;
            if count == 3 {
                ans = true;
                break;
            }
        } else {
            count = 0;
        }
    }
    if ans {
        println!("Yes");
    } else {
        println!("No");
    }
}
