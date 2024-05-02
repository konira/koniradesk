extern crate libc;

use std::ffi::c_int;
pub use libc::c_int as other_c_int;
use std::slice;
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
pub fn encode_rgba_to_h264(rgba: *const u8, width: c_int, height: c_int) -> Vec<u8> {
    let mut output_size = 0;
    let output = unsafe { encode_rgba_to_h264(rgba.as_ptr(), width, height) };
    let output_slice = unsafe { slice::from_raw_parts(output, output_size as usize) };
    let output_vec = output_slice.to_vec();
    unsafe { libc::free(output as *mut libc::c_void) };
    output_vec
}


pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

