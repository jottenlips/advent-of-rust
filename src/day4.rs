use std::fs;
use std::fmt;
use std::pin::Pin;

trait BoardActions {
    fn check_win(&self, items: Vec<&str>) -> bool;
}

struct Board<'a> {
    board: Vec<Vec<&'a str>>,
    width: usize,
    height: usize,
}

impl BoardActions for Board<'_> {
    fn check_win(&self, items: Vec<&str>)->bool {
      false
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
    let boards:Vec<Board> = board_inputs.iter().map(|board_input| {
        return Board {
            board: board_input.to_vec(),
            width: board_input[0].len(),
            height: board_input.len(),
        };
    }).collect();

   
    println!("bingo: {:?}, {:?}",bingo_numbers,boards[2]);
    

}