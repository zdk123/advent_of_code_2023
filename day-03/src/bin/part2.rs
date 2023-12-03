use regex::Regex;
use std::cmp::{min, max};
use std::collections::HashMap;

fn main() {
    let input = include_str!("./input3.txt");
    let output = part2(input);
    println!("{}", output);
}


fn part2(input: &str) -> u32 {
    let extracted_parts: Vec<&str> = input
        .lines()
        .collect();

    let number_re = Regex::new(r"\d+").unwrap();
    let nrows: usize = extracted_parts.len();
    let ncols: usize = extracted_parts[0].len();

    let mut gears: HashMap<(usize, usize), Vec<u32>> = HashMap::new();

    for i in 0..nrows {
        let row = &extracted_parts[i];
        let numbers = number_re.find_iter(row);
        'numloop: for number in numbers {
            let num = number.as_str().parse::<u32>().unwrap();

            // check the previous row
            if i > 0  { 
                let prev_row = &extracted_parts[i-1];
                for j in max(number.start().saturating_sub(1), 0)..min(number.end()+1, ncols) {
                    let c = prev_row.chars().nth(j).unwrap();
                    if c == '*' {
                        gears.entry((i-1, j)).or_default().push(num);
                    }
                }
            }

            // check the next row
            if i < (nrows-1)  {
                let next_row = &extracted_parts[i+1];
                for j in max(number.start().saturating_sub(1), 0)..min(number.end()+1, ncols) {
                    let c = next_row.chars().nth(j).unwrap();
                    if c == '*' {
                        gears.entry((i+1, j)).or_default().push(num);
                    }
                }
            }

            // check the previous char
            let j = number.start();
            if j > 0 {
                let c = row.chars().nth(j-1).unwrap();
                if c == '*' {
                    gears.entry((i, j-1)).or_default().push(num);
                }
            }

            // check the next char
            let j = number.end();
            if j < ncols {
                let c = row.chars().nth(j).unwrap();
                if c == '*' {
                    gears.entry((i, j)).or_default().push(num);
                }
            }

        }
    }

    // Sum up gear ratios iff each gear has two gear numbers
    let sum = gears
        .iter()
        .map(|(_, v)| {
            if v.len() == 2 {
                return v[0]*v[1];
            } else {
                return 0;
            }
        })
        .sum();
    return sum;
}
