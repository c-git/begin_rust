use std::{
    error::Error,
    io::{self, Write},
};

pub fn get_number() -> Result<i32, Box<dyn Error>> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    let result: i32 = buffer.trim().parse()?;
    Ok(result)
}

pub fn get_number_w_prompt(prompt: &str) -> Result<i32, Box<dyn Error>> {
    print!("{prompt}");
    io::stdout().flush()?;
    get_number()
}

pub fn get_string() -> Result<String, Box<dyn Error>> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim().to_string())
}

pub fn get_string_w_prompt(prompt: &str) -> Result<String, Box<dyn Error>> {
    print!("{prompt}");
    io::stdout().flush()?;
    get_string()
}
