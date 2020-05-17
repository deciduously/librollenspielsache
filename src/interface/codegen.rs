//! Code generation for FFI interfaces

macro_rules! ffi_to_string {
    ($type:ident) => {
        #[no_mangle]
        pub unsafe extern "C" fn $name(ptr: *const $type) -> *mut c_char {
            let rust_type = {
                assert!(!ptr.is_null());
            }
        }
    }
}