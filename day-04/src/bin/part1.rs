use std::collections::HashSet;


fn main() {
    let input = include_str!("./input4.txt");
    let output = part1(input);
    println!("{}", output);
}


fn part1(input: &str) -> u32 {
    let extracted_cards: Vec<HashSet<_>> = input
        .lines()
        .map(|cardstr| {
            cardstr.split(": ")
            .collect::<Vec<&str>>()
        })
        .map(|cardvec| {
            let cardnumbers: Vec<&str> = cardvec[1].split(" | ").collect();
            [cardvec[0], cardnumbers[0], cardnumbers[1]].to_vec()
        })
        .map(|cardvec| {
            let winning_numbers: Vec<&str> = cardvec[1].split_whitespace().collect();
            let my_numbers: Vec<&str> = cardvec[2].split_whitespace().collect();
            [winning_numbers, my_numbers].to_vec()
        })
        .map(| numbersvec | {
            let winning_numbers = HashSet::from_iter(numbersvec[0].iter().cloned());
            let my_numbers = HashSet::from_iter(numbersvec[1].iter().cloned());
            &winning_numbers & &my_numbers
        })
        .collect();


    // Get the HashSet intersection sizes and then compute winnings
    let match_points: Vec<u32> = extracted_cards.iter()
        .map(|set| set.len())
        .filter(|&i| i>0)
        .map(|i| 2u32.pow(i as u32 - 1u32))
        .collect();

    return match_points.iter().sum();
}
