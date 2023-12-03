use regex::Regex;
use std::cmp::max;

fn main() {
    let input = include_str!("./input2.txt");
    let output = part2(input);

    println!("{}", output);
}

fn evaluate_game(game: Vec<&str>) -> u32 {
    // Each string arrives as one part of the vector
    // let game_str = game[0];
    let sets_str = game[1];

    let sets = sets_str.split("; ").collect::<Vec<&str>>();
    let mut sets_iter = sets.iter();

    // set up regexes to parse the counts for each color cube
    let re_red = Regex::new(r"(\d+) red").unwrap();
    let re_green =Regex::new(r"(\d+) green").unwrap();
    let re_blue =Regex::new(r"(\d+) blue").unwrap();

    let mut red_count_max: u32 = 0;
    let mut green_count_max: u32 = 0;
    let mut blue_count_max: u32 = 0;

    while let Some(set) = sets_iter.next() {
        if let Some(mat_red) = re_red.captures(set) {
            red_count_max = max(red_count_max, mat_red.get(1).unwrap().as_str().parse().unwrap());
        }
        if let Some(mat_green) = re_green.captures(set) {
            green_count_max = max(green_count_max, mat_green.get(1).unwrap().as_str().parse().unwrap());
        }
        if let Some(mat_blue) = re_blue.captures(set) {
            blue_count_max = max(blue_count_max, mat_blue.get(1).unwrap().as_str().parse().unwrap());
        }
    }

    return red_count_max * green_count_max * blue_count_max;
}

fn part2(input: &str) -> u32 {
    let extracted_ids: Vec<u32> = input
        .lines()
        .map(|gamestr| {
            gamestr.split(": ")
            .collect::<Vec<&str>>()
           }
        )
        .map(evaluate_game)
        .collect();


    let sum: u32 = extracted_ids.iter().sum();

    return sum;
}
