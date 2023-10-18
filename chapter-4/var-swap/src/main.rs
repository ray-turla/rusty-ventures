use std::{mem, default::Default};

// Make a program that swaps variables
fn main() {
    let mut x = String::from("Hello");
    let mut y = String::from("World");
    println!("x: {x}, y: {y}");
    var_swap::<String>(&mut x, &mut y);
    println!("x: {x}, y: {y}");
}

fn var_swap<T: Default>(x: &mut T, y: &mut T) {
    let temp: T = mem::replace(x, mem::replace(y, Default::default()));
    *y = temp;
}