fn main() {
    let cost : i32 = 54_39;
    let paid : i32 = 100_00;

    let change : i32 = paid - cost;

    let nums : [i32; 9] = calc(&change);

    println!("Cost -> ${}", (cost as f32 / 100.0));
    println!("Paid -> ${}", (paid as f32 / 100.0));
    println!("Change -> ${}\n", (change as f32 / 100.0));

    println!("$50 bills -> {}", nums[0]);
    println!("$20 bills -> {}", nums[1]);
    println!("$10 bills -> {}", nums[2]);
    println!("$5 bills -> {}", nums[3]);
    println!("$1 bills -> {}", nums[4]);
    println!("Quarters -> {}", nums[5]);
    println!("Dimes -> {}", nums[6]);
    println!("Nickels -> {}", nums[7]);
    println!("Pennies -> {}", nums[8]);
}

fn calc(change : &i32) -> [i32; 9] {
    let mut result : [i32; 9] = [0; 9];
    let mut change_int : i32 = *change;

    result[0] = change_int / 50_00;
    change_int -= - 50_00 * result[0];

    result[1] = change_int / 20_00;
    change_int -= 20_00 * result[1];

    result[2] = change_int / 10_00;
    change_int -= 10_00 * result[2];

    result[3] = change_int / 5_00;
    change_int -= 5_00 * result[3];

    result[4] = change_int / 1_00;
    change_int -= 1_00 * result[4];

    result[5] = change_int / 0_25;
    change_int -= 0_25 * result[5];

    result[6] = change_int / 0_10;
    change_int -= 0_10 * result[6];

    result[7] = change_int / 0_05;
    change_int -= 0_05 * result[7];

    result[8] = change_int;

    return result;
}
