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
    let mut sixth = 0;
    let mut seventh = 0;
    let mut eighth = 0;
    let mut ninth = 0;
    let mut tenth = 0;
    let mut eleventh = 0;
    let mut twelfth = 0;

    let line_count = lines.clone().count() as u32;

    lines.for_each(|x|{
      let bits = x.chars().map(|x| x.to_digit(10).unwrap()).collect::<Vec<u32>>();
      for (i, bit) in bits.iter().enumerate() {
        match i {
            0 => first += bit,
            1 => second += bit,
            2 => third += bit,
            3 => fourth += bit,
            4 => fifth += bit,
            5 => sixth += bit,
            6 => seventh += bit,
            7 => eighth += bit,
            8 => ninth += bit,
            9 => tenth += bit,
            10 => eleventh += bit,
            11 => twelfth += bit,
            _ => panic!("More than five bits"),
        }
      }
    });
    let sums = [first, second, third, fourth, fifth, sixth, seventh, eighth, ninth, tenth, eleventh, twelfth];
    let gamma_bin:String = sums.iter().flat_map(|x| {
     return match x > &(line_count/2) {
        true => "1".chars(),
        false => "0".chars(),
     }
    }).collect();

    let epsilon_bin:String = sums.iter().flat_map(|x| {
     return match x > &(line_count/2) {
        true => "0".chars(),
        false => "1".chars(),
     }
    }).collect();

    let gamma = binary_to_decimal(&gamma_bin);
    let epsilon = binary_to_decimal(&epsilon_bin);
    let answer = gamma * epsilon;
    println!("Day 3 ans: {:?}", answer);      
}

fn binary_to_decimal(binary: &str) -> u32 {
  let decimal = u32::from_str_radix(binary, 2).unwrap();     
  return decimal
}