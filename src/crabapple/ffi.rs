use ::objc::runtime::*;
use objc_foundation::NSString;
use std::os::raw::{c_char, c_void};
use std::ptr::NonNull;

#[link(name = "CydiaSubstrate", kind = "framework")]
extern "C" {
	pub fn OBJC_NSString(str: *const c_char) -> *mut c_void;
	pub fn OBJC_NSLog(str: *const c_char);
	pub fn NSLogv(nsFormat: *const NSString); // format from inside rust or it dies
	pub fn MSHookMessageEx(
		class: *const Class,
		selector: Sel,
		replacement: *mut c_void,
		result: &mut Option<NonNull<Imp>>,
	);
}
#[cfg(feature = "arm64e")]
#[link(name = "rustsupport_arm64e", kind = "static")]
extern "C" {
	pub fn ptr_strip(address: *mut c_void) -> *mut c_void;
}
