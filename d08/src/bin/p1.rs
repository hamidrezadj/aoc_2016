use std::collections::VecDeque;

enum Instruction {
    Rectangle(usize, usize),
    RotateRow(usize, usize),
    RotateColumn(usize, usize),
}
struct Screen {
    state: Vec<VecDeque<bool>>,
    x_len: usize,
    y_len: usize,
}

fn main() {
    let x_len: usize = std::env::args()
        .nth(1)
        .map(|x| x.parse().expect("Bad first argument"))
        .unwrap_or(50);
    let y_len: usize = std::env::args()
        .nth(2)
        .map(|y| y.parse().expect("Bad second argument"))
        .unwrap_or(6);
    let output = std::io::stdin()
        .lines()
        .map(|line_result| line_result.expect("Stdin error"))
        .map(parse_instruction)
        .fold(Screen::new(x_len, y_len), |screen, instruction| {
            apply_instruction(screen, instruction)
        })
        .state
        .into_iter()
        .flatten()
        .filter(|pixel| *pixel)
        .count();
    println!("{}", output);
}

impl Screen {
    fn new(x_len: usize, y_len: usize) -> Screen {
        let state = vec![VecDeque::from(vec![false; x_len]); y_len];
        Screen {
            state,
            x_len,
            y_len,
        }
    }
}

fn apply_instruction(mut screen: Screen, instruction: Instruction) -> Screen {
    match instruction {
        Instruction::Rectangle(a, b) => screen
            .state
            .iter_mut()
            .take(b)
            .for_each(|row| row.iter_mut().take(a).for_each(|pixel| *pixel = true)),
        Instruction::RotateRow(row_number, count) => screen
            .state
            .get_mut(row_number)
            .into_iter()
            .for_each(|row| row.rotate_right(count % screen.x_len)),
        Instruction::RotateColumn(column_number, count) => {
            let mut new_column: VecDeque<bool> = screen
                .state
                .iter()
                .flat_map(|row| row.get(column_number))
                .copied()
                .collect();
            new_column.rotate_right(count % screen.y_len);
            screen
                .state
                .iter_mut()
                .flat_map(|row| row.get_mut(column_number))
                .zip(new_column)
                .for_each(|(pixel, new_pixel)| *pixel = new_pixel);
        }
    }
    screen
}

fn parse_instruction(line: String) -> Instruction {
    let splits: Vec<&str> = line.split_whitespace().collect();
    match (splits[0], splits[1]) {
        ("rect", axb) => {
            let (a, b) = axb.split_once('x').expect("No x found in AxB expression");
            let a = a
                .parse()
                .expect("A was not an unsigned integer that fits in usize");
            let b = b
                .parse()
                .expect("B was not an unsigned integer that fits in usize");
            Instruction::Rectangle(a, b)
        }
        ("rotate", "row") => {
            let (_, row) = splits[2]
                .split_once('=')
                .expect("No equal sign in row expression");
            let row = row
                .parse()
                .expect("Row was not an unsigned integer that fits in usize");
            let count = splits[4]
                .parse()
                .expect("Count was not an unsigned integer that fits in usize");
            Instruction::RotateRow(row, count)
        }
        ("rotate", "column") => {
            let (_, column) = splits[2]
                .split_once('=')
                .expect("No equal sign in column expression");
            let column = column
                .parse()
                .expect("Column was not an unsigned integer that fits in usize");
            let count = splits[4]
                .parse()
                .expect("Count was not an unsigned integer that fits in usize");
            Instruction::RotateColumn(column, count)
        }
        _ => panic!("Invalid instruction"),
    }
}
