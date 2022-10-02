struct Point {
    x: f32,
    y: f32,
}

use rand::Rng;
fn gen_point() -> Point {
    let x = (rand::thread_rng().gen_range(0.00..=10.00) * 100.0 as f32).round() / 100.0;
    let y = (rand::thread_rng().gen_range(0.00..=10.00) * 100.0 as f32).round() / 100.0;

    Point {
        x,
        y,
    }
}

fn main() {
    let point1 = gen_point(); // wowee
    let point2 = gen_point();

    let distance = (calc_distance(&point1, &point2) * 100.0).round() / 100.0;

    println!("the distance between ({}, {}) and ({}, {}) is {}", point1.x, point1.y, point2.x, point2.y, distance);
}

fn calc_distance(point1: &Point, point2: &Point) -> f32 {
    (f32::powf(point2.x - point1.x, 2.0) + f32::powf(point2.y - point1.y, 2.0)).sqrt()
}
