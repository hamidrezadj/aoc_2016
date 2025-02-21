use std::{collections::HashMap, ops::AddAssign};

fn main() {
    let message_length = std::env::args()
        .nth(1)
        .map(|arg| {
            arg.parse()
                .expect("Bad message length argument: Needs to be an unsigned integer")
        })
        .unwrap_or(8);
    let output: String = std::io::stdin()
        .lines()
        .map(|line_result| line_result.expect("Stdin error"))
        .inspect(|line| assert_eq!(line.len(), message_length, "Wrong message length"))
        .fold(
            vec![HashMap::new(); message_length],
            |mut accumulator: Vec<HashMap<char, u64>>, line| {
                line.chars().enumerate().for_each(|(index, character)| {
                    accumulator[index]
                        .entry(character)
                        .or_default()
                        .add_assign(1);
                });
                accumulator
            },
        )
        .into_iter()
        .map(|character_to_count_map_of_an_index| {
            character_to_count_map_of_an_index
                .into_iter()
                .max_by_key(|(_character, count)| *count)
                .map(|(character, _count)| character)
                .unwrap()
        })
        .collect();
    println!("{}", output);
}
