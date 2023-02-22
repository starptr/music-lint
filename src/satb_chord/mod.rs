use std::ops;

use crate::pitch::{Pitch, PitchMotion};

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

impl ops::Sub for SATBChord {
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
