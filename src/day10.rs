use std::fs;

pub fn day_10() {
  let filename = "day9.txt";
  let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
  let lines = contents.split('\n').map(|x| x.to_string()).collect::<Vec<String>>();
  let open_parens = ['(', '{', '['];
  let close_parens = [')', '}', ']'];
  for line in lines {
    println!("{}", line);
    let mut parens = Vec::new();
    for (i, char) in line.chars().enumerate() {
      println!("{}", char);
      let last_open_paren = parens.last();

      let index_of_open = open_parens.iter().position(|&x| x == char);
      let index_of_close = close_parens.iter().position(|&x| x == char);
      let index_of_last_open_paren = open_parens.iter().position(|&x| &x == last_open_paren.unwrap());

      if open_parens.contains(&char) {
        parens.push(char);
      } else if close_parens.contains(&char) && index_of_last_open_paren == index_of_close {
        parens.pop();
      }
          println!("parens: {:?}", parens);
    }
    println!("parens left: {:?}", parens);
  }
}