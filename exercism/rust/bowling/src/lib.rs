#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Default, Debug)]
struct Frame {
    i: usize,
    first_roll: Option<u16>,
}

#[derive(Default, Debug)]
pub struct BowlingGame {
    running_score: u16,
    final_score: Option<u16>,
    frame: Frame,
    multipliers: [u16; 2],
}

const PINS: u16 = 10;
const FILL: usize = 10;

impl BowlingGame {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.final_score.is_some() {
            return Err(Error::GameComplete);
        }
        // Pins
        let multiplier = self.multipliers.iter().filter(|m| *m > &0).count() as u16;
        if self.frame.i < FILL {
            self.running_score += pins * (multiplier + 1);
        } else {
            self.running_score += pins * multiplier;
        }
        self.multipliers.iter_mut().for_each(|m| *m = 1.max(*m) - 1);
        // Endgame
        if self.frame.i >= FILL && self.multipliers.iter().all(|m| *m == 0) {
            self.final_score = Some(self.running_score);
            return Ok(());
        }
        // Check errors and set frame state
        if let Some(first_roll) = self.frame.first_roll {
            if first_roll + pins > PINS {
                return Err(Error::NotEnoughPinsLeft);
            }
            if first_roll + pins == PINS {
                if self.frame.i < FILL {
                    let i = if self.multipliers[0] == 0 { 0 } else { 1 };
                    self.multipliers[i] = 1;
                }
            }
            self.frame.first_roll = None;
            self.frame.i += 1;
        } else {
            if pins > PINS {
                return Err(Error::NotEnoughPinsLeft);
            }
            if pins == PINS {
                if self.frame.i < FILL {
                    let i = if self.multipliers[0] == 0 { 0 } else { 1 };
                    self.multipliers[i] = 2;
                }
                self.frame.i += 1;
            } else {
                self.frame.first_roll = Some(pins);
            }
        }
        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        Some(self.final_score?)
    }
}
