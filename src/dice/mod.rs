/*!
 * Dice is responsible for executing dice rolls.
 */

mod die;
pub mod interface;
mod roll;
mod roll_result;

use crate::redis::*;
use std::{fmt, str::FromStr};

pub use die::*;
pub use roll::*;
pub use roll_result::*;
