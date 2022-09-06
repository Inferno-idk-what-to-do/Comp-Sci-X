fn main() {
    let a : i32 = 64;
    let b : i32 = 4;

    let c : i32 = (a + b) / 2;
    let d : i32 = b.pow(2) - c.pow(2);
    let e : i32 = (c - a) / (2 * a - 8);
    let f : i32 = b.pow(3) - c.pow(2);

    println!("a = {}", a);
    println!("a = {}", b);
    println!("a = {}", c);
    println!("a = {}", d);
    println!("a = {}", e);
    println!("a = {}", f);
}
