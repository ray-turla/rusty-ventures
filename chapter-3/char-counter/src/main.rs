use std::env::args;
use base64::{Engine as _, engine::general_purpose};

// Converts argument to base64 and count its vowels.
fn main() {
    let arg_bs64 = general_purpose::STANDARD.encode(args().collect::<Vec<_>>()[0].to_string());
    println!("Encoded: {arg_bs64}, Count: {}", arg_bs64.chars().filter(|c: &char| "aeiouAEIOU".contains(*c)).count());

}
