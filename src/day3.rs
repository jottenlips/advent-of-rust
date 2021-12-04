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

    let mut counts_oxygen:Vec<u32> = counts.clone();


    for (i, count) in counts.iter().enumerate() {

      let mut lines_removed = 0;
    
      oxygen_ratings = oxygen_ratings.iter().filter(|line| {
          let has_1 = line.chars().nth(i).unwrap() == '1';
          let has_0 = line.chars().nth(i).unwrap() == '0';
          
          let oxygen_line_count = oxygen_ratings.clone().iter().count() as u32;
          let total_lines_removed = line_count - oxygen_line_count;
        
          let more_o_1s = counts[i] - lines_removed  > (oxygen_line_count/2);
          let more_o_0s = counts[i] - lines_removed   < (oxygen_line_count/2);
          let equal_o_0_1 = counts[i] - lines_removed  == (oxygen_line_count/2);

     
          println!("{:?} {:?}", lines_removed,  oxygen_line_count);

          if has_1 && (more_o_1s || equal_o_0_1) {
            return true
          } 
          if has_0 && more_o_0s {
            return true
          }
          if oxygen_ratings.iter().count() == 1 {
            return true
          }
          lines_removed += 1;

          return false
      }).cloned().collect();
      lines_removed = 0;
      println!(" {:?}", oxygen_ratings);

    //   co2_ratings = co2_ratings.iter().filter(|line| {
    //       let has_1 = line.chars().nth(i).unwrap() == '1';
    //       let has_0 = line.chars().nth(i).unwrap() == '0';
          
    //       if has_0 && more_1s {
    //         return true
    //       } 
    //       if has_1 && (more_0s || equal_0_1) {
    //         return true
    //       }
    //       if co2_ratings.iter().count() == 1 {
    //         return true
    //       }
    //       // counts[i] - 1;
    //       return false
    //     }).cloned().collect();

    }
    let oxygen_rating = oxygen_ratings.iter().next().unwrap();
    // let co2_rating = co2_ratings.iter().next().unwrap();
    // let life_support_score = binary_to_decimal(oxygen_rating) * binary_to_decimal(co2_rating);

    println!("Day 3 pt 2 ans: {:?}", binary_to_decimal(oxygen_rating));

}

