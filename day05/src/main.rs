mod parse;

use std::sync::Mutex;

use crate::parse::parse;
use lazy_static::lazy_static;
use midly::{
    num::{u28, u7},
    Header, Smf, Track, TrackEvent, TrackEventKind,
};

lazy_static! {
    static ref OUT: Mutex<MidiOutput> = Mutex::new(MidiOutput::new());
}

struct MidiOutput {
    smf: Smf<'static>,
    skip: u32,
}

impl MidiOutput {
    fn new() -> MidiOutput {
        let smf = Smf {
            header: Header::new(
                midly::Format::SingleTrack,
                midly::Timing::Metrical(80.into()),
            ),
            tracks: vec![Track::new()],
        };

        Self { smf, skip: 0 }
    }

    fn note(&mut self, key: u7, vel: u7, len: u28) {
        self.smf.tracks[0].push(TrackEvent {
            delta: self.skip.into(),
            kind: TrackEventKind::Midi {
                channel: 0.into(),
                message: midly::MidiMessage::NoteOn { key, vel },
            },
        });

        self.smf.tracks[0].push(TrackEvent {
            delta: len,
            kind: TrackEventKind::Midi {
                channel: 0.into(),
                message: midly::MidiMessage::NoteOff { key, vel: 0.into() },
            },
        });

        self.skip = 0;
    }

    fn chord(&mut self, keys: Vec<u7>, mut len: u28) {
        for key in &keys {
            self.smf.tracks[0].push(TrackEvent {
                delta: self.skip.into(),
                kind: TrackEventKind::Midi {
                    channel: 0.into(),
                    message: midly::MidiMessage::NoteOn {
                        key: *key,
                        vel: 100.into(),
                    },
                },
            });

            self.skip = 0;
        }

        for key in &keys {
            self.smf.tracks[0].push(TrackEvent {
                delta: len,
                kind: TrackEventKind::Midi {
                    channel: 0.into(),
                    message: midly::MidiMessage::NoteOff {
                        key: *key,
                        vel: 0.into(),
                    },
                },
            });

            len = 0.into();
        }
    }
}

fn main() {
    let filecontents = get_input();
    let (stacks, instructions) = parse(&filecontents);

    let mut stacks_a = stacks.clone();
    crane(&mut stacks_a, instructions.clone(), false);
    println!("first result: {}", top_crates(&stacks_a));

    let mut stacks_b = stacks.clone();
    crane(&mut stacks_b, instructions, true);
    println!("second result: {}", top_crates(&stacks_b));

    OUT.lock().unwrap().smf.save("out.mid").unwrap();
}

fn crane(
    stacks: &mut Vec<Vec<String>>,
    instructions: Vec<(usize, usize, usize)>,
    bonus_rules: bool,
) {
    for (amount, source, destination) in instructions {
        println!("instruction {amount} {source} {destination}");

        OUT.lock().unwrap().chord(
            vec![
                (50 + amount as u8).into(),
                (50 + source as u8).into(),
                (50 + destination as u8).into(),
            ],
            40.into(),
        );

        if bonus_rules {
            let len = stacks[source - 1].len();

            let mut cs = stacks[source - 1]
                .splice((len - amount).., [])
                .collect::<Vec<String>>();

            stacks[destination - 1].append(&mut cs);
        } else {
            for _ in 0..amount {
                let c = stacks[source - 1].pop().unwrap();
                stacks[destination - 1].push(c);
            }
        }
    }
}

fn top_crates(stacks: &Vec<Vec<String>>) -> String {
    stacks
        .iter()
        .map(|stack| {
            let j = stack.last().unwrap();
            j.as_str()
        })
        .collect::<Vec<&str>>()
        .join("")
}

#[test]
fn test_crane() {
    let str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";

    let (stacks, instructions) = parse(str);

    let mut stacks_a = stacks.clone();
    crane(&mut stacks_a, instructions.clone(), false);
    assert_eq!(
        vec![
            vec!["C".to_owned()],
            vec!["M".to_owned()],
            vec![
                "P".to_owned(),
                "D".to_owned(),
                "N".to_owned(),
                "Z".to_owned()
            ]
        ],
        stacks_a,
    );
    assert_eq!("CMZ".to_owned(), top_crates(&stacks_a));

    let mut stacks_b = stacks.clone();
    crane(&mut stacks_b, instructions, true);
    assert_eq!(
        vec![
            vec!["M".to_owned()],
            vec!["C".to_owned()],
            vec![
                "P".to_owned(),
                "Z".to_owned(),
                "N".to_owned(),
                "D".to_owned()
            ]
        ],
        stacks_b,
    );
    assert_eq!("MCD".to_owned(), top_crates(&stacks_b));
}

fn get_input() -> String {
    dotenv::dotenv().ok();
    let key = std::env::var("KEY").expect("Missing env var KEY");

    let bytes = std::fs::read("./input.txt.encrypted").unwrap();
    decrypt(key.as_bytes(), &bytes)
}

fn decrypt(key: &[u8], enc: &[u8]) -> String {
    String::from_utf8(
        enc.iter()
            .enumerate()
            .map(|(i, &b)| b.wrapping_sub(key[i % key.len()]))
            .collect(),
    )
    .unwrap()
}
