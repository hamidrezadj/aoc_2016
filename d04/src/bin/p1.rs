use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    ops::AddAssign,
};

struct Room {
    encrypted_name: String,
    sector_id: u64,
    checksum: String,
}

fn main() {
    let output: u64 = std::io::stdin()
        .lines()
        .map(|line_result| line_result.expect("Stdin error"))
        .map(parse_room)
        .filter(is_real)
        .map(|room| room.sector_id)
        .sum();
    println!("{}", output);
}

fn parse_room(line: String) -> Room {
    let (line, _) = line.split_once(']').expect("No closing bracket");
    let (line, checksum) = line.split_once('[').expect("No opening bracket");
    let checksum = checksum.to_string();
    assert_eq!(checksum.len(), 5);
    let (encrypted_name, sector_id) = line.rsplit_once('-').expect("No final hyphen");
    let encrypted_name = encrypted_name.to_string();
    assert!(encrypted_name
        .chars()
        .all(|ch| ch.is_ascii_lowercase() || ch == '-'));
    let sector_id = sector_id
        .parse()
        .expect("Sector id did not fit in an unsigned 64 bit integer");
    Room {
        encrypted_name,
        sector_id,
        checksum,
    }
}

fn is_real(room: &Room) -> bool {
    let checksum = room
        .encrypted_name
        .chars()
        .filter(|ch| *ch != '-')
        .fold(HashMap::new(), |mut accumulator: HashMap<char, u64>, ch| {
            accumulator.entry(ch).or_default().add_assign(1);
            accumulator
        })
        .into_iter()
        .map(|(ch, count)| (Reverse(count), ch))
        .collect::<BinaryHeap<(Reverse<u64>, char)>>()
        .into_sorted_vec()
        .into_iter()
        .map(|(_, ch)| ch)
        .take(5)
        .collect::<String>();
    checksum == room.checksum
}
