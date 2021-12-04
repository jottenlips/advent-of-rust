use std::fs;

pub fn day_2() {
    let filename = "day2.txt";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let lines = contents.split('\n');

    let mut depth = 0;
    let mut forward_distance = 0;
  
    lines.for_each(|s|{
      let mut words = s.split(' ');
      let direction:String =  words.next().unwrap().to_string();
      let distance = words.next().unwrap().to_string().parse::<i32>().unwrap();
      match &direction as &str {
        "forward" => {
          forward_distance += distance;
        },
        "up" => {
          depth -= distance;
        },
        "down" => {
          depth += distance;
        },
        _ => {
          println!("Unknown direction: {}", direction);
        }
      }
    });
    let position = depth * forward_distance;
    println!("Day 2 answer: {:?}", position);
    day_2_pt_2()
}

pub fn day_2_pt_2() {
    let filename = "day2pt2.txt";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let lines = contents.split('\n');

    let mut depth = 0;
    let mut forward_distance = 0;
    let mut aim = 0;
  
    lines.for_each(|s|{
      let mut words = s.split(' ');
      let direction:String =  words.next().unwrap().to_string();
      let distance = words.next().unwrap().to_string().parse::<i32>().unwrap();
      match &direction as &str {
        "forward" => {
          forward_distance += distance;
          depth += distance * aim;
        },
        "up" => {
          aim -= distance;
        },
        "down" => {
          aim += distance;
        },
        _ => {
          println!("Unknown direction: {}", direction);
        }
      }
    });
    let position = depth * forward_distance;
    println!("Day 2 pt 2 answer: {:?}", position);
}