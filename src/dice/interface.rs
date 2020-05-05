use super::*;
use libc::c_char;
use std::ffi::{CStr, CString};

/// # Safety
/// This function unsafely converts a *const ch_char to a CStr
///
/// Construct a new Roll object from a string
#[no_mangle]
pub unsafe extern "C" fn roll_from_str(s: *const c_char) -> *mut Roll {
    // Get a Rust string form the char*
    let c_str = {
        assert!(!s.is_null());
        CStr::from_ptr(s) // unsafe
    };
    let r_str = c_str.to_str().unwrap();

    // Construct and return
    let roll = Roll::from_str(r_str).unwrap();
    Box::into_raw(Box::new(roll))
}

/// # Safety
/// This function unsafely turns a raw pointer to a reference
/// Execute a roll
#[no_mangle]
pub unsafe extern "C" fn roll_execute(ptr: *const Roll) -> *mut RollResult {
    // Attempt to use the raw pointer a a Roll object
    let roll = {
        assert!(!ptr.is_null());
        &*ptr // unsafe
    };

    // Obtain result, return pointer
    let result = roll.execute();
    Box::into_raw(Box::new(result))
}

/// Get the string representation of a roll result
#[no_mangle]
pub unsafe extern "C" fn roll_result_to_string(ptr: *const RollResult) -> *mut c_char {
    let result = {
        assert!(!ptr.is_null());
        &*ptr // unsafe
    };
    let c_str = CString::new(result.to_string()).unwrap();
    c_str.into_raw()
}
