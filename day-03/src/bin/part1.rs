// use ndarray::{Array, ArrayView, array};
use regex::Regex;
use std::cmp::{min, max};

fn main() {
    let input = include_str!("./input3.txt");
    let output = part1(input);
    println!("{}", output);
}


fn part1(input: &str) -> u32 {
    let extracted_parts: Vec<&str> = input
        .lines()
        .collect();

    let mut sum: u32 = 0;
    let number_re = Regex::new(r"\d+").unwrap();
    let nrows: usize = extracted_parts.len();
    let ncols: usize = extracted_parts[0].len();

    for i in 0..nrows {
        let row = &extracted_parts[i];
        let numbers = number_re.find_iter(row);
        'numloop: for number in numbers {

            // check the previous row
            if i > 0  { 
                let prev_row = &extracted_parts[i-1];
                for j in max(number.start().saturating_sub(1), 0)..min(number.end()+1, ncols) {
                    let c = prev_row.chars().nth(j).unwrap();
                    if c != '.' && !c.is_digit(10) {
                        sum = sum + number.as_str().parse::<u32>().unwrap();
                        continue 'numloop;
                    }
                }
            }

            // check the next row
            if i < (nrows-1)  {
                let next_row = &extracted_parts[i+1];
                for j in max(number.start().saturating_sub(1), 0)..min(number.end()+1, ncols) {
                    let c = next_row.chars().nth(j).unwrap();
                    if c != '.' && !c.is_digit(10) {
                        sum = sum + number.as_str().parse::<u32>().unwrap();
                        continue 'numloop;
                    }
                }
            }

            // check the previous char
            let j = number.start();
            if j > 0 {
                let c = row.chars().nth(j-1).unwrap();
                if c != '.' && !c.is_digit(10) {
                    sum = sum + number.as_str().parse::<u32>().unwrap();
                    continue 'numloop;
                }        
            }

            // check the next char
            let j = number.end();
            if j < ncols {
                let c = row.chars().nth(j).unwrap();
                if c != '.' && !c.is_digit(10) {
                    sum = sum + number.as_str().parse::<u32>().unwrap();
                    continue 'numloop;
                }        
            }

        }
    }

    return sum;
}
