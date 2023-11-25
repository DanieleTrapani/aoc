use std::fs;

fn main() {
    let filepath = "data.txt";
    let contents = fs::read_to_string(filepath).unwrap();
    let lines: Vec<&str> = contents.split("\n").collect();
    // iterate over the lines, if you encounter empty line do the comparison
    // increase the counter

    // index of the result
    let mut result = 0;
    // calories count of the max
    let mut max = 0;
    // index count
    let mut counter = 0;
    let mut sum = 0;
    for i in 0..lines.len() {
        if lines[i] == "" {
            if sum > max {
                max = sum;
                result = counter;
                sum = 0;
                counter += 1;
                continue;
            } else {
                sum = 0;
                counter += 1;
                continue;
            }
        }
        let line_as_int: i32 = lines[i].parse().unwrap();
        sum += line_as_int;
    }
    println!(
        "Result: Elf n.{}, they are carrying {} calories",
        result, max
    );
}
