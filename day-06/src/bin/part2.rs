fn main() {
    let input = include_str!("./input6.txt");
    let output = part2(input);
    println!("{}", output);
}

fn part2(input: &str) -> f64 {


    let races: Vec<&str> = input
                    .lines()
                    .collect();

    // parse the two lines
    let time_str: String = races[0].replace("Time:", "").chars().filter(|&c| !c.is_whitespace()).collect();
    let time: f64 = time_str.parse().unwrap();
    
    let winning_distance_str: String = races[1].replace("Distance:", "").chars().filter(|&c| !c.is_whitespace()).collect();
    let winning_distance: f64 = winning_distance_str.parse().unwrap();

    // lower and upper bound on the winning distance are from the roots of the quadratic formula
    let upper_bound = ((time.powi(2)- (4f64*winning_distance)).sqrt() + time)/2.0;
    let lower_bound = (time - (time.powi(2) - (4f64*winning_distance) ).sqrt())/2.0;
    
    return upper_bound.floor() - lower_bound.ceil() + 1.0
}


