
fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);

    println!("{}", output);
}


fn part1(input: &str) -> u32 {

    // Map a function over the iterator to extract digits from each word and parse as integers
    let extracted_digits: Vec<u32> = input
    .split_whitespace()
    .map(|word| word.chars().filter(|c| c.is_digit(10)).collect::<String>()) // Extract digits from each word
    .map(|word| {
        let first = word.chars().nth(0).unwrap();
        let last = word.chars().last().unwrap();
        format!("{}{}", first, last)
    })
    .filter_map(|digit_str| digit_str.parse::<u32>().ok()) // Parse each digit sequence as an integer
    .collect();

    let sum: u32 = extracted_digits.iter().sum();

    return sum
}
