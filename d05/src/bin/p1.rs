fn main() {
    let room_id = std::io::stdin()
        .lines()
        .map(|line_result| line_result.expect("Stdin error"))
        .next()
        .expect("Empty input");
    let output = (0..)
        .map(|index| format!("{}{}", room_id, index).into_bytes())
        .map(md5::compute)
        .map(|digest| format!("{:x}", digest))
        .filter(|hash| hash[0..5].eq("00000"))
        .map(|hash| hash.chars().nth(5).unwrap())
        .take(8)
        .collect::<String>();
    println!("{}", output);
}
