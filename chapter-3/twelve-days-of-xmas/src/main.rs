
// prints 12 days of christmas lyrics
fn main() {
    for i in 1..=12 {
        let position_suffix= get_position_suffix(i);
        println!("On the {i}{position_suffix} of Christmas, my true love sent to me");
        for j in (1..=i).rev() {
            let item = get_christmas_item(j);
            println!("{}", item);
        }
    }
}

fn get_christmas_item(num: i32) -> String {
    return match num {
        12 => "Twelve drummers drumming",
        11 => "Eleven pipers piping",
        10 => "Ten lords a-leaping",
        9 => "Nine ladies dancing",
        8 => "Eight maids a-milking",
        7 => "Seven swans a-swimming",
        6 => "Six geese a-laying",
        5 => "Five golden rings",
        4 => "Four calling birds",
        3 =>  "Three french hens",
        2 =>  "Two turtle doves, and",
        1 =>  "A partridge in a pear tree",
        _ => ""
    }.to_string()
}

fn get_position_suffix(number: i32) -> String {
    if (11..=13).contains(&(number % 100)) {
        return "th".to_string();
    }

    match number % 10 {
        1 => "st",
        2 => "nd",
        3 => "rd",
        _ => "th",
    }
    .to_string()
}
