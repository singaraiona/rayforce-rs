#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// Include the generated bindings
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;
    use std::os::raw::c_char;
    use std::ptr;

    #[test]
    fn test_rayforce_init_and_version() {
        let args = vec![
            CString::new("rayforce").unwrap(),
            CString::new("-r").unwrap(),
            CString::new("0").unwrap(),
        ];
        let mut c_args: Vec<*mut c_char> =
            args.iter().map(|arg| arg.as_ptr() as *mut c_char).collect();
        c_args.push(ptr::null_mut());

        unsafe {
            let runtime = runtime_create(c_args.len() as i32 - 1, c_args.as_mut_ptr());
            assert!(!runtime.is_null(), "Runtime should not be null");
            let version = version();
            assert!(version > 0, "Version should be positive");
        }
    }
}
