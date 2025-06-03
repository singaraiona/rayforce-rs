#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::ffi::{CStr, CString};
use std::fmt;
use std::os::raw::c_char;
use std::ptr;

// Include the generated bindings
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[derive(Debug)]
pub enum RayforceError {
    RuntimeCreationFailed,
}

impl std::fmt::Display for RayforceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RayforceError::RuntimeCreationFailed => write!(f, "Failed to create runtime"),
        }
    }
}

impl std::error::Error for RayforceError {}

pub struct RayforceBuilder {
    args: Vec<CString>,
}

impl RayforceBuilder {
    pub fn new() -> Self {
        Self {
            args: vec![CString::new("rayforce").unwrap()],
        }
    }

    pub fn with_arg(mut self, arg: &str) -> Self {
        self.args.push(CString::new(arg).unwrap());
        self
    }

    pub fn build(self) -> Result<Rayforce, RayforceError> {
        unsafe {
            let mut c_args: Vec<*mut c_char> = self
                .args
                .iter()
                .map(|arg| arg.as_ptr() as *mut c_char)
                .collect();
            c_args.push(ptr::null_mut());

            let runtime = runtime_create(c_args.len() as i32 - 1, c_args.as_mut_ptr());
            if !runtime.is_null() {
                Ok(Rayforce { runtime })
            } else {
                Err(RayforceError::RuntimeCreationFailed)
            }
        }
    }
}

pub struct Rayforce {
    runtime: *mut runtime_t,
}

// Since runtime_t is a C pointer, we need to manually implement Send and Sync
unsafe impl Send for Rayforce {}
unsafe impl Sync for Rayforce {}

impl Rayforce {
    pub fn new() -> Result<Self, RayforceError> {
        RayforceBuilder::new().with_arg("-r").with_arg("0").build()
    }

    pub fn get_version(&self) -> u8 {
        unsafe { version() }
    }

    pub fn run(&self) -> i32 {
        unsafe { runtime_run() }
    }

    pub fn as_ptr(&self) -> *mut runtime_t {
        self.runtime
    }

    pub fn eval_str(&self, code: &CStr) -> RayObj {
        unsafe {
            let obj = eval_str(code.as_ptr() as *const i8);
            RayObj::from_raw(obj)
        }
    }

    pub fn eval_obj_str(&self, code: &RayObj, name: &RayObj) -> RayObj {
        unsafe {
            let obj = ray_eval_str(code.as_ptr() as *mut obj_t, name.as_ptr() as *mut obj_t);
            RayObj::from_raw(obj)
        }
    }
}

impl Drop for Rayforce {
    fn drop(&mut self) {
        unsafe { runtime_destroy() };
    }
}

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
    pub fn type_of(&self) -> i8 {
        unsafe { (*self.ptr).type_ }
    }

    pub fn is_nil(&self) -> bool {
        unsafe { is_null(self.ptr) == 1 }
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

impl From<&[i64]> for RayObj {
    fn from(val: &[i64]) -> Self {
        unsafe {
            let mut obj = RayObj::from_raw(vector(TYPE_I64 as i8, val.len() as i64));
            <RayObj as AsMut<[i64]>>::as_mut(&mut obj).copy_from_slice(val);
            obj
        }
    }
}

impl From<bool> for RayObj {
    fn from(val: bool) -> Self {
        unsafe {
            let ptr = b8(val as b8_t);
            RayObj { ptr }
        }
    }
}

impl From<u8> for RayObj {
    fn from(val: u8) -> Self {
        unsafe {
            let ptr = u8_(val as u8_t);
            RayObj { ptr }
        }
    }
}

impl From<i16> for RayObj {
    fn from(val: i16) -> Self {
        unsafe {
            let ptr = i16_(val as i16_t);
            RayObj { ptr }
        }
    }
}

impl From<i32> for RayObj {
    fn from(val: i32) -> Self {
        unsafe {
            let ptr = i32_(val as i32_t);
            RayObj { ptr }
        }
    }
}

impl From<f64> for RayObj {
    fn from(val: f64) -> Self {
        unsafe {
            let ptr = f64_(val as f64_t);
            RayObj { ptr }
        }
    }
}

impl From<&str> for RayObj {
    fn from(val: &str) -> Self {
        unsafe {
            let ptr = string_from_str(val.as_ptr() as *const i8, val.len() as i64);
            RayObj { ptr }
        }
    }
}

impl From<String> for RayObj {
    fn from(val: String) -> Self {
        unsafe {
            let ptr = string_from_str(val.as_ptr() as *const i8, val.len() as i64);
            RayObj { ptr }
        }
    }
}

impl AsMut<i64> for RayObj {
    fn as_mut(&mut self) -> &mut i64 {
        unsafe { (*self.ptr).__bindgen_anon_1.i64_.as_mut() }
    }
}

impl AsMut<f64> for RayObj {
    fn as_mut(&mut self) -> &mut f64 {
        unsafe { (*self.ptr).__bindgen_anon_1.f64_.as_mut() }
    }
}

impl AsMut<[i64]> for RayObj {
    fn as_mut(&mut self) -> &mut [i64] {
        unsafe {
            let anon = &mut (*self.ptr).__bindgen_anon_1.__bindgen_anon_1;
            let len = anon.as_mut().len as usize;
            let raw = anon.as_mut().raw.as_mut_ptr() as *mut i64;
            std::slice::from_raw_parts_mut(raw, len)
        }
    }
}

impl AsMut<bool> for RayObj {
    fn as_mut(&mut self) -> &mut bool {
        unsafe {
            let b8_ptr = (*self.ptr).__bindgen_anon_1.b8.as_mut();
            std::mem::transmute::<&mut i8, &mut bool>(b8_ptr)
        }
    }
}

impl AsMut<u8> for RayObj {
    fn as_mut(&mut self) -> &mut u8 {
        unsafe { (*self.ptr).__bindgen_anon_1.u8_.as_mut() }
    }
}

impl AsMut<i16> for RayObj {
    fn as_mut(&mut self) -> &mut i16 {
        unsafe { (*self.ptr).__bindgen_anon_1.i16_.as_mut() }
    }
}

impl AsMut<i32> for RayObj {
    fn as_mut(&mut self) -> &mut i32 {
        unsafe { (*self.ptr).__bindgen_anon_1.i32_.as_mut() }
    }
}

impl AsMut<[u8]> for RayObj {
    fn as_mut(&mut self) -> &mut [u8] {
        unsafe {
            let anon = &mut (*self.ptr).__bindgen_anon_1.__bindgen_anon_1;
            let len = anon.as_mut().len as usize;
            let raw = anon.as_mut().raw.as_mut_ptr() as *mut u8;
            std::slice::from_raw_parts_mut(raw, len)
        }
    }
}

impl AsMut<[i16]> for RayObj {
    fn as_mut(&mut self) -> &mut [i16] {
        unsafe {
            let anon = &mut (*self.ptr).__bindgen_anon_1.__bindgen_anon_1;
            let len = anon.as_mut().len as usize;
            let raw = anon.as_mut().raw.as_mut_ptr() as *mut i16;
            std::slice::from_raw_parts_mut(raw, len)
        }
    }
}

impl AsMut<[i32]> for RayObj {
    fn as_mut(&mut self) -> &mut [i32] {
        unsafe {
            let anon = &mut (*self.ptr).__bindgen_anon_1.__bindgen_anon_1;
            let len = anon.as_mut().len as usize;
            let raw = anon.as_mut().raw.as_mut_ptr() as *mut i32;
            std::slice::from_raw_parts_mut(raw, len)
        }
    }
}

impl AsMut<[f64]> for RayObj {
    fn as_mut(&mut self) -> &mut [f64] {
        unsafe {
            let anon = &mut (*self.ptr).__bindgen_anon_1.__bindgen_anon_1;
            let len = anon.as_mut().len as usize;
            let raw = anon.as_mut().raw.as_mut_ptr() as *mut f64;
            std::slice::from_raw_parts_mut(raw, len)
        }
    }
}

impl AsMut<str> for RayObj {
    fn as_mut(&mut self) -> &mut str {
        unsafe {
            let anon = &mut (*self.ptr).__bindgen_anon_1.__bindgen_anon_1;
            let len = anon.as_mut().len as usize;
            let raw = anon.as_mut().raw.as_mut_ptr() as *mut u8;
            std::str::from_utf8_unchecked_mut(std::slice::from_raw_parts_mut(raw, len))
        }
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

impl Into<i64> for RayObj {
    fn into(self) -> i64 {
        unsafe { *(*self.ptr).__bindgen_anon_1.i64_.as_ref() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_obj_structure() {
        let rayforce = Rayforce::new().unwrap();

        let obj1 = RayObj::from(123);
        let val: i64 = obj1.into();
        assert_eq!(val, 123);

        let vec = vec![1, 2, 3];
        let mut obj2 = RayObj::from(vec.as_slice());
        let val: &mut [i64] = obj2.as_mut();
        assert_eq!(val, vec.as_slice());

        let mut obj3 = RayObj::from("Hello, world!");
        let val: &mut str = obj3.as_mut();
        assert_eq!(val, "Hello, world!");

        drop(rayforce);
    }
}
