use std::fs;
use std::fmt;

struct Point {
    up: u32,
    down: u32,
    left: u32,
    right: u32,
    center: u32,
}

trait PointTrait {
    fn is_center_low_point(&self) -> bool;
}

impl PointTrait for Point {
    fn is_center_low_point(&self) -> bool {
      self.center < self.up && self.center < self.down && self.center < self.left && self.center < self.right
    }
}


impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "up {}, down {}, left {}, right {}, center {} \n is low:{:?}", self.up, self.down, self.left, self.right, self.center, self.is_center_low_point())
    }
}

pub fn day_9 () {
  let filename = "day9.txt";
  let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
  let lines = contents.split('\n').map(|x| x.to_string()).collect::<Vec<String>>();
  let line_length = lines[0].len();
  let number_of_lines = lines.len();
  let size = number_of_lines * line_length;

  let mut points:Vec<Point> = Vec::new();
  for (i,line) in lines.iter().enumerate() {  
    for (j, c) in line.chars().enumerate() {
      let number = c.to_digit(10).unwrap();
      let mut left = u32::max_value();
      let mut up = u32::max_value();
      let mut right = u32::max_value();
      let mut down = u32::max_value();
      if i < number_of_lines-1 {
        down = lines[i+1].chars().nth(j).unwrap().to_digit(10).unwrap();
      }
      if i > 0 {
        up = lines[i-1].chars().nth(j).unwrap().to_digit(10).unwrap();
      }
      if j < line_length-1 {
        right = line.chars().nth(j+1).unwrap().to_digit(10).unwrap();
      }
      if j > 0 {
        left = line.chars().nth(j-1).unwrap().to_digit(10).unwrap();
      }
      let point = Point {
        up: up,
        down: down,
        left: left,
        right: right,
        center: number
      };
      points.push(point);
    }
  }
  let mut risk = 0;
  for point in points {
    if point.is_center_low_point() {
      risk += point.center + 1;
    }
  }
  println!("Day 9 pt 1: {}", risk);
}
