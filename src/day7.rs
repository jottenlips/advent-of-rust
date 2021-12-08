use std::fs;
// use std::fmt;

struct CrabSubmarine {
  initial_position: i32,
}

trait CrabFunctions {
  fn calculate_fuel_cost_estimate(&self, distance: i32) -> i32;
  fn calculate_real_fuel_cost(&self, distance: i32) -> i32;
}

impl CrabFunctions for CrabSubmarine {
  fn calculate_fuel_cost_estimate(&self, distance: i32) -> i32 {
    return (distance - self.initial_position).abs();
  }
  fn calculate_real_fuel_cost(&self, distance: i32) -> i32 {
    let change = (distance - self.initial_position).abs();
    return (change * (change+1)) / 2
  }
}

pub fn day_7 () {
    let filename = "day7.txt";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let positions = contents.split(',').map(|s| s.parse::<i32>().unwrap());
    let crab_subs = positions.clone().map(|p| CrabSubmarine { initial_position: p });
    let max_number_of_moves = positions.clone().max().unwrap();
    let min_number_of_moves = positions.clone().min().unwrap();
    // set to a high number
    let mut min_fuel_cost = std::i32::MAX;
    let mut alignment_parameter = -1;
    for i in min_number_of_moves..max_number_of_moves {
      let cost_to_meet_at_positions:i32 = crab_subs.clone().map(|c| c.calculate_fuel_cost_estimate(i)).sum();
      if cost_to_meet_at_positions < min_fuel_cost {
        alignment_parameter = i;
        min_fuel_cost = cost_to_meet_at_positions;
      }
    }
    println!("Day 7 ans: {:?} {:?}", alignment_parameter, min_fuel_cost);
    day_7_pt2();
}

// same input 
fn day_7_pt2 () {
    let filename = "day7.txt";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let positions = contents.split(',').map(|s| s.parse::<i32>().unwrap());
    let crab_subs = positions.clone().map(|p| CrabSubmarine { initial_position: p  });
    let max_number_of_moves = positions.clone().max().unwrap();
    let min_number_of_moves = positions.clone().min().unwrap();
    // set to a high number
    let mut min_fuel_cost = std::i32::MAX;
    let mut alignment_parameter = -1;
    for i in min_number_of_moves..max_number_of_moves {
      let cost_to_meet_at_positions:i32 = crab_subs.clone().map(|c| c.calculate_real_fuel_cost(i)).sum();
      if cost_to_meet_at_positions < min_fuel_cost {
        alignment_parameter = i;
        min_fuel_cost = cost_to_meet_at_positions;
      }
    }
  println!("Day 7 ans pt2: {:?} {:?}", alignment_parameter, min_fuel_cost); 
}
