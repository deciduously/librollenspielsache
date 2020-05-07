/// roll_result.rs defines the object returned by Roll::execute()
use super::*;

#[derive(Debug, Serialize)]
pub struct RollResult {
    base: isize,
    offset: isize,
}

impl Default for RollResult {
    fn default() -> Self {
        Self { base: 1, offset: 0 }
    }
}

impl RollResult {
    pub fn new(base: isize, offset: isize) -> Self {
        Self { base, offset }
    }
    pub fn total(&self) -> isize {
        self.base + self.offset
    }
}

impl fmt::Display for RollResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let sign = if self.offset >= 0 { "+" } else { "" };
        write!(f, "{} ({}{})", self.base, sign, self.offset)
    }
}
