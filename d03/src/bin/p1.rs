fn main() {
    let output = std::io::stdin()
        .lines()
        .map(|line_result| line_result.expect("Stdin error"))
        .map(|line| {
            line.split_whitespace()
                .map(|split| split.parse::<u64>().expect("Invalid triangle side length"))
                .collect::<Vec<_>>()
        })
        .inspect(|triangle| {
            assert_eq!(triangle.len(), 3);
        })
        .map(|triangle| (triangle[0], triangle[1], triangle[2]))
        .filter(|&(t0, t1, t2)| t0 < t1 + t2 && t1 < t0 + t2 && t2 < t0 + t1)
        .count();
    println!("{}", output);
}
