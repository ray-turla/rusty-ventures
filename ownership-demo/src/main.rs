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

    // SLICES -> reference to a sequence of elements in a collection

    // Implementation w/0 slices
    // let mut s = String::from("Symphony of Destruction");
    // let word = first_word(&s);
    // s.clear();
    // println!("S: '{}', Word: '{}'", s, word);

    // String Slice -> reference to a part of string
    // let s = String::from("Hello World");
    // let hello = &s[..5];
    // let world = &s[6..];
    // let whole_slice = &s[..];
    // println!("{}, {}, {}", hello, world, whole_slice);

    // REFACTOR IMPLEMENTATION W/O SLICE USING SLICE
    // let mut s = String::from("Symphony of Destruction");
    // let word = first_word(&s);
    // // s.clear();
    // println!("S: '{}', Word: '{}'", s, word);

    // SLICE FOR STRING LITERAL
    // let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    // let word = first_word(&my_string[0..6]);
    // let word = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    // let word = first_word(&my_string);

    // let my_string_literal = "hello world";
    // `first_word` works on slices of string literals, whether partial or whole
    // let word = first_word(&my_string_literal[0..6]);
    // let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    // let word = first_word(my_string_literal);

    // ARRAY SLICE
    let arr = [1,2,3,4,5];
    let slice = &arr[1..3];

    println!("{:?}, {:?}", slice, &[2,3]);
    assert_eq!(slice, [2,3]);
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

// IMPLEMENTATION WITHOUT SLICE
// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }

//     s.len()
// }

// REFACTORED USING SLICE   
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i]
        }
    }
    &s[..]
}

