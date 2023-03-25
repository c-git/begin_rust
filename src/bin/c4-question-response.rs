use std::error::Error;

use begin_rust::user_input::{get_number_w_prompt, get_string_w_prompt};

fn can_drive(age: i32) -> bool {
    age >= 16
}

fn main() -> Result<(), Box<dyn Error>> {
    let name = get_string_w_prompt("Please enter your name: ")?;
    let age = get_number_w_prompt("How old are you? ")?;
    let drive = if can_drive(age) {
        "can drive"
    } else {
        "cannot drive yet"
    };

    println!("Hi {name}, you are {age} years old. You {drive}.");
    Ok(())
}
