use std::fs;

fn main() {
    let filepath = "data.txt";
    let contents = fs::read_to_string(filepath).unwrap();
    let lines: Vec<&str> = contents.split("\n").collect();

    let mut score = 0;

    // 0 if lost, 3 if draw, 6 if win
    // 1 for Rock, 2 for Paper, 3 for Scissors

    for line in lines.iter() {
        // println!("{:?} ", line);
        if line.is_empty() {
            break;
        }
        // split the line in two
        // match case the first part
        let round: Vec<&str> = line.split(" ").collect();
        match round[0] {
            "A" => match round[1] {
                "X" => {
                    score += 3;
                }
                "Y" => {
                    score += 4;
                }
                "Z" => {
                    score += 8;
                }
                _ => {
                    println!("Invalid input");
                }
            },
            "B" => match round[1] {
                "X" => {
                    score += 1;
                }
                "Y" => {
                    score += 5;
                }
                "Z" => {
                    score += 9;
                }
                _ => {
                    println!("Invalid input");
                }
            },
            "C" => match round[1] {
                "X" => {
                    score += 2;
                }
                "Y" => {
                    score += 6;
                }
                "Z" => {
                    score += 7;
                }
                _ => {
                    println!("Invalid input");
                }
            },
            _ => {
                println!("Invalid input");
            }
        }
    }
    println!("Score: {}", score);
}

// enum RPS {
//     Rock,
//     Paper,
//     Scissors,
// }
