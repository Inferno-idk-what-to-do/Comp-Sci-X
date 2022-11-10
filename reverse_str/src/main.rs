fn main() {
    println!("{}", reverse(String::from("Hello world!")));
}

fn reverse(str: String) -> String {
    let mut sum = String::from("");
    for i in (0..str.len()).rev() {
        sum.push(str.chars().nth(i).unwrap());
    }

    return sum;
}
