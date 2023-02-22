use std::str::FromStr;
use std::ops;
use strum_macros::EnumString;

use crate::mod12::Mod12;

pub enum Mode {
    Major,
    Minor,
}

pub struct Key {
    tone: Tone,
    mode: Mode,
}

impl Key {
    pub fn get_leading(&self) -> Tone {
        let numeric: Mod12 = self.tone.into();
        Tone::from(numeric - 1.into())
    }
}

impl FromStr for Key {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Key {
            tone: Tone::from_str(s)?,
            mode: if s.chars().nth(0).expect("s is empty").is_uppercase() {
                Mode::Major
            } else {
                Mode::Minor
            },
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Tone {
    Do,
    Di,
    Re,
    Me,
    Mi,
    Fa,
    Fi,
    So,
    Li,
    La,
    Su,
    Si,
}

impl FromStr for Tone {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();
        match s.as_str() {
            "c" => Ok(Tone::Do),
            "cs" => Ok(Tone::Di),
            "d" => Ok(Tone::Re),
            "ef" => Ok(Tone::Me),
            "e" => Ok(Tone::Mi),
            "f" => Ok(Tone::Fa),
            "fs" => Ok(Tone::Fi),
            "g" => Ok(Tone::So),
            "af" => Ok(Tone::Li),
            "a" => Ok(Tone::La),
            "bb" => Ok(Tone::Su),
            "b" => Ok(Tone::Si),
            _ => Err("Invalid tone".to_owned()),
        }
    }
}

impl From<Mod12> for Tone {
    fn from(value: Mod12) -> Self {
        match value {
            Mod12(0) => Tone::Do,
            Mod12(1) => Tone::Di,
            Mod12(2) => Tone::Re,
            Mod12(3) => Tone::Me,
            Mod12(4) => Tone::Mi,
            Mod12(5) => Tone::Fa,
            Mod12(6) => Tone::Fi,
            Mod12(7) => Tone::So,
            Mod12(8) => Tone::Li,
            Mod12(9) => Tone::La,
            Mod12(10) => Tone::Su,
            Mod12(11) => Tone::Si,
            _ => unreachable!(),
        }
    }
}
