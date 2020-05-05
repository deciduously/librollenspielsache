//! Supported RPG-specific rules.

use crate::character::Character;

trait GameSystem {
    fn attack(aggressor: &mut Character, recipient: &mut Character);
}
