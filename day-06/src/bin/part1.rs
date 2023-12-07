fn main() {
    let input = include_str!("./input6.txt");
    let output = part1(input);
    println!("{}", output);
}

fn part1(input: &str) -> u32 {

    let races: Vec<&str> = input
                    .lines()
                    .collect();

    // parse the two lines
    let times: Vec<u32> = races[0].split_whitespace().skip(1).map(|v| v.parse().unwrap()).collect();
    let winning_distances: Vec<u32> = races[1].split_whitespace().skip(1).map(|v| v.parse().unwrap()).collect();

    let mut num_wins: Vec<usize> = Vec::new();
    for i in 0..4 {
        let time_sequence: Vec<_> = (0..=times[i]).collect();
        let distances: Vec<_> = time_sequence.iter()
                                     .map(|t| (times[i]-t)*t)
                                     .collect();
        // filter distances
        let better_distances: Vec<_> = distances.iter().filter(|d| d>&&winning_distances[i]).collect();
        num_wins.push(better_distances.len());
    }

    let margin: usize =  num_wins.iter().product();
    // println!("{:?}",margin);

    return margin as u32;
    
}


