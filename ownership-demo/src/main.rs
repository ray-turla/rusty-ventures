fn main() {
    // STRING DEMO
    // let mut s = String::from("Hello");
    // println!("{s}");
    // s.push_str(" World!");
    // println!("{s}");

    // COPY/CLONE DEMO -> EXPENSIVE
    // let x = 5;
    // let y = x;
    // let s1 = String::from("hello");
    // let s2 = s1.clone();
    // println!("s1: {}, s2: {}", s1, s2);
    // println!("x: {}, y: {}", x, y);

    // OWN AND COPY
    // let s = String::from("Hello");
    // own(s);
    // println!("Main s: {}", s);
    // println!("Main s: {}", s);
    // let x = 5;
    // copy(x);
    // println!("Main x: {}", x);

    // TAKE AND RETURN OWNERSHIP
    // let s1 = String::from("hello");
    // let (s2, len) = calculate_length(s1);
    // println!("String: {}, Length: {}", s2, len);
    
    // REFERENCES (&)
    // let s1 = String::from("hello");
    // let len = calculate_length2(&s1);
    // println!("The length of '{}' is {}.", s1, len);
    // causes error -> can't modify borrowed reference
    // change(&s1);
    // println!("String: {}", s1);
    
    // MUTABLE REFERENCES(&)
    // let mut s1 = String::from("Hello");
    // println!("String Before: {}", s1);
    // change(&mut s1);
    // println!("String After: {}", s1);

    // MULTIPLE MUTABLE REFERENCES(&) -> ERROR -> CAN'T BORROW MUTABLE REFERENCES MORE THAN ONCE
    // let mut s = String::from("hello");

    // let r1 = &mut s;
    // let r2 = &mut s;

    // println!("{}, {}", r1, r2);



}

fn own(string: String) {
    println!("{}", string);
}

fn copy(int: i32) {
    println!("{}", int);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

// ACCEPTS A REFERENCE TO THE STRING
fn calculate_length2(s: &String) -> usize {
    s.len()
}

// THIS FUNCTION CAUSES ERROR -> CAN'T MODIFY BORROWED REFERENCE
// fn change(some_string: &String) {
//     some_string.push_str(", world");
// }

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}



