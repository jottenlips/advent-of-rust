use std::fs;
use std::fmt;

static mut FISH_COUNT: i32 = 0;

struct LanternFish {
  internal_time: i32,
}

trait FishFunctions {
    fn life_cycle(&mut self, days_left: i32);
    fn reproduce(&self) -> LanternFish;
}

impl FishFunctions for LanternFish {
    fn life_cycle(&mut self, days_left: i32) {
        if days_left == 0 {
          return;
        }
        let remaining_days = days_left - 1;
        if self.internal_time == 0 {
          self.internal_time = 6;
          let mut new_fish = self.reproduce();
          self.life_cycle(remaining_days);
          return new_fish.life_cycle(remaining_days);
        }
        self.internal_time -= 1;
        return self.life_cycle(remaining_days);
    }
    fn reproduce(&self) -> LanternFish {
        unsafe {
          FISH_COUNT +=1;
        }
        return LanternFish { internal_time: 8 }
    }
}

impl fmt::Debug for LanternFish {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "{}", self.internal_time) 
    }
}

// brute force solution
pub fn day_6() {
    let filename = "day6.txt";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let timers = contents.split(',').map(|s| s.parse::<i32>().unwrap());
    let mut lantern_fish = timers.map(|t| LanternFish { internal_time: t }).collect::<Vec<LanternFish>>();
    let days = 80;
    unsafe {
      FISH_COUNT = lantern_fish.len() as i32;
    }
    for fish in lantern_fish.iter_mut() {
        fish.life_cycle(days);
    }
    unsafe {
      println!("Day 6 ans: {:?}", FISH_COUNT);
    }
    day_6_pt_2();
}

// more efficent / mathy solution neccesary
pub fn day_6_pt_2() {
    let filename = "day6pt2.txt";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let days = 256;

    let mut reproduction_groups = [0; 9];
    
    let timers = contents.split(',')
      // use u128 to avoid overflow
      .map(|s| s.trim().parse::<u128>().unwrap());

    for timer in timers {
      // classify the fish
      reproduction_groups[timer as usize] += 1;
    }
    // lifecycle of fish
    for _day in 1..=days {
        reproduction_groups.rotate_left(1); // move fish groups to the left daily at 0, move back to 8
        let reproduced_fish_count = reproduction_groups[8]; // all of the fish that were in 0 move to 8 have new fish
        reproduction_groups[6] += reproduced_fish_count; // the number of fish that came from 0 needs to also be added to 6, these are the old fish
    }

    println!("Day 6 pt 2 ans: {:?}", reproduction_groups.iter().sum::<u128>());
}


