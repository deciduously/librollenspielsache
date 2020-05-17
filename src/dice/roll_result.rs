//! A RollResult is the object returned by Roll::execute(), including a copy of the original roll
use super::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct RollResult {
    roll: Roll,
    base: isize,
    offset: isize,
}

impl Default for RollResult {
    fn default() -> Self {
        Self { roll: Roll::default(), base: 1, offset: 0 }
    }
}

impl RollResult {
    pub fn new(roll: Roll, base: isize, offset: isize) -> Self {
        Self { roll, base, offset }
    }
    pub fn total(&self) -> isize {
        self.base + self.offset
    }
}

impl fmt::Display for RollResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let sign = if self.offset >= 0 { "+" } else { "" };
        write!(f, "{}: {} ({}{})", self.roll, self.base, sign, self.offset)
    }
}
