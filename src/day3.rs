use std::fs;

pub fn day_3() {
    let filename = "day3.txt";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let lines = contents.split('\n');

    let mut first = 0;
    let mut second = 0;
    let mut third = 0;
    let mut fourth = 0;
    let mut fifth = 0;

    lines.for_each(|x|{
      let bits = x.chars().map(|x| x.to_digit(10).unwrap()).collect::<Vec<u32>>();
      bits.for_each(|x|{
     
      });
      println!("{:?}", bits);      
    });

}
