use rand::Rng;

fn main() {
    let rand_raw: f64 = rand::thread_rng().gen_range(0.00..=10.00) as f64;
    let rand_floor: f64 = (rand_raw * 100.0).floor() / 100.0;
    let rand_ceil: f64 = (rand_raw * 100.0).ceil() / 100.0;
    let rand_round: f64 = (rand_raw * 100.0).round() / 100.0;

    println!("{:.5}", rand_raw);
    println!("{:.5}", rand_floor);
    println!("{:.5}", rand_ceil);
    println!("{:.5}", rand_round);
}
