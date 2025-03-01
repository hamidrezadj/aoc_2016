#[derive(Default, Clone, Copy)]
enum MarkerState {
    #[default]
    Outside,
    InsideLeft,
    InsideRight,
}

#[derive(Default)]
struct State {
    incomplete_marker: MarkerState,
    incomplete_marker_left: Vec<char>,
    incomplete_marker_right: Vec<char>,
    complete_marker_skips: usize,
    decomperessed_message_length: usize,
}

fn main() {
    let output = std::io::stdin()
        .lines()
        .map(|line_result| line_result.expect("Stdin error"))
        .flat_map(|line| line.chars().collect::<Vec<char>>())
        .fold(State::default(), |mut state, character| {
            if state.complete_marker_skips > 0 {
                state.complete_marker_skips -= 1;
                return state;
            }
            match (character, state.incomplete_marker) {
                ('(', _) => {
                    state.decomperessed_message_length += state.incomplete_marker_len();
                    state.reset();
                    state.incomplete_marker = MarkerState::InsideLeft;
                }
                (ch, MarkerState::InsideLeft) if ch.is_numeric() => {
                    state.incomplete_marker_left.push(ch)
                }
                ('x', MarkerState::InsideLeft) => {
                    state.incomplete_marker = MarkerState::InsideRight;
                }
                (ch, MarkerState::InsideRight) if ch.is_numeric() => {
                    state.incomplete_marker_right.push(ch)
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
                    state.decomperessed_message_length += marker_left * marker_right;
                    state.complete_marker_skips = marker_left;
                    state.reset();
                }
                (_, MarkerState::Outside) => state.decomperessed_message_length += 1,
                _ => {
                    state.decomperessed_message_length += state.incomplete_marker_len();
                    state.decomperessed_message_length += 1;
                    state.reset();
                }
            }
            state
        })
        .decomperessed_message_length;
    println!("{}", output);
}

impl State {
    fn incomplete_marker_len(&self) -> usize {
        match self.incomplete_marker {
            MarkerState::Outside => 0,
            // +1 for open parentheses
            MarkerState::InsideLeft => self.incomplete_marker_left.len() + 1,
            // +2 for open parentheses and x
            MarkerState::InsideRight => {
                self.incomplete_marker_left.len() + self.incomplete_marker_right.len() + 2
            }
        }
    }
    fn reset(&mut self) {
        self.incomplete_marker = Default::default();
        self.incomplete_marker_left = Default::default();
        self.incomplete_marker_right = Default::default();
    }
}
