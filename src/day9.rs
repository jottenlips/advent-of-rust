use std::fs;
// use std::fmt;

#[derive(Clone, Copy, Debug)]
struct Point {
    up: u32,
    down: u32,
    left: u32,
    right: u32,
    center: u32,
    visited: bool,
    // location
    i: usize,
    j: usize,
}

trait PointTrait {
    fn is_center_low_point(&self) -> bool;
    // fn search_point(&mut self, points: &Vec<Point>, count: u32, visited: Vec<(u32, u32)>) -> u32;
}

impl PointTrait for Point {
  fn is_center_low_point(&self) -> bool {
    self.center < self.up && self.center < self.down && self.center < self.left && self.center < self.right
  }
  // start count at 1
  // fn search_point(&mut self, mut points:&Vec<Point>, count: u32, mut visited: Vec<(u32, u32)>) -> u32 {
  //     println!("searching point: {:?}", self);
  //     if visited.contains(&(self.i as u32, self.j as u32)) {
  //       return count;
  //     }
  //     visited.push((self.i as u32, self.j as u32));
  //     if self.left < 9 {
  //       let mut left_point = points.iter().find(|p| p.i == self.i && p.j == self.j - 1).unwrap().clone();
  //       let mut remove_index = points.iter().position(|p| p.i == self.i && p.j == self.j - 1).unwrap();
  //       points.clone().remove(remove_index);
  //       return left_point.search_point(points, count + 1, visited);
  //     }
  //     if self.right < 9 {
  //       let mut right_point = points.iter().find(|p| p.i == self.i && p.j == self.j + 1).unwrap().clone();
  //       let mut remove_index = points.iter().position(|p| p.i == self.i && p.j == self.j + 1).unwrap();
  //       points.clone().remove(remove_index);
  //       return right_point.search_point(points, count + 1, visited);
  //     }
  //     if self.down < 9 {
  //       let mut down_point = points.iter().find(|p| p.i == self.i + 1 && p.j == self.j).unwrap().clone();
  //       let mut remove_index = points.iter().position(|p| p.i == self.i + 1 && p.j == self.j).unwrap();
  //       points.clone().remove(remove_index);
  //       return down_point.search_point(points, count + 1, visited);
  //     }
  //     if self.up < 9 {
  //       let mut up_point = points.iter().find(|p| p.i == self.i - 1 && p.j == self.j).unwrap().clone();
  //       let mut remove_index = points.iter().position(|p| p.i == self.i - 1 && p.j == self.j).unwrap();
  //       points.clone().remove(remove_index);
  //       return up_point.search_point(points, count + 1, visited);
  //     }
  //     return count;
  // }
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
        visited: false,
        i: i,
        j: j,
      };
      points.push(point);
    }
  }

  let (low_points, risk) = find_low_points_and_risk(&points);
  println!("Day 9 pt 1: {}", risk);

  let mut basins:Vec<u32> = Vec::new();
  for _point in low_points {
    let mut points_to_search:Vec<Point> = points.clone();

    while points_to_search.len() > 0 {
      let mut current_point = points_to_search.pop().unwrap();
      if current_point.visited {
        continue;
      } else {
        current_point.visited = true;
        if current_point.center != 9 {
          basins.push(current_point.center);
          if current_point.left < 9 {
            let left_point = points_to_search.iter().find(|p| p.i == current_point.i && p.j == current_point.j - 1);
            if left_point.is_some() {
              let mut left_point = left_point.unwrap().clone();
              if !left_point.visited {
                left_point.visited = true;
                points_to_search.push(left_point);
              }
            }
          }
            let right_point = points_to_search.iter().find(|p| p.i == current_point.i && p.j == current_point.j + 1);
            if right_point.is_some() {
              let mut right_point = right_point.unwrap().clone();
              if !right_point.visited {
                right_point.visited = true;
                points_to_search.push(right_point);
              }
            }
          
            let  down_point = points_to_search.iter().find(|p| p.i == current_point.i + 1 && p.j == current_point.j);
            if down_point.is_some() {
              let mut down_point = down_point.unwrap().clone();
              if !down_point.visited {
                down_point.visited = true;
                points_to_search.push(down_point);
              }
            }
   
            // let mut up_point = points_to_search.iter().find(|p| p.i == current_point.i - 1 && p.j == current_point.j);
            // if up_point.is_some() {
            //   let mut up_point = up_point.unwrap().clone();
            //   if !up_point.visited {
            //     up_point.visited = true;
            //     points_to_search.push(up_point);
            //   }
            // }
          
        }
      }


    }
    // basins.push(start_point.search_point(&points.clone(), 1, visited));
    
  }
  println!("Day 9 pt 2: {:?}", basins);
}


fn find_low_points_and_risk(points: &Vec<Point>) -> (Vec<Point>, u32) {
  let mut low_points:Vec<Point> = Vec::new();
  let mut risk = 0;
  for point in points {
    if point.is_center_low_point() {
            risk += point.center + 1;
      low_points.push(*point);
    } 
  }
  return (low_points, risk);
}