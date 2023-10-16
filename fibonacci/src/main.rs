use std::env::args;

fn main() {
    let args: Vec<String> = args().collect();

    let min: i32 = args[1].trim().parse().expect("Must be numeric");
    let max: i32 = args[2].trim().parse().expect("Must be numeric");
    
    for i in min..=max {
        println!("{}", fibonacci(i))
    }
}


fn fibonacci(num: i32) -> i32 {
    if num < 2 {
        return num;
    }
    return fibonacci(num - 1) + fibonacci(num - 2);
}