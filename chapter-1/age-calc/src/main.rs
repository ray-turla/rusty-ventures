use chrono::{NaiveDate, Utc};
use std::{env, process::{exit, ExitCode}, str::FromStr};

fn main() -> Result<(), &'static str> {
    let arg = &env::args().collect::<Vec<_>>()[1];
    let age = Utc::now().date_naive().years_since(NaiveDate::from_str(arg).unwrap()).unwrap();

    println!("{age}");
    return Ok(());
}

/**
 * Validate a birthdate format. Enables either yyyy-mm-dd or yyyy/mm/dd format;
 */
fn validate_date_format(arg: &String) -> Result<char, &'static str> {
    let error_message = "Invalid Format. Birthdate must follow either yyyy-mm-dd or yyyy/mm/dd";
    if arg.matches("-").collect::<Vec<_>>().len() == 2 {
        Ok('-')
    } else if arg.matches("/").collect::<Vec<_>>().len() == 2 {
        Ok('/')
    } else {
        Err(error_message)
    }
}