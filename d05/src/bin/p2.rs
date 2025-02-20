use std::ops::ControlFlow;

fn main() {
    let room_id = std::io::stdin()
        .lines()
        .map(|line_result| line_result.expect("Stdin error"))
        .next()
        .expect("Empty input");
    let output: String = (0..)
        .map(|index| format!("{}{}", room_id, index).into_bytes())
        .map(md5::compute)
        .map(|digest| format!("{:x}", digest))
        .filter(|hash| hash[0..5].eq("00000"))
        .filter(|hash| {
            hash.chars()
                .nth(5)
                .filter(|i| ('0'..'8').contains(i))
                .is_some()
        })
        .map(|hash| {
            (
                hash.chars()
                    .nth(5)
                    .map(|i| (i as u8 - b'0') as usize)
                    .unwrap(),
                hash.chars().nth(6).unwrap(),
            )
        })
        .try_fold(['\0'; 8], |mut password, (idx, ch)| {
            if password[idx] == '\0' {
                password[idx] = ch;
            }
            if password.iter().all(|ch| *ch != '\0') {
                ControlFlow::Break(password)
            } else {
                ControlFlow::Continue(password)
            }
        })
        .break_value()
        .unwrap()
        .into_iter()
        .collect();
    println!("{}", output);
}
