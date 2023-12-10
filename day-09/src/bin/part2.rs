
fn main() {
    let input = include_str!("./input9.txt");
    let output = part2(input);
   println!("{}", output);
}

fn previous_number(numbers: &Vec<i64>) -> i64 {
    let differences = numbers.windows(2).map(|w| w[1] - w[0]).collect::<Vec<_>>();

    let first = numbers.first().unwrap();

    if differences.iter().all(|&d| d == 0) {
        return *first;
    }

    let prev_difference = previous_number(&differences);
    *first - prev_difference
}


fn part2(input: &str) -> i64 {

    let mut values: Vec<i64> = Vec::new();
    for line in input.lines() {
        let numbers: Vec<i64> = line
                        .split_whitespace()
                        .map(|str| str.parse().unwrap())
                        .collect();
        let extrapolated_value = previous_number(&numbers);
        values.push(extrapolated_value);
    }

    return values.iter().sum();
}


