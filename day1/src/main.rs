fn calc_fuel(fuel: i32) -> i32 {
    let result = fuel / 3 - 2;
    if result < 0 {
        0
    } else {
        result + calc_fuel(result)
    }
}

fn main() {
    let ouput: i32 = include_str!("../input")
        .split('\n')
        .map(|x| calc_fuel(x.parse().unwrap()))
        .sum();
    println!("{}", ouput);
}
