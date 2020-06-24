use objc::runtime::Object;
use objc::*;
use std::ffi::CStr;
use std::os::raw::{c_char, c_void};

/// Strips the Pointer Authentication signature from a pointer. Only useful on arm64e (ARMv8.3+).
#[inline(always)]
#[cfg(not(feature = "arm64e"))]
pub fn strip_pac(pointer: *mut c_void) -> *mut c_void {
	pointer
}

/// Strips the Pointer Authentication signature from a pointer. Only useful on arm64e (ARMv8.3+).
#[inline(always)]
#[cfg(feature = "arm64e")]
pub fn strip_pac(pointer: *mut c_void) -> *mut c_void {
	unsafe { crate::ffi::ptr_strip(pointer) }
}

/// Converts a Rust `&str` to a C `const char*`.
#[inline(always)]
pub fn to_c_str(s: &str) -> *const c_char {
	let mut bytes = String::from(s).into_bytes();
	bytes.push(0);
	let ptr = bytes.as_ptr();
	std::mem::forget(bytes);
	unsafe { std::ffi::CStr::from_ptr(ptr as *const c_string::c_char).as_ptr() }
}

/// Converts a Rust `&str` to an Objective-C `*NSString`.
/// Returns a pointer to the `NSString`.
#[inline(always)]
pub fn to_nsstr(s: &str) -> *const c_void {
	unsafe { crate::ffi::OBJC_NSString(to_c_str(s)) }
}

/// Converts an Objective-C `*NSString` to a Rust `String`.
#[inline(always)]
pub fn from_nsstr(s: &Object) -> String {
	let nschar: *mut std::os::raw::c_char = unsafe { msg_send![s, UTF8String] };
	let c_str: &CStr = unsafe { CStr::from_ptr(nschar) };
	match c_str.to_str() {
		Ok(e) => e.to_string(),
		Err(_) => "".to_string(),
	}
}
