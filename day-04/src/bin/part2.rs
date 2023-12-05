use std::collections::{HashMap, HashSet};


fn main() {
    let input = include_str!("./input4.txt");
    let output = part2(input);
    println!("{}", output);
}


fn add_elements_recursively(card_stack: &mut Vec<u32>, cards: &HashMap<u32, usize>, i: u32) {
    // card_stack: vector of all the winning cards and their copies
    // cards: all of the cards plus their matching numbers
    // i: the card currently under evaluation

    // Add the current element to the vector
    card_stack.push(i);

    if (i as usize) < cards.len() {

        // Recursively call the function with a incremented value
        let n_matches = *cards.get(&i).unwrap() as u32;
        let next_card = i+1;
        let last_card = i+n_matches;
        for k in next_card..(last_card+1) {
            add_elements_recursive(card_stack, &cards, k);
        }
    }
}

fn part2(input: &str) -> usize {
    let extracted_cards: Vec<Vec<&str>> = input
        .lines()
        .map(|cardstr| {
            cardstr.split(": ")
            .collect::<Vec<&str>>()
        })
        .map(|cardvec| {
            let cardnumbers: Vec<&str> = cardvec[1].split(" | ").collect();
            [cardvec[0], cardnumbers[0], cardnumbers[1]].to_vec()
        })
        .collect();


    let mut cards: HashMap<u32, usize> = HashMap::new();
    for cardvec in extracted_cards {
        let card_number: u32 = cardvec[0].split_whitespace().nth(1).unwrap().parse().unwrap();
        let winning_numbers: Vec<&str> = cardvec[1].split_whitespace().collect();
        let my_numbers: Vec<&str> = cardvec[2].split_whitespace().collect();


        let winning_numbers_set: HashSet<_> = HashSet::from_iter(winning_numbers.iter().cloned());
        let my_numbers_set: HashSet<_> = HashSet::from_iter(my_numbers.iter().cloned());
        let my_winning_numbers = &winning_numbers_set & &my_numbers_set;
        cards.insert(card_number, my_winning_numbers.len() );

    }

    let mut card_stack: Vec<u32> = Vec::new();
    for i in  1..(cards.len()+1) {
        add_elements_recursively(&mut card_stack, &cards, i as u32);
    }

    return card_stack.len();
}
