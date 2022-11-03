fn main() {
    let years: u32 = 10;
    let mut balance: f64 = 1000.00;
    for rate in 6..=12 {
        println!("rate -> {}%", rate as f64 * 0.01);
        println!("balance @ year 0 -> ${}", balance);
        for year in 1..=years {
            balance += balance * (rate as f64 * 0.01);
            balance = (balance * 100.0).round() / 100.0;
            println!("balance @ year {} -> ${}", year, balance);
        }
        println!();
        balance = 1000.00;
    }
}
