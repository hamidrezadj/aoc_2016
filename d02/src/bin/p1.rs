fn main() {
    let output = std::io::stdin()
        .lines()
        .map(|line_result| line_result.expect("Stdin error"))
        .scan(5, |tile, directions| {
            *tile = directions
                .chars()
                .fold(*tile, |tile, direction| match (direction, tile) {
                    ('R', 1) => 2,
                    ('R', 2) => 3,
                    ('R', 3) => 3,
                    ('R', 4) => 5,
                    ('R', 5) => 6,
                    ('R', 6) => 6,
                    ('R', 7) => 8,
                    ('R', 8) => 9,
                    ('R', 9) => 9,
                    ('D', 1) => 4,
                    ('D', 2) => 5,
                    ('D', 3) => 6,
                    ('D', 4) => 7,
                    ('D', 5) => 8,
                    ('D', 6) => 9,
                    ('D', 7) => 7,
                    ('D', 8) => 8,
                    ('D', 9) => 9,
                    ('L', 1) => 1,
                    ('L', 2) => 1,
                    ('L', 3) => 2,
                    ('L', 4) => 4,
                    ('L', 5) => 4,
                    ('L', 6) => 5,
                    ('L', 7) => 7,
                    ('L', 8) => 7,
                    ('L', 9) => 8,
                    ('U', 1) => 1,
                    ('U', 2) => 2,
                    ('U', 3) => 3,
                    ('U', 4) => 1,
                    ('U', 5) => 2,
                    ('U', 6) => 3,
                    ('U', 7) => 4,
                    ('U', 8) => 5,
                    ('U', 9) => 6,
                    _ => unreachable!(),
                });
            Some(*tile)
        })
        .map(|tile| tile.to_string())
        .collect::<String>();
    println!("{}", output);
}
