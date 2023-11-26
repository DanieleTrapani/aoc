use std::{collections::HashMap, fs};

fn main() {
    let data = fs::read_to_string("data.txt").unwrap();
    let lines = data.lines();

    // used to convert the letter into a number
    let score_hash = generate_letter_to_number_map();

    let mut result = 0;

    for line in lines {
        // split line in half
        let (first_half, second_half) = line.split_at(line.len() / 2);
        // look for a common letter in each half
        let common_letter = first_half
            .chars()
            .find(|&letter| second_half.contains(letter))
            .unwrap();

        result += score_hash.get(&common_letter).unwrap();
    }

    // println!("{}", score_hash.get(&'B').unwrap())
    println!("Result: {}", result);
}

fn generate_letter_to_number_map() -> HashMap<char, i32> {
    let mut letter_to_number = HashMap::new();

    // Populate the HashMap with the letter-number pairs
    let mut number = 1;
    for letter in ('a'..='z').chain('A'..='Z') {
        letter_to_number.insert(letter, number);
        number += 1;
    }
    return letter_to_number;
}
