use std::fs;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Elf {
    calories: i32,
}

fn main() {
    let filepath = "data.txt";
    let contents = fs::read_to_string(filepath).unwrap();
    let lines: Vec<&str> = contents.split("\n").collect();
    // iterate over the lines, if you encounter empty line do the comparison
    // increase the counter

    // index of the result
    // let mut result = 0;
    // calories count of the max
    // let mut max = 0;
    // index count
    // let mut counter = 0;
    let mut sum = 0;
    // vector of elfs
    let mut elves: Vec<Elf> = Vec::new();

    for i in 0..lines.len() {
        if lines[i] == "" {
            let new_elf = Elf { calories: sum };
            elves.push(new_elf);
            sum = 0;
            // if sum > max {
            //     max = sum;
            //     result = counter;
            //     sum = 0;
            //     counter += 1;
            //     continue;
            // } else {
            //     sum = 0;
            //     counter += 1;
            //     continue;
            // }
        }
        let line_as_int: i32 = lines[i].parse().unwrap();
        sum += line_as_int;
    }
    elves.sort_by(|a, b| b.calories.cmp(&a.calories));
    let mut result = 0;

    for i in 0..3 {
        result += elves[i].calories;
    }

    println!("Result: The top 3 elves are carrying {} calories", result);
}
