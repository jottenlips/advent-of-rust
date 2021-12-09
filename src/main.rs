mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
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
        3 => {
            day3::day_3();
        },
        4 => {
            day4::day_4();
        },
        5 => {
            day5::day_5();
        },
        6 => {
            day6::day_6();
        },
        7 => {
            day7::day_7();
        },
        8 => {
            day8::day_8();
        }
        _ => {println!("Day not found");}
    }
}