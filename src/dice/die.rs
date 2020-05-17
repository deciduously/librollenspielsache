use super::*;
use rand::{thread_rng, Rng};

/// A Die represents a single N-sided die
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Die {
    pub sides: usize,
}

impl Die {
    pub fn new(sides: usize) -> Self {
        Self { sides }
    }

    pub fn roll(self) -> usize {
        thread_rng().gen_range(1, self.sides as usize)
    }
}

impl Default for Die {
    fn default() -> Self {
        Self::new(6)
    }
}

impl std::fmt::Display for Die {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.sides)
    }
}

impl FromStr for Die {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Die::new(s.parse::<usize>()?))
    }
}
