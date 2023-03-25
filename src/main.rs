use std::io;

fn can_drive(age:i32)->bool{
    age >= 16
}

fn main() -> io::Result<()> {
    println!("Please enter your name");
    let mut name = String::new();
    io::stdin().read_line(&mut name)?;
    name = name.trim().to_string();

    println!("How old are you?");
    let mut age = String::new();
    io::stdin().read_line(&mut age)?;
    let age:i32 = age.trim().parse().expect("Error converting age to a number");
    let drive = if can_drive(age){
        "can drive"
    } else {
        "cannot drive yet"
    };

    println!("Hi {name}, you are {age} years old. You {drive}.");
    Ok(())
}

