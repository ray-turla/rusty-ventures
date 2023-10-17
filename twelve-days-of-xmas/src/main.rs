fn main() {
    for i in 1..=12 {
        let (position_suffix, _) = match_pattern(i);
        println!("On the {i}{position_suffix} of Christmas, my true love sent to me");
        for j in (1..=i).rev() {
            let (_, item) = match_pattern(j);
            println!("{}", item);
        }
    }
}

fn match_pattern(pattern: i32) -> (&'static str, &'static str) {
    return match pattern {
        12 => ("th", "Twelve drummers drumming"),
        11 => ("th", "Eleven pipers piping"),
        10 => ("th", "Ten lords a-leaping"),
        9 => ("th", "Nine ladies dancing"),
        8 => ("th", "Eight maids a-milking"),
        7 => ("th", "Seven swans a-swimming"),
        6 => ("th", "Six geese a-laying"),
        5 => ("th", "Five golden rings"),
        4 => ("th", "Four calling birds"),
        3 => ("rd", "Three french hens"),
        2 => ("nd", "Two turtle doves, and"),
        1 => ("st", "A partridge in a pear tree"),
        _ => ("", "")
    }
}
