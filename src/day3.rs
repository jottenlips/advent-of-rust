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
  
    let filename = "day3pt2.txt";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let lines = contents.split('\n');
    let line_count = lines.clone().count() as u32;

    let mut oxygen_ratings:Vec<String> = lines.clone().map(|s| s.to_string()).collect();
    let mut co2_ratings:Vec<String> = lines.clone().map(|s| s.to_string()).collect();

    let first_line = contents
      .split_whitespace()
      .next()
      .unwrap_or("");
    let line_length = first_line.len();

    let mut counts = vec![0; line_length];



    lines.for_each(|x|{
      let bits = x.chars().map(|x| x.to_digit(10).unwrap()).collect::<Vec<u32>>();
      for (i, bit) in bits.iter().enumerate() {
        counts[i] += bit;
      }
    });



    for (i, count) in counts.iter().enumerate() {
     
      oxygen_ratings = oxygen_ratings.iter().filter(|line| {
          let has_1 = line.chars().nth(i).unwrap() == '1';
          let has_0 = line.chars().nth(i).unwrap() == '0';
          let mut oxygen_counts:Vec<u32> = vec![0; line_length];
          let oxygen_line_count = oxygen_ratings.clone().iter().count() as f64;
          oxygen_ratings.iter().clone().for_each(|x|{
            let bits = x.chars().map(|x| x.to_digit(10).unwrap()).collect::<Vec<u32>>();
            for (j, bit) in bits.iter().enumerate() {
              oxygen_counts[j] += bit;
            }
          });

          let half = oxygen_line_count/2.0;
          let more_o_1s = (oxygen_counts[i] as f64) > half;
          let more_o_0s = (oxygen_counts[i] as f64) < half;
          let equal_o_0_1 = (oxygen_counts[i] as f64) == half;

          if has_1 && (more_o_1s || equal_o_0_1) {
            return true
          } 
          if has_0 && (more_o_0s) {
            return true
          }
          if oxygen_ratings.iter().count() == 1 {
            return true
          }
          return false
      }).cloned().collect();

        co2_ratings = co2_ratings.iter().filter(|line| {
          let has_1 = line.chars().nth(i).unwrap() == '1';
          let has_0 = line.chars().nth(i).unwrap() == '0';
          let mut co2_counts:Vec<u32> = vec![0; line_length];
          let co2_line_count = co2_ratings.clone().iter().count() as f64;
          co2_ratings.iter().clone().for_each(|x|{
            let bits = x.chars().map(|x| x.to_digit(10).unwrap()).collect::<Vec<u32>>();
            for (j, bit) in bits.iter().enumerate() {
              co2_counts[j] += bit;
            }
          });
          let half = co2_line_count/2.0;
          let more_1s = (co2_counts[i] as f64) > half;
          let more_0s = (co2_counts[i] as f64) < half;
          let equal_0_1 = (co2_counts[i] as f64) == half;

          if has_0 && (more_1s || equal_0_1) {
              return true
          } 
          if has_1 && (more_0s) {
            return true
          }
          if co2_ratings.iter().count() == 1 {
              return true
          }
          return false
        }).cloned().collect();
    }
    let oxygen_rating = oxygen_ratings.iter().next().unwrap();
    let co2_rating = co2_ratings.iter().next().unwrap();
    let life_support_score = binary_to_decimal(oxygen_rating) * binary_to_decimal(co2_rating);

    println!("Day 3 pt 2 ans: {:?}", life_support_score);

}

