mod pitch;
mod satb_chord;
mod mod12;
mod key;
use key::Key;

#[derive(Clone, Copy)]
enum ParseState {
    Context,
    Roll,
}
enum State {
    Parse(ParseState),
    Verify,
}

fn main() {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let mut state = State::Parse(ParseState::Context);

    let file = File::open("input.txt").expect("input.txt not found");
    let reader = BufReader::new(file);
    let mut key: Option<Key> = None;

    for line in reader.lines() {
        let line = line.unwrap();
        let line = line.trim();

        match state {
            State::Parse(parse_state) => {
                match parse_state {
                    ParseState::Context => {
                        if line.starts_with("context:") {
                            state = State::Parse(ParseState::Context);
                        } else if line.starts_with("keysig:") {
                            let rest_idx = line.find(' ').expect("Keysig missing arg");
                            let (_, rest) = line.split_at(rest_idx + 1);
                            key = Some(rest.parse::<Key>().expect("Keysig arg not a key"));
                        } else if line.starts_with("roll:") {
                            state = State::Parse(ParseState::Roll);
                        }
                    }
                    ParseState::Roll => {
                        let tokenized: Vec<_> = line.split(' ').collect();
                    }
                }
            }
            State::Verify => {
                // Verify roll
                state = State::Parse(ParseState::Context);
            }
        }
    }
}
