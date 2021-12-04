use std::fs;

pub fn day_3() {
    let filename = "day3.txt";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let lines = contents.split('\n');

  
    let first_line = contents
      .split_whitespace()
      .next()
      .unwrap_or("");
    let line_length = first_line.len();

    let mut counts = vec![0; line_length];
    // let mut counts:Vec<u32> = [];
    println!("ll {:?}", counts);


    let line_count = lines.clone().count() as u32;

    lines.for_each(|x|{
      let bits = x.chars().map(|x| x.to_digit(10).unwrap()).collect::<Vec<u32>>();
      for (i, bit) in bits.iter().enumerate() {
        counts[i] += bit;
      }
    });
    let gamma_bin:String = counts.iter().flat_map(|x| {
     return match x > &(line_count/2) {
        true => "1".chars(),
        false => "0".chars(),
     }
    }).collect();

    let epsilon_bin:String = counts.iter().flat_map(|x| {
     return match x > &(line_count/2) {
        true => "0".chars(),
        false => "1".chars(),
     }
    }).collect();

    let gamma = binary_to_decimal(&gamma_bin);
    let epsilon = binary_to_decimal(&epsilon_bin);
    let answer = gamma * epsilon;
    println!("Day 3 ans: {:?}", answer);   
    day_3_pt2()   
}

fn binary_to_decimal(binary: &str) -> u32 {
  let decimal = u32::from_str_radix(binary, 2).unwrap();     
  return decimal
}

fn day_3_pt2() {
  
    // let filename = "day3pt2.txt";
    // let contents = fs::read_to_string(filename)
    //     .expect("Something went wrong reading the file");
    // let lines = contents.split('\n');

    
    // println!("Day 3 pt 2 ans: {:?}", lines);      

}