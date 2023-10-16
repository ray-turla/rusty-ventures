fn main() {
    for i in 0..=10 {
        println!("{}", fibonacci(i))
    }
}


fn fibonacci(num: i32) -> i32 {
    if num < 2 {
        return num;
    }
    return fibonacci(num - 1) + fibonacci(num - 2);
}