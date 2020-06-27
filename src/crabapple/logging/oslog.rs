use crate::ffi::NSLogv;
use objc_foundation::{INSString, NSString};

/// Logs a string to the OSLog.
pub fn log(data: &str) {
	unsafe { NSLogv(&*NSString::from_str(data)) }
}
