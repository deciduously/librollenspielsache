//! skill.rs is concerned with skills and abilities.

use crate::dice::Roll;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
pub struct Skill {
    name: String,
    short_name: String,
    roll: Roll,
}

impl Skill {
    pub fn new(name: &str, short_name: &str, roll_str: &str) -> Self {
        Self {
            name: name.into(),
            short_name: short_name.into(),
            roll: Roll::from_str(roll_str).unwrap(),
        }
    }
}

impl fmt::Display for Skill {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({})", self.name, self.roll)
    }
}
