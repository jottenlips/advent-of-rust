mod day1;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = args[1].parse::<u32>().unwrap();

    match day {
        1 => {day1::day_1();}
        _ => {println!("Day not found");}
    }
}