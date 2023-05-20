fn add(number1: i32, number2: i32) {
    let answer = number1 + number2;
    println!("{number1} + {number2} = {answer}");
}

fn sub(number1: i32, number2: i32) {
    let answer = number1 - number2;
    println!("{number1} - {number2} = {answer}");
}

fn multiply(number1: i32, number2: i32) {
    let answer = number1 * number2;
    println!("{number1} x {number2} = {answer}");
}

fn main() {
    add(6, 6);
    add(100, 900);
    add(2, 3);
    add(8, 1);

    sub(8, 1);
    sub(1, 2);
    sub(7, 9);
    sub(7, 2);

    multiply(1,2);
    multiply(3,5);
}
