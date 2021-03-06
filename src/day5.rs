use std::fs;

use std::fmt;

trait MapActions {
    fn draw_line(&mut self, start:Vec<i32>, end:Vec<i32>, diaganol: bool);
    fn count_intersections(&self) -> i32;
}


struct FloorMap {
    topography: Vec<Vec<i32>>,
    width: usize,
    height: usize,
}



impl MapActions for FloorMap {
    fn draw_line(&mut self, start:Vec<i32>, end:Vec<i32>, diaganol: bool) {
      let start_x = if start[0] > end[0] { end[0] } else { start[0] };
      let start_y = if start[1] > end[1] { end[1] } else { start[1] };
      
      let end_x = if end[0] > start[0] { end[0] } else { start[0]};
      let end_y = if end[1] > start[1] { end[1] } else { start[1]};

      // vertical and horizontal lines
      if start[0] == end[0] || start[1] == end[1] {
        (start_y..end_y+1).for_each(|y| {
          (start_x..end_x+1).for_each(|x| {
              self.topography[y as usize][x as usize] += 1;
          });
        });
      } else if diaganol {
        let slope = (end[1] - start[1]) as f64 / (end[0] - start[0]) as f64;
        for x in start_x..end_x+1 {
          let y = (slope * (x as f64 - start[0] as f64)) + start[1] as f64;
          let y = y.round() as i32;
          if y >= 0 && y < self.height as i32 {
            self.topography[y as usize][x as usize] += 1;
          }
        }
      }
  }
    fn count_intersections(&self) -> i32 {
      let mut intersections = 0;
      for y in 0..self.height {
        for x in 0..self.width {
          if self.topography[y][x] > 1 {
            intersections += 1;
          }
        }
      }
      intersections
    }
}

impl fmt::Debug for FloorMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\n\n-----------------------\n")?;
        Ok(for row in &self.topography {
            for cell in row {
              if cell != &0 {
                write!(f, "{}", cell)?;
              } else {
                write!(f, ".")?;
              }
            }
            write!(f, "\n")?;
        })
    }
}


pub fn day_5() {
    let filename = "day5.txt";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let lines = contents.split('\n');
    let mut largest_x = 0;
    let mut largest_y = 0;
    let sea_floor_vent_coord_pairs = lines.map(|line| {
      let mut coords = line.split(" -> ");
      let first_coord = coords.next().unwrap().split(',').map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
      let second_coord = coords.next().unwrap().split(',').map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
      if first_coord[0] > largest_x {
        largest_x = first_coord[0];
      }  
      if second_coord[0] > largest_x {
        largest_x = second_coord[0];
      }
      if first_coord[1] > largest_y {
        largest_y = first_coord[1];
      }
      if second_coord[1] > largest_y {
        largest_y = second_coord[1];
      }
      return (first_coord, second_coord);
    }).collect::<Vec<(Vec<i32>, Vec<i32>)>>();

    let mut floor_map = FloorMap {
      topography: vec![vec![0; largest_x as usize + 1]; largest_y as usize + 1],
      width: largest_x as usize + 1,
      height: largest_y as usize + 1,
    };

    for sea_floor_vent_coord_pair in sea_floor_vent_coord_pairs {
      floor_map.draw_line(sea_floor_vent_coord_pair.0, sea_floor_vent_coord_pair.1, false);
    }

    println!("Day 5 ans: {:?}", floor_map.count_intersections());
    day_5_pt_2();
}

pub fn day_5_pt_2() {

  let filename = "day5pt2.txt";
  let contents = fs::read_to_string(filename)
      .expect("Something went wrong reading the file");
  let lines = contents.split('\n');
  let mut largest_x = 0;
  let mut largest_y = 0;
  let sea_floor_vent_coord_pairs = lines.map(|line| {
    let mut coords = line.split(" -> ");
    let first_coord = coords.next().unwrap().split(',').map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    let second_coord = coords.next().unwrap().split(',').map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    if first_coord[0] > largest_x {
      largest_x = first_coord[0];
    }  
    if second_coord[0] > largest_x {
      largest_x = second_coord[0];
    }
    if first_coord[1] > largest_y {
      largest_y = first_coord[1];
    }
    if second_coord[1] > largest_y {
      largest_y = second_coord[1];
    }
    return (first_coord, second_coord);
  }).collect::<Vec<(Vec<i32>, Vec<i32>)>>();

  let mut floor_map = FloorMap {
    topography: vec![vec![0; largest_x as usize + 1]; largest_y as usize + 1],
    width: largest_x as usize + 1,
    height: largest_y as usize + 1,
  };

  for sea_floor_vent_coord_pair in sea_floor_vent_coord_pairs {
    floor_map.draw_line(sea_floor_vent_coord_pair.0, sea_floor_vent_coord_pair.1, true);
  }

  println!("Day 5 pt 2 ans: {:?}", floor_map.count_intersections());

}