//! Interface contains all the unsafe FFI code.  It should be optional one da, or perhaps even its own crate.

use libc::c_char;
use std::ffi::{CStr, CString};

mod dice;
mod ffi_string;
pub use self::dice::*;
pub use self::ffi_string::*;
