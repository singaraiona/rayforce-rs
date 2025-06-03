#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::fmt;

// Include the generated bindings
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

/// A safe wrapper around the Rayforce object pointer
pub struct RayObj {
    ptr: *mut obj_t,
}

impl RayObj {
    /// Create a new RayObj from a raw pointer
    pub unsafe fn from_raw(ptr: *mut obj_t) -> Self {
        Self { ptr }
    }

    /// Get the raw pointer
    pub fn as_ptr(&self) -> *mut obj_t {
        self.ptr
    }

    /// Get the type of the object
    pub fn type_(&self) -> i8 {
        unsafe { (*self.ptr).type_ }
    }
}

impl Clone for RayObj {
    fn clone(&self) -> Self {
        unsafe { RayObj::from_raw(clone_obj(self.ptr)) }
    }
}

impl Drop for RayObj {
    fn drop(&mut self) {
        unsafe { drop_obj(self.ptr) }
    }
}

impl From<i64> for RayObj {
    fn from(val: i64) -> Self {
        unsafe { RayObj::from_raw(i64_(val)) }
    }
}

impl Into<i64> for RayObj {
    fn into(self) -> i64 {
        unsafe { *(*self.ptr).__bindgen_anon_1.i64_.as_ref() }
    }
}

impl fmt::Display for RayObj {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unsafe {
            let obj = obj_fmt(self.ptr, 0);
            if obj.is_null() {
                write!(f, "null")
            } else {
                let anon = &(*obj).__bindgen_anon_1.__bindgen_anon_1;
                let len = anon.as_ref().len as usize;
                let raw = anon.as_ref().raw.as_ptr() as *const u8;
                let bytes = std::slice::from_raw_parts(raw, len);
                let s = String::from_utf8_lossy(bytes);
                write!(f, "{s}")
            }
        }
    }
}

impl fmt::Debug for RayObj {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unsafe {
            let obj = obj_fmt(self.ptr, 1);
            if obj.is_null() {
                write!(f, "null")
            } else {
                let anon = &(*obj).__bindgen_anon_1.__bindgen_anon_1;
                let len = anon.as_ref().len as usize;
                let raw = anon.as_ref().raw.as_ptr() as *const u8;
                let bytes = std::slice::from_raw_parts(raw, len);
                let s = String::from_utf8_lossy(bytes);
                write!(f, "{s}")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_obj_structure() {
        unsafe { ray_init() };
        let obj = RayObj::from(123);
        let val: i64 = obj.into();
        assert_eq!(val, 123);
        unsafe { ray_clean() };
    }
}
