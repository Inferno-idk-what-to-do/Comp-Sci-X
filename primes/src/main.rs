fn main() {
    let result : Vec<i32> = calc(300);

    for i in result.iter() {
        println!("{}", i);
    }
}

fn calc(end: i32) -> Vec<i32> {
    let mut nums: Vec<i32> = (2..end).collect();

    let mut outer = 0;

    while !(outer > (nums.len() - 1)) {
        let mut inner = outer + 1;

        while !(inner > (nums.len() - 1)) {
            if nums[inner] % nums[outer] == 0 {
                nums.remove(inner);
            }

            inner += 1;
        }

        outer += 1;
    }

    nums
}
