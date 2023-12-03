use regex::Regex;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);

    println!("{}", output);
}


fn reverse_ascii_string(string: &str) -> String {
    string.chars().rev().collect::<String>()
}


fn digit_replace(input_string: &str) -> String {
    let patt = "one|two|three|four|five|six|seven|eight|nine";
    let re = Regex::new(patt).unwrap();
    let mut input_string_1 = input_string.to_string();
    
    let matches: Vec<_> = re.find_iter(input_string).collect();

    if matches.len()==0 {
        // no need to do any replacements
        return input_string_1;
    }

    let first_match = matches[0];
    // Grab the first matched substring in the original string
    let replacement = match first_match.as_str() {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        &_ => first_match.as_str()
    };

    // Replace the first matched substring in the original string
    input_string_1.replace_range(first_match.start()..first_match.end(), &replacement);


    // Match the reverse string
    let rev = Regex::new(&reverse_ascii_string(patt)).unwrap();

    let mut input_string_rev = reverse_ascii_string(input_string);
    let input_string_rev_copy = input_string_rev.clone();

    let last_match = rev.find(&input_string_rev_copy).unwrap();
    let replacement = match last_match.as_str() {
        "eno" => "1",
        "owt" => "2",
        "eerht" => "3",
        "ruof" => "4",
        "evif" => "5",
        "xis" => "6",
        "neves" => "7",
        "thgie" => "8",
        "enin" => "9",
        &_ => last_match.as_str()
    };

    // Replace the last matched substring in the original string
    input_string_rev.replace_range(last_match.start()..last_match.end(), &replacement);

    format!("{}{}", input_string_1, reverse_ascii_string(&input_string_rev))
}

fn part2(input: &str) -> u32 {
    // Map a function over the iterator to extract digits from each word and parse as integers
    let extracted_digits: Vec<u32> = input
    .split_whitespace()
    .map(digit_replace)
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
