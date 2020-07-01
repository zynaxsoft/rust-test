use std::convert::TryInto;
use test2::ABC;

fn main() {
    let mut a = std::num::Wrapping(20i8);
    a += std::num::Wrapping(127);
    println!("{}", a);
}
