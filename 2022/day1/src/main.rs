use std::{fs, num::ParseIntError};

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Elf {
    calories: i32,
}

fn main() {
    let filepath = "data.txt";
    let contents = fs::read_to_string(filepath).unwrap();
    let lines: Vec<&str> = contents.split("\n").collect();

    let mut sum = 0;
    // vector of elfs
    let mut elves: Vec<Elf> = Vec::new();

    for i in 0..lines.len() {
        if lines[i] == "" {
            let new_elf = Elf { calories: sum };
            // print!("{:?} ", &new_elf);
            elves.push(new_elf);
            sum = 0;
        } else {
            let parse_line: Result<i32, ParseIntError> = lines[i].trim_end().parse();
            match parse_line {
                Ok(line_as_int) => sum += line_as_int,
                Err(err) => print!("Error: {}", err),
            }
        }
    }
    elves.sort_by(|a, b| b.calories.cmp(&a.calories));
    let mut result = 0;

    for i in 0..3 {
        result += elves[i].calories;
    }

    println!("Result: The top 3 elves are carrying {} calories", result);
}
