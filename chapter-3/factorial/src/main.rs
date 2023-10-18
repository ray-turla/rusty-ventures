use std::env;
fn main() {
    println!("{}", factorial(env::args().collect::<Vec<_>>()[1].parse().expect("Argument must be a number")))
}

fn factorial(num: u32) -> u32 {
    if num < 2 { return 1 }
    num * factorial(num - 1)
}
