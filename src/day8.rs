use std::fs;

pub fn day_8() {
  let filename = "day8.txt";
  let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
  let lines = contents.split('\n').map(|x| x.to_string()).collect::<Vec<String>>();
  let mut count = 0;
  let mut output_sum = 0;
  for line in lines {
    let input_output = line.split('|').map(|x| x.to_string()).collect::<Vec<String>>();
    let input_letters = input_output[0].clone().split(' ').map(|x| x.to_string()).filter(|x| x!="").collect::<Vec<String>>();
    let output_letters = input_output[1].clone().split(' ').map(|x| x.to_string()).filter(|x| x!="").collect::<Vec<String>>();
    let mut zero = String::new();
    let mut one = String::new();
    let mut two = String::new();
    let mut three = String::new();
    let mut four = String::new();
    let mut five = String::new();
    let mut six = String::new();
    let mut seven = String::new();
    let mut eight = String::new();
    let mut nine = String::new();
    let mut unknown_numbers = vec![String::new(); 0];

//  0000
// 6    1
// 6    1
//  3333
// 4    2
// 4    2
//  5555

    input_letters.iter().for_each(|x| {
      let length = x.len();
      let sorted_x = sort_string(&x);
      match length {
        // 1 uses two segments
        2 => {
          one = sorted_x;
        },
        // 4 uses 4 segments
        4 => {
          four = sorted_x;
        },
        // 7 uses 3 segments
        3 => {
          seven = sorted_x;
        },
        // 8 uses 7 segments
        7 => {
          eight = sorted_x;
        },
        _ => {
          unknown_numbers.push(sorted_x);
        }
      }
    });
  
    for number in unknown_numbers {
      let mut chars_shared_with_one = 0;
      let mut chars_shared_with_four = 0;
      let mut chars_shared_with_seven = 0;
      let mut chars_shared_with_eight = 0;
      let chars = number.chars().collect::<Vec<char>>();
      for char in chars {
        if one.contains(char) {
          chars_shared_with_one += 1;
        }
        if four.contains(char) {
          chars_shared_with_four += 1;
        }
        if seven.contains(char) {
          chars_shared_with_seven += 1;
        }
        if eight.contains(char) {
          chars_shared_with_eight += 1;
        }
 
      } 
  
      match (chars_shared_with_one, chars_shared_with_four, chars_shared_with_seven, chars_shared_with_eight) {
        (1, 2, 2, 5) => {
          two = number.clone();
        },
        (2, 3, 3, 5) => {
          three = number.clone();
        },
        (1, 3, 2, 5) => {
          five = number.clone();
        },
        (1, 3, 2, 6) => {
          six = number.clone();
        },
        (2, 4, 3, 6) => {
          nine = number.clone();
        },
        (2, 3, 3, 6) => {
          zero = number.clone();
        },
        _ => {
        }
      }
    }

    let mut output_vec = Vec::new();
    output_letters.iter().for_each(|y| {
      let output_number = sort_string(&y.clone());
      if output_number == one {
        count += 1;
        output_vec.push('1')
      } else if output_number == two {
        output_vec.push('2')
      } else if output_number == three {
        output_vec.push('3')
      }else if output_number == four {
        count += 1;
        output_vec.push('4')
      }else  if output_number == five {
        output_vec.push('5')
      } else if output_number == six {
        output_vec.push('6')
      } else if output_number == seven {
        count += 1;
        output_vec.push('7')
      } else if output_number == eight {
        count += 1;
        output_vec.push('8')
      } else if output_number == nine {
        output_vec.push('9')
      } else if output_number == zero {
        output_vec.push('0');
      }
    });
    let output_string = output_vec.iter().collect::<String>();
    output_sum += output_string.parse::<i32>().unwrap();
  }

  println!("Day 8 ans: {}", count);
  println!("Day 8 part 2 ans: {}", output_sum);

}

fn sort_string(input: &str) -> String {
  let mut chars = input.chars().collect::<Vec<char>>();
  chars.sort();
  return chars.into_iter().collect::<String>()
}