use std::fs;
// use std::fmt;

#[derive(Clone, Debug)]
struct Point {
    up: u32,
    down: u32,
    left: u32,
    right: u32,
    center: u32,
    // location
    i: usize,
    j: usize,
}

trait PointTrait {
    fn is_center_low_point(&self) -> bool;
    fn search_point(&self, points: &Vec<Point>, count: u32) -> u32;
}

impl PointTrait for Point {
  fn is_center_low_point(&self) -> bool {
    self.center < self.up && self.center < self.down && self.center < self.left && self.center < self.right
  }
  // start count at 1
  fn search_point(&self, points:&Vec<Point>, count: u32) -> u32 {
      if self.left != 9 {
        let left_point = points.iter().find(|p| p.i == self.i && p.j == self.j - 1).unwrap();
        return left_point.search_point(points, count + 1);
      }
      if self.right != 9 {
        let right_point = points.iter().find(|p| p.i == self.i && p.j == self.j + 1).unwrap();
        return right_point.search_point(points, count + 1);
      }
      if self.down != 9 {
        let down_point = points.iter().find(|p| p.i == self.i + 1 && p.j == self.j).unwrap();
        return down_point.search_point(points, count + 1);
      }
      if self.up != 9 {
        let up_point = points.iter().find(|p| p.i == self.i - 1 && p.j == self.j).unwrap();
        return up_point.search_point(points, count + 1);
      }
      return count;
  }
}


// impl fmt::Debug for Point {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//       write!(f, "up {}, down {}, left {}, right {}, center {} \n is low:{:?} at i{} j{}", self.up, self.down, self.left, self.right, self.center, self.is_center_low_point(), self.i, self.j)
//     }
// }

pub fn day_9 () {
  let filename = "day9.txt";
  let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
  let lines = contents.split('\n').map(|x| x.to_string()).collect::<Vec<String>>();
  let line_length = lines[0].len();
  let number_of_lines = lines.len();
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
        center: number,
        i: i,
        j: j,
      };
      points.push(point);
    }
  }

  let (low_points, risk) = find_low_points_and_risk(points);
  println!("Day 9 pt 1: {}", risk);

  let mut basins = Vec::new();
  for point in low_points {
    basins.push(point.search_point(points, 1));
  }
  println!("Day 9 pt 2: {:?}", basins);
}


fn find_low_points_and_risk(points: Vec<Point>) -> (Vec<Point>, u32) {
  let mut low_points:Vec<Point> = Vec::new();
  let mut risk = 0;
  for point in points {
    if point.is_center_low_point() {
            risk += point.center + 1;
      low_points.push(point);
    } 
  }
  return (low_points, risk);
}