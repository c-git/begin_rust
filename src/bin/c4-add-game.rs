use rand::Rng;
use std::io;
use std::io::Write;

fn main() -> io::Result<()> {
    let first_number = rand::thread_rng().gen_range(1..=50);
    let second_number = rand::thread_rng().gen_range(1..=50);
    let correct_answer = first_number + second_number;

    print!("{first_number} + {second_number} = ");
    io::stdout().flush()?;
    let mut user_answer = String::new();
    io::stdin().read_line(&mut user_answer)?;
    let user_answer: i32 = user_answer
        .trim()
        .parse()
        .expect("Error converting to a number");

    if user_answer == correct_answer {
        println!("You are correct!")
    } else {
        println!("You are wrong. The correct answer was {correct_answer}")
    }

    Ok(())
}
