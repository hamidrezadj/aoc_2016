use std::{collections::HashSet, ops::ControlFlow};

enum RelativeDirection {
    Right,
    Left,
}
#[derive(Clone, Copy)]
enum Direction {
    East,
    South,
    West,
    North,
}
type Distance = u64;
type Position = (i64, i64);

fn main() {
    let final_position = std::io::stdin()
        .lines()
        .map(|line_result| line_result.expect("Stdin error"))
        .flat_map(|line| line.split(", ").map(|s| s.to_string()).collect::<Vec<_>>())
        .map(|split| {
            let relative_direction = match &split[0..1] {
                "R" => RelativeDirection::Right,
                "L" => RelativeDirection::Left,
                _ => panic!("Invalid pattern"),
            };
            let distance = split[1..]
                .to_string()
                .parse::<Distance>()
                .expect("Invalid pattern");
            (relative_direction, distance)
        })
        .scan(
            Direction::North,
            |direction, (relative_direction, distance)| {
                *direction = match (*direction, relative_direction) {
                    (Direction::East, RelativeDirection::Right) => Direction::South,
                    (Direction::East, RelativeDirection::Left) => Direction::North,
                    (Direction::South, RelativeDirection::Right) => Direction::West,
                    (Direction::South, RelativeDirection::Left) => Direction::East,
                    (Direction::West, RelativeDirection::Right) => Direction::North,
                    (Direction::West, RelativeDirection::Left) => Direction::South,
                    (Direction::North, RelativeDirection::Right) => Direction::East,
                    (Direction::North, RelativeDirection::Left) => Direction::West,
                };
                Some((*direction, distance))
            },
        )
        .scan((0, 0), |(i, j): &mut Position, (direction, distance)| {
            let (prev_i, prev_j) = (*i, *j);
            let next_positions = match direction {
                Direction::East => {
                    *j = j.checked_add_unsigned(distance).expect("Overflow");
                    (prev_j + 1..=*j).map(|j| (*i, j)).collect::<Vec<_>>()
                }
                Direction::South => {
                    *i = i.checked_add_unsigned(distance).expect("Overflow");
                    (prev_i + 1..=*i).map(|i| (i, *j)).collect::<Vec<_>>()
                }
                Direction::West => {
                    *j = j.checked_sub_unsigned(distance).expect("Overflow");
                    (*j..prev_j).map(|j| (*i, j)).collect::<Vec<_>>()
                }
                Direction::North => {
                    *i = i.checked_sub_unsigned(distance).expect("Overflow");
                    (*i..prev_i).map(|i| (i, *j)).collect::<Vec<_>>()
                }
            };
            Some(next_positions)
        })
        .flatten()
        .try_fold(HashSet::from([(0, 0)]), |mut visited, (i, j)| {
            if visited.insert((i, j)) {
                ControlFlow::Continue(visited)
            } else {
                ControlFlow::Break((i, j))
            }
        })
        .break_value()
        .expect("Could not find a position that was visited twice");
    let (i, j) = final_position;
    let output = i.abs() + j.abs();
    println!("{}", output);
}
