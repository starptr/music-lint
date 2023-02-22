use std::ops;
use crate::key::Key;

use crate::pitch::{Pitch, PitchMotion, OCTAVE};

pub struct SATBChord {
    soprano: Pitch,
    alto: Pitch,
    tenor: Pitch,
    bass: Pitch,
}

pub struct SATBMotion {
    soprano: PitchMotion,
    alto: PitchMotion,
    tenor: PitchMotion,
    bass: PitchMotion,
}

pub struct MContext {
    key: Key,
}

impl SATBChord {
    fn check_doubling(&self, context: &MContext) -> Result<(), String> {


        Ok(())
    }

    fn check_range(&self) -> Result<(), String> {
        let is_valid_soprano = self.soprano >= "c3".into() && self.soprano <= "g4".into();
        let is_valid_alto = self.alto >= "g2".into() && self.alto <= "d4".into();
        let is_valid_tenor = self.tenor >= "c2".into() && self.tenor <= "g3".into();
        let is_valid_bass = self.bass >= "e1".into() && self.bass <= "c3".into();
        if !is_valid_soprano {
            Err("Error: Range: Soprano out of range".to_string())
        } else if !is_valid_alto {
            Err("Error: Range: Alto out of range".to_string())
        } else if !is_valid_tenor {
            Err("Error: Range: Tenor out of range".to_string())
        } else if !is_valid_bass {
            Err("Error: Range: Bass out of range".to_string())
        } else {
            Ok(())
        }
    }

    fn check_spacing(&self) -> Result<(), String> {
        let spacing_soprano_alto = self.soprano - self.alto;
        let spacing_alto_tenor = self.alto - self.tenor;
        if spacing_soprano_alto > OCTAVE {
            Err("Error: Spacing: Soprano-Alto too far apart".to_string())
        } else if spacing_alto_tenor > OCTAVE {
            Err("Error: Spacing: Alto-Tenor too far apart".to_string())
        } else {
            Ok(())
        }
    }

    /**
     * Check rules about the current chord only.
     * Errors with message explaining broken rule.
     */
    pub fn verify(&self, context: &MContext) -> Result<(), String> {
        self.check_range()?;
        self.check_spacing()?;
        self.check_doubling(context)?;
        Ok(())
    }

    /**
     * Check rules not covered by `verify()` about the current and next chord only.
     * Errors with message explaining broken rule.
     */
    pub fn verify_motion(&self, next: &SATBChord) -> Result<(), String> {
        let motion = self - &next;

        // dummy
        Ok(())
    }
}

impl ops::Sub for SATBChord {
    type Output = SATBMotion;

    fn sub(self, rhs: Self) -> Self::Output {
        &self - &rhs
    }
}

impl ops::Sub for &SATBChord {
    type Output = SATBMotion;

    fn sub(self, rhs: Self) -> Self::Output {
        SATBMotion {
            soprano: self.soprano - rhs.soprano,
            alto: self.alto - rhs.alto,
            tenor: self.tenor - rhs.tenor,
            bass: self.bass - rhs.bass,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
