use begin_rust::user_input::get_number_w_prompt;
use rand::Rng;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut score = 0;
    let mut total_questions = 0;
    let first_number = rand::thread_rng().gen_range(1..=50);
    let second_number = rand::thread_rng().gen_range(1..=50);
    let correct_answer = first_number + second_number;

    let user_answer = get_number_w_prompt(&format!("{first_number} + {second_number} = "))?;

    total_questions += 1;
    if user_answer == correct_answer {
        println!("You are correct!");
        score += 1;
    } else {
        println!("You are wrong. The correct answer was {correct_answer}")
    }

    let first_number = rand::thread_rng().gen_range(1..=50);
    let second_number = rand::thread_rng().gen_range(1..=50);
    let correct_answer = first_number + second_number;

    let user_answer = get_number_w_prompt(&format!("{first_number} + {second_number} = "))?;

    total_questions += 1;
    if user_answer == correct_answer {
        println!("You are correct!");
        score += 1;
    } else {
        println!("You are wrong. The correct answer was {correct_answer}")
    }

    let first_number = rand::thread_rng().gen_range(1..=50);
    let second_number = rand::thread_rng().gen_range(1..=50);
    let correct_answer = first_number + second_number;

    let user_answer = get_number_w_prompt(&format!("{first_number} + {second_number} = "))?;

    total_questions += 1;
    if user_answer == correct_answer {
        println!("You are correct!");
        score += 1;
    } else {
        println!("You are wrong. The correct answer was {correct_answer}")
    }

    let first_number = rand::thread_rng().gen_range(1..=50);
    let second_number = rand::thread_rng().gen_range(1..=50);
    let correct_answer = first_number + second_number;

    let user_answer = get_number_w_prompt(&format!("{first_number} + {second_number} = "))?;

    total_questions += 1;
    if user_answer == correct_answer {
        println!("You are correct!");
        score += 1;
    } else {
        println!("You are wrong. The correct answer was {correct_answer}")
    }

    let first_number = rand::thread_rng().gen_range(1..=50);
    let second_number = rand::thread_rng().gen_range(1..=50);
    let correct_answer = first_number + second_number;

    let user_answer = get_number_w_prompt(&format!("{first_number} + {second_number} = "))?;

    total_questions += 1;
    if user_answer == correct_answer {
        println!("You are correct!");
        score += 1;
    } else {
        println!("You are wrong. The correct answer was {correct_answer}")
    }

    println!("You're score was {score} out of {total_questions}");

    Ok(())
}
