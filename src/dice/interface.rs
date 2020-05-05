use super::*;
use libc::c_char;
use std::ffi::{CStr, CString};

/// Construct a new Roll object from a string.  This is the only way to build at present.
/// # Safety
/// This function unsafely converts a *const ch_char to a CStr
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

/// Free the Roll object
/// # Safety
/// If pointer is null, simply returns.
#[no_mangle]
pub unsafe extern "C" fn roll_free(ptr: *mut Roll) {
    if ptr.is_null() {
        return;
    }
    Box::from_raw(ptr);
}

/// Getter for base
/// # Safety
/// Checks for null pointer, panics
#[no_mangle]
pub unsafe extern "C" fn roll_base(ptr: *const Roll) -> usize {
    // Attempt to use the raw pointer a a Roll object
    let roll = {
        assert!(!ptr.is_null());
        &*ptr // unsafe
    };
    roll.get_base()
}

/// Getter for repeat
/// # Safety
/// Checks for null pointer, panics
#[no_mangle]
pub unsafe extern "C" fn roll_repeat(ptr: *const Roll) -> usize {
    // Attempt to use the raw pointer a a Roll object
    let roll = {
        assert!(!ptr.is_null());
        &*ptr // unsafe
    };
    roll.get_repeat()
}

// TODO modifier getter?

/// Execute a roll
/// # Safety
/// This function unsafely turns a raw pointer to a reference
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

/// Free the RollResult object
/// # Safety
/// If pointer is null, simply returns.
#[no_mangle]
pub unsafe extern "C" fn roll_result_free(ptr: *mut RollResult) {
    if ptr.is_null() {
        return;
    }
    Box::from_raw(ptr);
}
