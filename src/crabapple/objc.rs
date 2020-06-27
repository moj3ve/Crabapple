use crate::util::*;
use ::objc::runtime::*;

/// Returns an Objective-C `Class` by it's name.
pub fn get_class(class: &str) -> *const Class {
	unsafe { objc_getClass(to_c_str(class)) }
}
