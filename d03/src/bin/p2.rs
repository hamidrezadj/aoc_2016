use std::ops::Not;

fn main() {
    let input: Vec<Vec<u64>> = std::io::stdin()
        .lines()
        .map(|line_result| line_result.expect("Stdin error"))
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<u64>().expect("Invalid side length"))
                .collect()
        })
        .collect();
    assert!(input.is_empty().not());
    assert!(input[0].is_empty().not());
    assert!(input.iter().all(|row| row.len() == input[0].len()));
    let i_len = input.len();
    let j_len = input[0].len();
    let scan_positions: Vec<(usize, usize)> = (0..j_len)
        .flat_map(|j| (0..i_len).map(move |i| (i, j)))
        .collect();
    let output = scan_positions
        .windows(3)
        .step_by(3)
        .map(|triangle_positions| {
            let triangle: Vec<u64> = triangle_positions
                .iter()
                .map(|&(i, j)| input[i][j])
                .collect();
            assert_eq!(triangle.len(), 3);
            (triangle[0], triangle[1], triangle[2])
        })
        .filter(|&(t0, t1, t2)| t0 < t1 + t2 && t1 < t0 + t2 && t2 < t0 + t1)
        .count();
    println!("{}", output);
}
