use std::fs;
use std::fmt;

trait BoardActions {
    fn mark_board(&mut self, items: &str);
    fn check_bingo(&self) -> bool;
    fn calculate_score(&self, last_call: &str) -> u32;
}

struct Board<'a> {
    board: Vec<Vec<&'a str>>,
    markers:  Vec<Vec<bool>>,
    width: usize,
    height: usize,
}

impl BoardActions for Board<'_> {
    fn mark_board(&mut self, item: &str) {
      for (i, row) in self.board.iter().enumerate() {
          for (j, cell) in row.iter().enumerate() {
            if &item == cell {
              self.markers[i][j] = true;
            }
          }
        }
    }
    fn check_bingo(&self) -> bool {
      // check horizontal
      for row in self.markers.iter() {
        let mut row_count = 0;
        for cell in row.iter() {
          if *cell {
            row_count += 1;
          }
          if row_count == 5 {
            return true;
          }
        }
      }
      // check vertical
      for x in 0..self.height {
        let mut col_count = 0;
        for y in 0..self.width {
          if self.markers[y][x] {
            col_count += 1;
          }
          if col_count == 5 {
            return true;
          }
        }
      }
      return false;
    }
    fn calculate_score(&self, last_call: &str) -> u32 {
      let mut score = 0;
      for x in 0..self.width {
        for y in 0..self.height {
          if !self.markers[x][y] {
            score += self.board[x][y].parse::<u32>().unwrap();
          }
        }
      }
      return score * last_call.parse::<u32>().unwrap();
    }


}

impl fmt::Debug for Board<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\n\n-----------------------\n")?;
        Ok(for row in &self.board {
            for cell in row {
              write!(f, "{} ", cell)?;
            }
            write!(f, "\n-----------------------\n")?;
        })
    }
}

pub fn day_4() {
    let filename = "day4.txt";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let mut lines = contents.split("\n\n");
    let bingo_line = lines.next().unwrap();
    let bingo_numbers:Vec<&str> = bingo_line.split(',').collect();

    let board_strings:Vec<&str> = lines.collect();
    let number_of_boards = board_strings.len();
    let board_inputs:Vec<Vec<Vec<&str>>> = board_strings.iter().map(|board_string| {
        return board_string.split("\n").collect::<Vec<&str>>().iter().map(|row| {
            return row.split(" ").filter(|&x| x != "" && x != "\n").collect();
        }).collect();
    }).collect();
    let mut boards:Vec<Board> = board_inputs.iter().map(|board_input| {
        let board = Board {
            board: board_input.to_vec(),
            markers: vec![vec![false; 5]; 5],
            width: board_input[0].len(),
            height: board_input.len(),
        };
        return board
    }).collect();

    let mut bingo = false;
    let mut current_board_number = 0;
    for bingo_number in bingo_numbers {
      for i in 0..number_of_boards {
        boards[i].mark_board(bingo_number);
        bingo = boards[i].check_bingo();
        current_board_number = i;
        if bingo {
          break;
        }
     
      }
      if bingo {
        println!("Day 4 answer: {}", boards[current_board_number].calculate_score(bingo_number));
        break;
      }
    }
  day_4_pt_2() 
}

fn day_4_pt_2() {
    let filename = "day4pt2.txt";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let mut lines = contents.split("\n\n");
    let bingo_line = lines.next().unwrap();
    let bingo_numbers:Vec<&str> = bingo_line.split(',').collect();

    let board_strings:Vec<&str> = lines.collect();
    let number_of_boards = board_strings.len();
    let board_inputs:Vec<Vec<Vec<&str>>> = board_strings.iter().map(|board_string| {
        return board_string.split("\n").collect::<Vec<&str>>().iter().map(|row| {
            return row.split(" ").filter(|&x| x != "" && x != "\n").collect();
        }).collect();
    }).collect();
    let mut boards:Vec<Board> = board_inputs.iter().map(|board_input| {
        let board = Board {
            board: board_input.to_vec(),
            markers: vec![vec![false; 5]; 5],
            width: board_input[0].len(),
            height: board_input.len(),
        };
        return board
    }).collect();

    let mut _bingo = false;
    
    let mut last_winning_board_number = 0;
    let mut winning_board_history = vec![];
    let mut last_winning_number = "";
    let mut _number_of_winners = 0;
    let mut break_loop = false;

    for bingo_number in bingo_numbers {
      for i in 0..number_of_boards {
        boards[i].mark_board(bingo_number);
        _bingo = boards[i].check_bingo();
        if _bingo && !winning_board_history.contains(&i) {
          last_winning_board_number = i;
          winning_board_history.push(i);
          last_winning_number = bingo_number;
          _number_of_winners += 1;
        }
        if winning_board_history.len() == number_of_boards {
          break_loop = true;
          break;
        }
      }
      if break_loop {
        break;
      }
    } 

    println!("Day 4 pt2 answer: {:?}", boards[last_winning_board_number].calculate_score(last_winning_number));
}