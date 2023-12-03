use regex::Regex;

fn main() {
    let input = include_str!("./input2.txt");
    let output = part1(input);

    println!("{}", output);
}

fn evaluate_game(game: Vec<&str>) -> u32 {
    // Each string arrives as one part of the vector
    let game_str = game[0];
    let sets_str = game[1];

    let sets = sets_str.split("; ").collect::<Vec<&str>>();
    let mut sets_iter = sets.iter();

    // set up regexes to parse the counts for each color cube
    let re_red = Regex::new(r"(\d+) red").unwrap();
    let re_green =Regex::new(r"(\d+) green").unwrap();
    let re_blue =Regex::new(r"(\d+) blue").unwrap();

    while let Some(set) = sets_iter.next() {
        let mut red_count: u32 = 0;
        let mut green_count: u32 = 0;
        let mut blue_count: u32 = 0;
        if let Some(mat_red) = re_red.captures(set) {
            red_count = mat_red.get(1).unwrap().as_str().parse().unwrap();
        }
        if let Some(mat_green) = re_green.captures(set) {
            green_count = mat_green.get(1).unwrap().as_str().parse().unwrap();
        }
        if let Some(mat_blue) = re_blue.captures(set) {
            blue_count = mat_blue.get(1).unwrap().as_str().parse().unwrap();
        }
        // Maximum possible counts: only 12 red cubes, 13 green cubes, and 14 blue cubes.
        if red_count > 12 || green_count > 13 || blue_count > 14 {
            return 0;
        }
    }


    // Parse game ID
    let re = Regex::new(r"Game (\d+)").unwrap();
    let mat = re.captures(game_str).unwrap();
    let game_id: u32 = mat.get(1).unwrap().as_str().parse().unwrap();
    return game_id;
}

fn part1(input: &str) -> u32 {
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
