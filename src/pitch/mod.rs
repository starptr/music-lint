use std::str::FromStr;
use std::ops;
use regex::Regex;
use lazy_static::lazy_static;

#[derive(Debug, PartialEq, Eq)]
pub struct Pitch {
    spn_idx: i32,
}

fn parse_pitch_decomp(spn: &str) -> Result<(i32, i32, i32), String> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(?x)
(?P<name>[a-g])
(?P<accidental>[snf])?
(?P<octave>\d)
$").unwrap();
    }

    if !RE.is_match(spn) {
        return Err(format!("Invalid SPN: {}", spn));
    }

    let caps = RE.captures(spn).unwrap();

    let name: &str = caps.name("name").unwrap().into();
    let accidental: &str = caps.name("accidental").map_or("n", |match_str| match_str.into());
    let octave: &str = caps.name("octave").unwrap().into();

    let name_idx = match name {
        "c" => 0,
        "d" => 2,
        "e" => 4,
        "f" => 5,
        "g" => 7,
        "a" => 9,
        "b" => 11,
        _ => unreachable!(),
    };
    let accidental_idx = match accidental {
        "s" => 1,
        "n" => 0,
        "f" => -1,
        _ => unreachable!(),
    };
    let octave_idx = octave.parse::<i32>().unwrap() * 12;

    Ok((name_idx, accidental_idx, octave_idx))
}

fn pitch_decomp_to_spn_idx(pitch_decomp: (i32, i32, i32)) -> i32 {
    return pitch_decomp.0 + pitch_decomp.1 + pitch_decomp.2;
}

impl ops::Sub for Pitch {
    type Output = i32;

    fn sub(self, rhs: Self) -> Self::Output {
        self.spn_idx - rhs.spn_idx
    }
}

impl FromStr for Pitch {
    type Err = String;

    fn from_str(spn: &str) -> Result<Self, Self::Err> {
        let pitch_decomp = parse_pitch_decomp(spn)?;

        // for now, just return a dummy value
        Ok(Pitch { spn_idx: pitch_decomp_to_spn_idx(pitch_decomp) })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod idx_agnostic_tests {
        use super::*;

        #[test]
        fn sub_basic() {
            assert_eq!(7, Pitch::from_str("g3").unwrap() - Pitch::from_str("c3").unwrap());
            assert_eq!(8, Pitch::from_str("gs3").unwrap() - Pitch::from_str("c3").unwrap());
            assert_eq!(7, Pitch::from_str("gs3").unwrap() - Pitch::from_str("cs3").unwrap());
            assert_eq!(6, Pitch::from_str("g3").unwrap() - Pitch::from_str("cs3").unwrap());
            assert_eq!(5, Pitch::from_str("gf3").unwrap() - Pitch::from_str("cs3").unwrap());
            assert_eq!(9, Pitch::from_str("gs3").unwrap() - Pitch::from_str("cf3").unwrap());
        }
    }

    mod spn_idx_tests {
        use super::*;

        #[test]
        fn idx_basic_decomp() {
            let ans = parse_pitch_decomp("c3");
            assert_eq!(ans, Ok((0, 0, 36)));
        }

        #[test]
        fn idx_decomp_naturals() {
            assert_eq!(Ok((0, 0, 36)), parse_pitch_decomp("c3"));
            assert_eq!(Ok((7, 0, 36)), parse_pitch_decomp("g3"));
            assert_eq!(Ok((0, 0, 36)), parse_pitch_decomp("cn3"));
            assert_eq!(Ok((7, 0, 36)), parse_pitch_decomp("gn3"));
            assert_eq!(Ok((0, 0, 24)), parse_pitch_decomp("c2"));
            assert_eq!(Ok((7, 0, 48)), parse_pitch_decomp("g4"));
            assert_eq!(Ok((0, 0, 12)), parse_pitch_decomp("cn1"));
            assert_eq!(Ok((7, 0, 60)), parse_pitch_decomp("gn5"));
        }

        #[test]
        fn idx_decomp_sharp() {
            assert_eq!(Ok((0, 1, 36)), parse_pitch_decomp("cs3"));
            assert_eq!(Ok((7, 1, 36)), parse_pitch_decomp("gs3"));
            assert_eq!(Ok((0, 1, 24)), parse_pitch_decomp("cs2"));
            assert_eq!(Ok((7, 1, 48)), parse_pitch_decomp("gs4"));
        }

        #[test]
        fn idx_decomp_flat() {
            assert_eq!(Ok((0, -1, 36)), parse_pitch_decomp("cf3"));
            assert_eq!(Ok((7, -1, 36)), parse_pitch_decomp("gf3"));
            assert_eq!(Ok((0, -1, 24)), parse_pitch_decomp("cf2"));
            assert_eq!(Ok((7, -1, 48)), parse_pitch_decomp("gf4"));
        }

        #[test]
        fn idx_basic() {
            assert_eq!(Ok(Pitch { spn_idx: 36 }), Pitch::from_str("c3"));
            assert_eq!(Ok(Pitch { spn_idx: 43 }), Pitch::from_str("g3"));
            assert_eq!(Ok(Pitch { spn_idx: 24 }), Pitch::from_str("c2"));
        }
    }
}
