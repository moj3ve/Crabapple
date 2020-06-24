use ::objc::runtime::*;
use objc_foundation::NSString;
use std::os::raw::c_void;
use std::ptr::NonNull;

#[link(name = "CydiaSubstrate", kind = "framework")]
extern "C" {
	/// `NSLog` function, outputs to iOS OSLog.const
	/// `func NSLogv(_ format: String, _ args: CVaListPointer)`
	#[allow(improper_ctypes)]
	pub fn NSLogv(nsFormat: *const NSString); // format from inside rust or it dies

	/// `MSHookMessageEx` function from Cydia Substrate
	/// Hooks a selector within a class, pointing it to a function.
	/// `void MSHookMessageEx(Class _class, SEL message, IMP hook, IMP *old)`
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
	/// Strips pointer authentication from a C pointer on arm64e (ARMv8.3+)
	/// Does nothing on arm64 (ARMv8)
	/// `void* ptr_strip(void* address)`
	pub fn ptr_strip(address: *mut c_void) -> *mut c_void;
}
