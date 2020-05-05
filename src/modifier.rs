/// modifier.rs encodes all the different tyeps of modifier
use crate::skill::Skill;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum ModifierType {
    Skill(Skill),
    Unspecified,
}

impl fmt::Display for ModifierType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ModifierType::*;
        let variant_s = match self {
            Skill(s) => s.to_string(),
            Unspecified => "".to_string(),
        };
        write!(f, "{}", &variant_s)
    }
}

// A Modifier represents an offset to apply to a roll result
#[derive(Debug, Clone, PartialEq)]
pub struct Modifier {
    pub offset: isize,
    pub variant: ModifierType,
}

impl Modifier {
    pub fn new(offset: isize, negative: bool, variant: ModifierType) -> Self {
        Self {
            offset: (if negative { -offset } else { offset }),
            variant,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Modifiers(Vec<Modifier>);

impl Modifiers {
    pub fn net(&self) -> isize {
        self.0.iter().fold(0, |acc, el| acc + el.offset)
    }
}

impl Default for Modifiers {
    fn default() -> Self {
        Self(vec![Modifier {
            offset: 0,
            variant: ModifierType::Unspecified,
        }])
    }
}

impl fmt::Display for Modifiers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl From<Vec<Modifier>> for Modifiers {
    fn from(v: Vec<Modifier>) -> Self {
        Self(v)
    }
}
