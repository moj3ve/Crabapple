use dlopen::wrapper::WrapperApi;
use objc::runtime::*;
use std::os::raw::c_void;
use std::ptr::NonNull;

/// Wrapper for the CydiaSubstrate library.
#[derive(WrapperApi)]
pub struct Substrate {
	/// `MSHookMessageEx` function from Cydia Substrate
	/// Hooks a selector within a class, pointing it to a function.
	/// `void MSHookMessageEx(Class _class, SEL message, IMP hook, IMP *old)`
	#[allow(non_snake_case)]
	MSHookMessageEx: unsafe extern "C" fn(
		class: *const Class,
		selector: Sel,
		replacement: *mut c_void,
		result: &mut Option<NonNull<Imp>>,
	),
}
