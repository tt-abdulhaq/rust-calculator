use std::io;
fn main() {
    loop {
        let user_input: (f64, f64, char) = get_user_input();
        let n1: f64 = user_input.0;
        let n2: f64 = user_input.1;
        let o: char = user_input.2;
        if o == '+' {
            println!("{} + {} = {}", n1, n2, add(n1, n2))
        } else if o == '-' {
            println!("{} - {} = {}", n1, n2, sub(n1, n2))
        } else if o == '*' {
            println!("{} * {} = {}", n1, n2, multi(n1, n2))
        } else if o == '/' {
            println!("{} / {} = {}", n1, n2, divid(n1, n2))
        } else if o == 'C' {
            break;
        }
        else {
            println!("Wrong operand or Not support!(for exit press C)")
        }

    }
}
fn get_user_input() -> (f64, f64, char) {
    println!("Please enter the first number: ");
    let mut first_number: String = String::new();
    io::stdin()
        .read_line(&mut first_number)
        .expect("Failed to read line");
    let first_number: f64 = first_number.trim().parse().expect("Not a number");

    println!("Please enter the second number: ");
    let mut second_number: String = String::new();
    io::stdin()
        .read_line(&mut second_number)
        .expect("Failed to read line");
    let second_number: f64 = second_number.trim().parse().expect("Not a number");

    println!("Please enter the operand(+, _, *, /): ");
    let mut operand: String = String::new();
    io::stdin()
        .read_line(&mut operand)
        .expect("Failed to read line");
    let operand: char = operand.trim().parse().expect("not a char");

    (first_number, second_number, operand)
}

fn add(first_number: f64, second_number: f64) -> f64 {
    first_number + second_number
}
fn sub(first_number: f64, second_number: f64) -> f64 {
    first_number - second_number
}

fn multi(first_number: f64, second_number: f64) -> f64 {
    first_number * second_number
}

fn divid(first_number: f64, second_number: f64) -> f64 {
    first_number / second_number
}
