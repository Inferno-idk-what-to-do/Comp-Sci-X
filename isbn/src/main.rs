// this has to be one of the worst remakes of this program but it works

fn main() {
    let isbn: String = String::from("0-912517-31-X");
    let check_digit: char = isbn.chars().last().unwrap();

    println!("ISBN Number -> {}", isbn);
    println!("Check digit from isbn -> {}", check_digit);
    println!("Check digit from calc -> {}", calc_check_digit(&isbn));
}

fn calc_check_digit(isbn: &String) -> char {
    let mut isbn_clean = String::from(isbn.replace("-", ""));
    isbn_clean.pop();

    let mut sum = 0;

    for i in 0..=8 {
        sum += (10 - i) * (isbn_clean.chars().nth(i as usize).unwrap() as u32 - 0x30);
    }

    let digit = 10 - ((sum - 1) % 11);
    let digit_char = "0123456789X".chars().nth(digit as usize).unwrap();

    return digit_char;
}
