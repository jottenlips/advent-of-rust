use std::fs;

pub fn day_1() {
    let filename = "day1.txt";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let number_strings = contents.split('\n');
    let numbers: Vec<i32> = number_strings.map(|s| s.parse::<i32>().unwrap()).collect();
    let mut depth_increase_count =  0;
    for slice in numbers.chunks(2)  {
        if slice[0] < slice[1] {
            depth_increase_count += 1
        }
    }
    println!("Depth increase count: {}", depth_increase_count);
}

