/*!
 * Dice is responsible for executing dice rolls.
 */

mod die;
mod roll;
mod roll_result;

use crate::redis::*;
use serde::Serialize;
use std::{fmt, str::FromStr};

pub use die::*;
pub use roll::*;
pub use roll_result::*;
