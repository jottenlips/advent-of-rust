use std::fs;

pub fn day_1() {
    let filename = "day1.txt";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let number_strings = contents.split('\n');
    let numbers: Vec<i32> = number_strings.map(|s| s.parse::<i32>().unwrap()).collect();
    println!("Day 1 ans: {}", get_depth_increases(numbers));
    day_1_pt_2();
}

fn get_depth_increases(numbers: Vec<i32>) -> i32 {
    let mut depth_increase_count = 0;
    for (i, number) in numbers.iter().enumerate() {
        if i as i32 + 1 < numbers.iter().len() as i32{
            let next_number = &numbers[i+1];
            if number < next_number {
                depth_increase_count += 1;
            }
        }
    }
    return depth_increase_count;
}

pub fn day_1_pt_2() {
    let filename = "day1pt2.txt";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let number_strings = contents.split('\n');
    let numbers: Vec<i32> = number_strings.map(|s| s.parse::<i32>().unwrap()).collect();
    let mut sums: Vec<i32> = vec![];
    for (i, _number) in numbers.iter().enumerate() {
        let sum_of_three: i32 = [0, 1, 2].iter().map(|x| get_value(numbers.clone(), *x as usize, i)).sum();
        sums.push(sum_of_three);
    }

    println!("Day 1 pt2 ans: {:?}", get_depth_increases(sums));
}

fn get_value (numbers: Vec<i32>, index: usize, offset: usize) -> i32 {
    if (index as i32) + (offset as i32)  < numbers.iter().len() as i32 {
        return numbers[index + offset];
    }
    return 0
}