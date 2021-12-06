use std::fs;

struct FloorMap {
    topography: Vec<Vec<i32>>,
    width: usize,
    height: usize,
}

pub fn day_5() {
    let filename = "day5.txt";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let lines = contents.split('\n');
    let mut largest_x = 0;
    let mut largest_y = 0;
    for line in lines {
      let mut coords = line.split(" -> ");
      let mut first_coord = coords.next().unwrap().split(',').map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
      let mut second_coord = coords.next().unwrap().split(',').map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
      println!("{:?} to {:?} ", first_coord, second_coord);
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
    };

    println!("{:?} by {:?} ", largest_x, largest_y);


}