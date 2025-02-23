use std::collections::HashSet;

fn main() {
    let output = std::io::stdin()
        .lines()
        .map(|line_result| line_result.expect("Stdin error"))
        .filter(|line| supports_ssl(line))
        .count();
    println!("{}", output);
}

fn supports_ssl(line: &str) -> bool {
    let open_brackets_indicies: Vec<usize> = line
        .chars()
        .enumerate()
        .filter(|(_idx, ch)| *ch == '[')
        .map(|(idx, _ch)| idx)
        .collect();
    let close_brackets_indicies: Vec<usize> = line
        .chars()
        .enumerate()
        .filter(|(_idx, ch)| *ch == ']')
        .map(|(idx, _ch)| idx)
        .collect();
    assert_eq!(open_brackets_indicies.len(), close_brackets_indicies.len());
    let brackets_indicies = open_brackets_indicies
        .iter()
        .copied()
        .zip(close_brackets_indicies.clone())
        .flat_map(|(i0, i1)| [i0, i1])
        .collect::<Vec<usize>>();
    assert!(brackets_indicies.is_sorted());

    let hypernet_sequences: Vec<&str> = brackets_indicies
        .windows(2)
        .step_by(2)
        .map(|indicies| (indicies[0], indicies[1]))
        .map(|(left, right)| &line[left + 1..right])
        .collect();

    let supernet_sequences_boundary_indicies = {
        let mut a = brackets_indicies.clone();
        a.iter_mut()
            .enumerate()
            .filter(|(i, _)| i % 2 == 1)
            .map(|(_, closed_bracket_index)| closed_bracket_index)
            .for_each(|closed_bracket_index| *closed_bracket_index += 1);
        a.insert(0, 0);
        a.push(line.len());
        a
    };
    let supernet_sequences: Vec<&str> = supernet_sequences_boundary_indicies
        .windows(2)
        .step_by(2)
        .map(|indicies| (indicies[0], indicies[1]))
        .map(|(left, right)| &line[left..right])
        .collect();

    let valid_b_and_a_pairs_conforming_to_bab_pattern: HashSet<(char, char)> = hypernet_sequences
        .into_iter()
        .flat_map(|hypernet_sequence| {
            hypernet_sequence
                .chars()
                .collect::<Vec<char>>()
                .windows(3)
                .map(|window| window.to_vec())
                .collect::<Vec<Vec<char>>>()
        })
        .filter(|window| window[0] != window[1] && window[0] == window[2])
        .map(|window| (window[0], window[1]))
        .collect();
    supernet_sequences
        .into_iter()
        .flat_map(|supernet_sequence| {
            supernet_sequence
                .chars()
                .collect::<Vec<char>>()
                .windows(3)
                .map(|window| window.to_vec())
                .collect::<Vec<Vec<char>>>()
        })
        .filter(|window| window[0] != window[1] && window[0] == window[2])
        .map(|window| (window[0], window[1]))
        .any(|(a, b)| valid_b_and_a_pairs_conforming_to_bab_pattern.contains(&(b, a)))
}
