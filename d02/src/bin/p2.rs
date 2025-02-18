use std::fmt::Write;

fn main() {
    let output = std::io::stdin()
        .lines()
        .map(|line_result| line_result.expect("Stdin error"))
        .scan(5, |tile, directions| {
            *tile = directions
                .chars()
                .fold(*tile, |tile, direction| match (direction, tile) {
                    ('R', 1) => 1,
                    ('R', 2) => 3,
                    ('R', 3) => 4,
                    ('R', 4) => 4,
                    ('R', 5) => 6,
                    ('R', 6) => 7,
                    ('R', 7) => 8,
                    ('R', 8) => 9,
                    ('R', 9) => 9,
                    ('R', 10) => 11,
                    ('R', 11) => 12,
                    ('R', 12) => 12,
                    ('R', 13) => 13,
                    ('D', 1) => 3,
                    ('D', 2) => 6,
                    ('D', 3) => 7,
                    ('D', 4) => 8,
                    ('D', 5) => 5,
                    ('D', 6) => 10,
                    ('D', 7) => 11,
                    ('D', 8) => 12,
                    ('D', 9) => 9,
                    ('D', 10) => 10,
                    ('D', 11) => 13,
                    ('D', 12) => 12,
                    ('D', 13) => 13,
                    ('L', 1) => 1,
                    ('L', 2) => 2,
                    ('L', 3) => 2,
                    ('L', 4) => 3,
                    ('L', 5) => 5,
                    ('L', 6) => 5,
                    ('L', 7) => 6,
                    ('L', 8) => 7,
                    ('L', 9) => 8,
                    ('L', 10) => 10,
                    ('L', 11) => 10,
                    ('L', 12) => 11,
                    ('L', 13) => 13,
                    ('U', 1) => 1,
                    ('U', 2) => 2,
                    ('U', 3) => 1,
                    ('U', 4) => 4,
                    ('U', 5) => 5,
                    ('U', 6) => 2,
                    ('U', 7) => 3,
                    ('U', 8) => 4,
                    ('U', 9) => 9,
                    ('U', 10) => 6,
                    ('U', 11) => 7,
                    ('U', 12) => 8,
                    ('U', 13) => 11,
                    _ => unreachable!(),
                });
            Some(*tile)
        })
        .fold(String::new(), |mut output, tile| {
            let _ = write!(output, "{:X}", tile);
            output
        });
    println!("{}", output);
}
