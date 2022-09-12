// this has to be one of the worst remakes of this program but it works

use std::convert::TryInto;

fn main() {
    let isbn: String = String::from("0-7645-0417-7");
    let check_digit: char = isbn.chars().last().unwrap();

    println!("ISBN Number -> {}", isbn);
    println!("Check digit from isbn -> {}", check_digit);
    println!("Check digit from calc -> {}", calc_check_digit(&isbn));
}

fn calc_check_digit(isbn: &String) -> char {
    let mut isbn_clean: String = String::from(isbn.replace("-", ""));
    isbn_clean.pop();

    let mut mult = 10;
    let mut sum = 0;
    for c in isbn_clean.chars() {
        sum += mult * (c as u32 - 0x30);
        mult -= 1;
    }

    let digit = 10 - ((sum - 1) % 11);
    let digit_char = "0123456789X".chars().nth(digit.try_into().unwrap()).unwrap();

    return digit_char;
}
