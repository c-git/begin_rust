use begin_rust::user_input::get_number_w_prompt;
use rand::Rng;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let first_number = rand::thread_rng().gen_range(1..=50);
    let second_number = rand::thread_rng().gen_range(1..=50);
    let correct_answer = first_number + second_number;

    let user_answer = get_number_w_prompt(&format!("{first_number} + {second_number} = "))?;

    if user_answer == correct_answer {
        println!("You are correct!")
    } else {
        println!("You are wrong. The correct answer was {correct_answer}")
    }

    Ok(())
}
