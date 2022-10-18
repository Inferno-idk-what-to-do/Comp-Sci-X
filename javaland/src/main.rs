use std::io::{self, Write};

fn main() {
    print!("enter income -> ");
    io::stdout().flush().unwrap(); // flush so print actually registers
    let mut income = String::from(""); // store input here
    io::stdin().read_line(&mut income).unwrap(); // take input
    let mut income_int: f64 = income.trim().parse().unwrap(); // convert to int

    // round to two decimal places
    income_int *= 100.0;
    income_int = income_int.round();
    income_int /= 100.0;

    let mut tax: f64 = 0.0;

    if income_int <= 10_000.0 {
        tax = income_int * 0.01;
    }
    else if income_int <= 30_000.0 {
        tax += 10_000.0 * 0.01; // first 10k tax
        tax += (income_int - 10_000.0) * 0.05;
    }
    else {
        tax += 10_000.0 * 0.01; // first 10k tax
        tax += 20_000.0 * 0.05; // next 20k tax
        tax += (income_int - 30_000.0) * 0.10;
    }

    if income_int > 1_000.0 {
        tax += 500.0;
    }

    // round to two decimal places
    tax *= 100.0;
    tax = tax.round();
    tax /= 100.0;

    println!("\nincome -> ${:.2}\ntax owed -> ${:.2}", income_int, tax);
}
