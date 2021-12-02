use std::fs;

pub fn day_1() {
    let filename = "day1.txt";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let number_strings = contents.split('\n');
    let numbers: Vec<i32> = number_strings.map(|s| s.parse::<i32>().unwrap()).collect();
    let mut depth_increase_count = 0;
    for (i, number) in numbers.iter().enumerate() {
        if i as i32 + 1 < numbers.iter().len() as i32{
            let next_number = &numbers[i+1];
            if number < next_number {
                depth_increase_count += 1;
            }
        }

    }
    println!("Day 1 ans: {}", depth_increase_count);
}

