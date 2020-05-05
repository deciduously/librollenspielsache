/*!
    The Rollenspielsache is a set of tools for managing tabletop RPG games.
    The goal is to handle the mechanics seamlessly, allowing you to participate in
    or run a tabletop encounter without getting distracted by rule clarifications.

    This library is the core API defining all the entities involved and their relationships.

    # Types
    * Roll
    * RollResult
*/

pub mod character;
pub mod dice;
pub mod error;
pub mod game_system;
pub mod interface;
pub mod modifier;
pub mod redis;
pub mod skill;

pub use interface::*;
pub use dice::interface::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
