fn main() {
    let horizontal_line = format!("o{}o", "-".repeat(78));
    let vertical_line = format!("|{}|", " ".repeat(78));
    println!("{horizontal_line}");
    let mut i = 0;
    while i < 38 {
        println!("{vertical_line}");
        i += 1;
    }
    println!("{horizontal_line}");
}
