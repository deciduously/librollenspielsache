use super::*;
use crate::modifier::*;
use lazy_static::lazy_static;
use rand::{thread_rng, Rng};
use regex::Regex;

// A Roll represents a single computation, for instance "2d4+5"
#[derive(Debug, Clone, PartialEq)]
pub struct Roll {
    die: Die,
    repeat: usize,
    modifiers: Modifiers,
}

impl Roll {
    fn new(die: Die, repeat: usize, modifiers: Modifiers) -> Self {
        Self {
            die,
            repeat,
            modifiers,
        }
    }
    fn get_modifier(&self) -> isize {
        self.modifiers.net()
    }
    pub fn execute(&self) -> RollResult {
        RollResult::new(
            thread_rng().gen_range(1, self.die.sides as isize),
            self.get_modifier(),
        )
    }
}

impl Default for Roll {
    fn default() -> Self {
        Self {
            die: Die::default(),
            repeat: 1,
            modifiers: Modifiers::default(),
        }
    }
}

impl fmt::Display for Roll {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}d{} ({})", self.repeat, self.die, self.modifiers)
    }
}

impl RedisInterface for Roll {
    fn to_redis_hash(&self) -> RedisHash {
        let mut hash = RedisHash::new();
        hash.add_pair("die", &format!("{}", self.die));
        hash.add_pair("repeat", &format!("{}", self.repeat));
        hash.add_pair("modifiers", &format!("{}", self.modifiers)); // TODO this should really become a List
        hash
    }
}

lazy_static! {
    // Minimal match is just the character 'd' - that will roll 1d6+0
    pub static ref ROLL_RE: Regex = Regex::new(r"(?x)
    ^
    (?P<repeat>[[:digit:]]+)?
    d
    (?P<sides>[[:digit:]]+)?
    (
        (?P<sign>\+|-)?
        (?P<offset>[[:digit:]]+)?
    )?$
    ").unwrap();
}

impl FromStr for Roll {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if ROLL_RE.is_match(s) {
            let caps = ROLL_RE.captures(s).unwrap();
            let mut negative = false;
            if let Some(sign_s) = caps.name("sign") {
                negative = sign_s.as_str().starts_with('-');
            }
            let mut offset = 0;
            if let Some(offset_s) = caps.name("offset") {
                offset += offset_s.as_str().parse::<isize>().unwrap();
            }

            let modifiers = vec![Modifier::new(offset, negative, ModifierType::Unspecified)];
            let die = match caps.name("sides") {
                Some(sides_s) => Die::new(sides_s.as_str().parse::<usize>().unwrap()),
                None => Die::default(),
            };
            let mut repeat = 1;
            if let Some(repeat_s) = caps.name("repeat") {
                repeat = repeat_s.as_str().parse::<usize>().unwrap();
            }

            Ok(Self::new(die, repeat, Modifiers::from(modifiers)))
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Bad format: {}", s),
            ))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    #[test]
    fn test_roll_from_string() {
        assert_eq!(
            Roll::from_str("2d20").unwrap(),
            Roll {
                die: Die { sides: 20 },
                repeat: 2,
                modifiers: Modifiers::default()
            }
        );
        assert_eq!(
            Roll::from_str("34d100+3").unwrap(),
            Roll {
                die: Die { sides: 100 },
                repeat: 34,
                modifiers: Modifiers::from(vec![Modifier {
                    offset: 3,
                    variant: ModifierType::Unspecified
                }])
            }
        );
        assert_eq!(
            Roll::from_str("d5-20").unwrap(),
            Roll {
                die: Die { sides: 5 },
                repeat: 1,
                modifiers: Modifiers::from(vec![Modifier {
                    offset: -20,
                    variant: ModifierType::Unspecified
                }])
            }
        );
        assert_eq!(Roll::from_str("d").unwrap(), Roll::default());
    }
}
