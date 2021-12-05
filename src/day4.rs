use std::fs;

pub fn day_4() {
    let filename = "day4.txt";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let mut lines = contents.split("\n\n");
    let bingo_line = lines.next().unwrap();
    let bingo_numbers:Vec<&str> = bingo_line.split(',').collect();

    let board_strings:Vec<&str> = lines.collect();
    let number_of_boards = board_strings.len();
    let boards:Vec<Vec<Vec<&str>>> = board_strings.iter().map(|board_string| {
        return board_string.split("\n").collect::<Vec<&str>>().iter().map(|row| {
            return row.split(" ").filter(|&x| x != "").collect();
        }).collect();
    }).collect();
   
    println!("bingo: {:?}, {:?}",bingo_numbers,boards);
    

}