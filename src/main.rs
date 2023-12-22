use std::env;

mod utils;
mod day1;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = &args[1];
    match day.as_str() {
        "1" => {
            let input = crate::utils::read_lines("inputs/day1.txt");
            crate::day1::part1(&input);
            crate::day1::part2(&input);
        }
        _ => {
            println!("Unknown argument.")
        }
    }
}
