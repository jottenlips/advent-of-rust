use std::fs;



trait FishFunctions {
    fn decrement_time(&mut self);
    fn reproduce(&self);
}

struct LanternFish {
  age: i32,
  // time left until reproduction
  internal_time: i32,
  new: bool,
}

trait SchoolFuntions {
  add_fish(&mut self, LanternFish);
}

struct School {
  fish: Vec<LanternFish>,
}

pub fn day_6() {
    let filename = "day6.txt";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let ages = contents.split(',').map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    println!("{:?}", ages);

}