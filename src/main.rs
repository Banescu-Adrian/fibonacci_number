use std::io;

fn main() {
    println!("Fibonnaci number!");
    println!("Enter a number!");

    let mut input_number = String::new();

    io::stdin()
        .read_line(&mut input_number)
        .expect("Failed to read line");

    let input_number: i32 = input_number
        .trim()
        .parse()
        .expect("Must be a number");

    println!("Fibonnaci number is: {:?}", fibonnaci_number(input_number));
}

fn fibonnaci_number(n: i32) -> i32 {
    if n <= 1 {
        return n;
    }

    return fibonnaci_number(n - 1) + fibonnaci_number(n - 2);
}
