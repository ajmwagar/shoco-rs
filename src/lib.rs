extern crate shoco_sys as ffi;
use std::os::raw::c_char;
use std::ffi::CStr;


/// Compress a string and return the value as a Vec<u8>
pub fn compress(s: &str) -> Vec<u8> {
    unsafe {
        let out_buffer: *mut c_char = std::mem::uninitialized();
        ffi::shoco_compress(s.as_ptr() as *mut c_char, s.len(), out_buffer, s.len() * 4);
        let c_str = CStr::from_ptr(out_buffer);
        c_str.to_bytes().to_vec()
    }
}

/// Compress a string and return the value as a Vec<u8>
pub fn decompress(bytes: &[u8]) -> &str {
    unsafe {
        let out_buffer: *mut c_char = std::mem::uninitialized();
        ffi::shoco_decompress(bytes.as_ptr() as *mut c_char, bytes.len(), out_buffer, bytes.len() * 4);
        let c_str = CStr::from_ptr(out_buffer);
        c_str.to_str().unwrap()
    }
}
