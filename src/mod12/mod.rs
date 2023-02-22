use std::ops;
use crate::key::Tone;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Mod12(pub i32);

impl Mod12 {
    pub fn new(val: i32) -> Self {
        Mod12((val % 12 + 12) % 12)
    }
}

impl From<i32> for Mod12 {
    fn from(value: i32) -> Self {
        Mod12::new(value)
    }
}

impl From<Tone> for Mod12 {
    fn from(value: Tone) -> Self {
        match value {
            Tone::Do => Mod12(0),
            Tone::Di => Mod12(1),
            Tone::Re => Mod12(2),
            Tone::Me => Mod12(3),
            Tone::Mi => Mod12(4),
            Tone::Fa => Mod12(5),
            Tone::Fi => Mod12(6),
            Tone::So => Mod12(7),
            Tone::Li => Mod12(8),
            Tone::La => Mod12(9),
            Tone::Su => Mod12(10),
            Tone::Si => Mod12(11),
        }
    }
}

impl ops::Add for Mod12 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Mod12::new(self.0 + rhs.0)
    }
}

impl ops::Sub for Mod12 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Mod12::new(self.0 - rhs.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn negatives() {
        assert_eq!(Mod12::from(-1), Mod12(11));
        assert_eq!(Mod12::from(-12), Mod12(0));
    }

    #[test]
    fn add() {
        assert_eq!(Mod12::new(5) + Mod12::new(10), Mod12(3));
        assert_eq!(Mod12::new(20) + Mod12::new(36), Mod12(8));
    }

    #[test]
    fn sub() {
        assert_eq!(Mod12::new(5) - Mod12::new(10), Mod12(7));
        assert_eq!(Mod12::new(24) - Mod12::new(2), Mod12(10));
    }
}
