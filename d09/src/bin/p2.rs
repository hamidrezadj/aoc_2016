use std::ops::ControlFlow;

#[derive(Default, Clone, Copy)]
enum MarkerState {
    #[default]
    Outside,
    InsideLeft,
    InsideRight,
}

#[derive(Default)]
struct AutomataState {
    incomplete_marker_state: MarkerState,
    incomplete_marker_left: Vec<char>,
    incomplete_marker_right: Vec<char>,
}

struct Marker {
    content_length: usize,
    repeat_count: usize,
    marker_index: usize,
    content_index: usize,
}

fn main() {
    let mut compressed_data: Vec<char> = std::io::stdin()
        .lines()
        .map(|line_result| line_result.expect("Stdin error"))
        .flat_map(|line| line.chars().collect::<Vec<char>>())
        .collect();
    compressed_data.reverse();
    let mut decompressed_length = 0;
    while let Some(first_marker) = compressed_data
        .iter()
        .rev()
        .enumerate()
        .try_fold(AutomataState::default(), |mut state, (index, character)| {
            match (character, state.incomplete_marker_state) {
                ('(', _) => {
                    state = Default::default();
                    state.incomplete_marker_state = MarkerState::InsideLeft;
                }
                (ch, MarkerState::InsideLeft) if ch.is_numeric() => {
                    state.incomplete_marker_left.push(*ch)
                }
                ('x', MarkerState::InsideLeft) => {
                    state.incomplete_marker_state = MarkerState::InsideRight;
                }
                (ch, MarkerState::InsideRight) if ch.is_numeric() => {
                    state.incomplete_marker_right.push(*ch)
                }
                (')', MarkerState::InsideRight) => {
                    let marker_left = state
                        .incomplete_marker_left
                        .iter()
                        .collect::<String>()
                        .parse::<usize>()
                        .unwrap();
                    let marker_right = state
                        .incomplete_marker_right
                        .iter()
                        .collect::<String>()
                        .parse::<usize>()
                        .unwrap();
                    let marker_length = state.incomplete_marker_left.len()
                        + state.incomplete_marker_right.len()
                        + 3;
                    return ControlFlow::Break(Marker {
                        content_length: marker_left,
                        repeat_count: marker_right,
                        marker_index: (index + 1) - marker_length,
                        content_index: index + 1,
                    });
                }
                (_, MarkerState::Outside) => (),
                _ => {
                    state = Default::default();
                }
            }
            ControlFlow::Continue(state)
        })
        .break_value()
    {
        decompressed_length += first_marker.marker_index;
        compressed_data.truncate(compressed_data.len() - first_marker.content_index);
        assert!(first_marker.content_length <= compressed_data.len());
        assert!(first_marker.repeat_count > 0);
        let repeated_section = &compressed_data
            [compressed_data.len() - first_marker.content_length..compressed_data.len()];
        compressed_data.append(&mut repeated_section.repeat(first_marker.repeat_count - 1));
    }
    let remaining_data = compressed_data;
    let output = decompressed_length + remaining_data.len();
    println!("{}", output);
}
