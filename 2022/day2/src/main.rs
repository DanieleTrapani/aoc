use std::fs;

fn main() {
    let filepath = "data.txt";
    let contents = fs::read_to_string(filepath).unwrap();
    let lines: Vec<&str> = contents.split("\n").collect();

    let mut score = 0;

    // 0 if lost, 3 if draw, 6 if win
    // 1 for Rock, 2 for Paper, 3 for Scissors

    for line in lines.iter() {
        if line.is_empty() {
            break;
        }
        let round: Vec<&str> = line.split(" ").collect();
        score += calculate_score(round[0], round[1]);
    }
    println!("Score: {}", score);
}

fn calculate_score(first: &str, second: &str) -> i32 {
    match first {
        "A" => match second {
            "X" => 3,
            "Y" => 4,
            "Z" => 8,
            _ => 0,
        },
        "B" => match second {
            "X" => 1,
            "Y" => 5,
            "Z" => 9,
            _ => 0,
        },
        "C" => match second {
            "X" => 2,
            "Y" => 6,
            "Z" => 7,
            _ => 0,
        },
        _ => 0,
    }
}
