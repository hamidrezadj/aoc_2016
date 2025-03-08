use std::collections::HashMap;

type OutputId = u64;
type BotId = u64;
type Value = u64;

#[derive(Clone, Copy)]
enum Chip {
    Value(Value),
    Low(BotId),
    High(BotId),
}

#[derive(Default)]
struct Factory {
    bots: HashMap<BotId, Vec<Chip>>,
    outputs: HashMap<OutputId, Chip>,
}

#[derive(Default)]
struct ResolvedFactory {
    bots: HashMap<BotId, (Value, Value)>,
    _outputs: HashMap<OutputId, Value>,
}

fn main() {
    let factory: Factory = std::io::stdin()
        .lines()
        .map(|line_result| line_result.expect("Stdin error"))
        .fold(Factory::default(), |mut factory, line| {
            let splits: Vec<&str> = line.split_whitespace().collect();
            match splits.first().copied().expect("Empty line") {
                "value" => {
                    let value = splits
                        .get(1)
                        .expect("Invalid pattern")
                        .parse::<Value>()
                        .expect("Invalid pattern");
                    match splits.get(4).copied().expect("Invalid Pattern") {
                        "bot" => {
                            let bot = splits
                                .get(5)
                                .expect("Invalid pattern")
                                .parse::<BotId>()
                                .expect("Invalid pattern");
                            let bot_chips = factory.bots.entry(bot).or_default();
                            assert!(bot_chips.len() < 2);
                            bot_chips.push(Chip::Value(value));
                        }
                        "output" => {
                            let output = splits
                                .get(5)
                                .expect("Invalid pattern")
                                .parse::<OutputId>()
                                .expect("Invalid pattern");
                            let prev_output_chip =
                                factory.outputs.insert(output, Chip::Value(value));
                            assert!(prev_output_chip.is_none());
                        }
                        _ => panic!("Invalid pattern"),
                    }
                }
                "bot" => {
                    let transceiver_bot_id = splits
                        .get(1)
                        .expect("Invalid pattern")
                        .parse::<BotId>()
                        .expect("Invalid pattern");
                    assert_eq!(splits.get(3).copied().expect("Invalid pattern"), "low");
                    assert_eq!(splits.get(8).copied().expect("Invalid pattern"), "high");
                    let (low_receiver_type, low_receiver_id) = (
                        splits.get(5).copied().expect("Invalid pattern"),
                        splits.get(6).copied().expect("Invalid pattern"),
                    );
                    let (high_receiver_type, high_receiver_id) = (
                        splits.get(10).copied().expect("Invalid pattern"),
                        splits.get(11).copied().expect("Invalid pattern"),
                    );
                    match low_receiver_type {
                        "bot" => {
                            let receiver_bot_id =
                                low_receiver_id.parse::<BotId>().expect("Invalid pattern");
                            let bot_chips = factory.bots.entry(receiver_bot_id).or_default();
                            assert!(bot_chips.len() < 2);
                            bot_chips.push(Chip::Low(transceiver_bot_id));
                        }
                        "output" => {
                            let output_id = low_receiver_id
                                .parse::<OutputId>()
                                .expect("Invalid pattern");
                            let previously_inserted_output_chip = factory
                                .outputs
                                .insert(output_id, Chip::Low(transceiver_bot_id));
                            assert!(previously_inserted_output_chip.is_none());
                        }
                        _ => panic!("Invalid pattern"),
                    }
                    match high_receiver_type {
                        "bot" => {
                            let receiver_bot_id =
                                high_receiver_id.parse::<BotId>().expect("Invalid pattern");
                            let bot_chips = factory.bots.entry(receiver_bot_id).or_default();
                            assert!(bot_chips.len() < 2);
                            bot_chips.push(Chip::High(transceiver_bot_id));
                        }
                        "output" => {
                            let output_id = high_receiver_id
                                .parse::<OutputId>()
                                .expect("Invalid pattern");
                            let previously_inserted_output_chip = factory
                                .outputs
                                .insert(output_id, Chip::High(transceiver_bot_id));
                            assert!(previously_inserted_output_chip.is_none());
                        }
                        _ => panic!("Invalid pattern"),
                    }
                }
                _ => panic!("Invalid pattern"),
            }
            factory
        });
    let mut resolved_factory: ResolvedFactory = Default::default();
    let mut resolution_queue: Vec<BotId> = factory.bots.keys().copied().collect();
    while let Some(bot_id) = resolution_queue.pop() {
        if resolved_factory.bots.contains_key(&bot_id) {
            continue;
        }
        let chips = factory.bots.get(&bot_id).unwrap();
        let value_0 = match chips[0] {
            Chip::Value(v0) => Some(v0),
            Chip::Low(dependency_bot_id) => resolved_factory
                .bots
                .get(&dependency_bot_id)
                .copied()
                .map(|(low, _high)| low),
            Chip::High(dependency_bot_id) => resolved_factory
                .bots
                .get(&dependency_bot_id)
                .copied()
                .map(|(_low, high)| high),
        };
        let value_1 = match chips[1] {
            Chip::Value(v1) => Some(v1),
            Chip::Low(dependency_bot_id) => resolved_factory
                .bots
                .get(&dependency_bot_id)
                .copied()
                .map(|(low, _high)| low),
            Chip::High(dependency_bot_id) => resolved_factory
                .bots
                .get(&dependency_bot_id)
                .copied()
                .map(|(_low, high)| high),
        };
        if let (Some(v0), Some(v1)) = (value_0, value_1) {
            resolved_factory
                .bots
                .insert(bot_id, (v0.min(v1), v0.max(v1)));
            continue;
        }
        let dependency_bot_id_0 = match chips[0] {
            Chip::Low(id) => Some(id),
            Chip::High(id) => Some(id),
            Chip::Value(_) => None,
        };
        let dependency_bot_id_1 = match chips[1] {
            Chip::Low(id) => Some(id),
            Chip::High(id) => Some(id),
            Chip::Value(_) => None,
        };
        resolution_queue.append(
            &mut [Some(bot_id), dependency_bot_id_0, dependency_bot_id_1]
                .into_iter()
                .flatten()
                .collect(),
        );
    }
    let output: Value = (0..=2)
        .map(|output_id| match factory.outputs.get(&output_id).unwrap() {
            Chip::Value(v) => *v,
            Chip::Low(bot_id) => resolved_factory
                .bots
                .get(bot_id)
                .copied()
                .map(|(low, _high)| low)
                .unwrap(),
            Chip::High(bot_id) => resolved_factory
                .bots
                .get(bot_id)
                .copied()
                .map(|(_low, high)| high)
                .unwrap(),
        })
        .product();
    println!("{}", output);
}
