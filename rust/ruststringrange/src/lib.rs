use std::rc::Rc;
use core::ffi::c_void;

use compare_with_wildcard::compare_with_wildcard;

// Don't bind the struct itself, instead just expose functions receiving and returning void*
// The C++ wrapper will care those void pointers

pub struct RustStringRange {
  string: Rc<String>,
  offset: usize,
  length: usize,
}

impl RustStringRange {
  pub fn len(&self) -> usize {
    self.length
  }

  fn from_string(s: String) -> RustStringRange {
    let string = Rc::new(s);
    let length = string.len();

    RustStringRange {
      string,
      offset: 0,
      length
    }
  }

  pub fn from_str(s: &str) -> RustStringRange {
    let string = String::from(s);
    RustStringRange::from_string(string)
  }

  pub unsafe fn from_buffer_utf16(buffer: *const u16, buffer_len: usize) -> RustStringRange {
    let buffer_slice: &[u16] = std::slice::from_raw_parts(buffer, buffer_len);

    RustStringRange::from_string(String::from_utf16_lossy(buffer_slice))
  }

  pub unsafe fn from_buffer_utf8(buffer: *const u8, buffer_len: usize) -> RustStringRange {
    let buffer_slice: &[u8] = std::slice::from_raw_parts(buffer, buffer_len);

    RustStringRange::from_str(&String::from_utf8_lossy(buffer_slice))
  }

  pub unsafe fn from_void(p: *mut c_void) -> Box<RustStringRange> {
    Box::from_raw(p as *mut RustStringRange)
  }

  pub unsafe fn as_ptr(&self) -> *const u8 {
    self.string.as_ptr().offset(self.offset as isize)
  }

  pub fn as_slice(&self) -> &str {
    &self.string[self.offset..self.offset + self.length]
  }
}

#[no_mangle]
pub unsafe extern fn ruststringrange_new(buffer: *const u16, buffer_len: usize) -> *mut c_void {
  Box::into_raw(Box::new(RustStringRange::from_buffer_utf16(buffer, buffer_len))) as *mut c_void
}

#[no_mangle]
pub unsafe extern fn ruststringrange_new_utf8(buffer: *const u8, buffer_len: usize) -> *mut c_void {
  Box::into_raw(Box::new(RustStringRange::from_buffer_utf8(buffer, buffer_len))) as *mut c_void
}

#[no_mangle]
pub unsafe extern fn ruststringrange_free(p: *mut c_void) -> () {
  RustStringRange::from_void(p); // implicit cleanup
}

#[no_mangle]
pub unsafe extern fn ruststringrange_raw(p: *mut c_void) -> *const u8 {
  let rsr = Box::leak(RustStringRange::from_void(p));
  rsr.as_ptr()
}

#[no_mangle]
pub unsafe extern fn ruststringrange_len(p: *const c_void) -> usize {
  let rsr = Box::leak(RustStringRange::from_void(p as *mut c_void));
  rsr.len()
}

#[no_mangle]
pub unsafe extern fn ruststringrange_compare_with_wildcard(x_raw: *mut c_void, y_raw: *mut c_void) -> bool {
  let x = Box::leak(RustStringRange::from_void(x_raw));
  let y = Box::leak(RustStringRange::from_void(y_raw));
  compare_with_wildcard(&x.as_slice(), &y.as_slice())
}
