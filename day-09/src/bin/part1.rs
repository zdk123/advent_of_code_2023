
fn main() {
    let input = include_str!("./input9.txt");
    let output = part1(input);
   println!("{}", output);
}

fn next_number(numbers: &Vec<i64>) -> i64 {
    let differences = numbers.windows(2).map(|w| w[1] - w[0]).collect::<Vec<_>>();

    let last = numbers.last().unwrap();

    if differences.iter().all(|&d| d == 0) {
        return *last;
    }

    let next_difference = next_number(&differences);
    *last + next_difference
}


fn part1(input: &str) -> i64 {

    let mut values: Vec<i64> = Vec::new();
    for line in input.lines() {
        let numbers: Vec<i64> = line
                        .split_whitespace()
                        .map(|str| str.parse().unwrap())
                        .collect();
        // println!("{:?}", numbers);
        let extrapolated_value = next_number(&numbers);
        // println!("{:?}", extrapolated_value);
        values.push(extrapolated_value);
    }
    // println!("{:?}", values);

    return values.iter().sum();
}


