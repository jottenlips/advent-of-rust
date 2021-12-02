mod day1;
mod day2;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = args[1].parse::<u32>().unwrap();

    match day {
        1 => {
            day1::day_1();
        },
        2 => {
            day2::day_2();
        },
        _ => {println!("Day not found");}
    }
}