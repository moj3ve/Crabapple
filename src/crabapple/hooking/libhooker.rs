use dlopen::wrapper::WrapperApi;
use objc::runtime::*;
use std::os::raw::{c_char, c_void};
use std::ptr::NonNull;

pub type LBHookMessage = unsafe extern "C" fn(
	class: *const Class,
	selector: Sel,
	replacement: *mut c_void,
	result: &mut Option<NonNull<Imp>>,
) -> LibhookerError;

/// Wrapper for the libblackjack library.
#[derive(WrapperApi)]
pub struct LibBlackjack {
	/// `LBHookMessage` function from libblackjack (part of libhooker)
	/// This provides an easy way to hook the method and call to the
	/// original method. This also provides a guarantee that hooking a class that doesn't implement a method won't overwrite the method
	/// in the super class.
	/// `enum LIBHOOKER_ERR LBHookMessage(Class objcClass, SEL selector, void *replacement, void *old_ptr);`
	LBHookMessage: unsafe extern "C" fn(
		class: *const Class,
		selector: Sel,
		replacement: *mut c_void,
		result: &mut Option<NonNull<Imp>>,
	) -> LibhookerError,
}

impl LibBlackjack {
	pub unsafe extern "C" fn LBHookMessage_NP(
		&self,
		class: *const Class,
		selector: Sel,
		replacement: *mut c_void,
		result: &mut Option<NonNull<Imp>>,
	) -> LibhookerError {
		let pacced: *mut c_void = std::mem::transmute(self.LBHookMessage);
		let nopac = crate::util::strip_pac(pacced);
		let finale: LBHookMessage = std::mem::transmute(nopac);
		(finale)(class, selector, replacement, result)
	}
}

pub type LHStrError = unsafe extern "C" fn(err: LibhookerError) -> *const c_char;

/// Wrapper for the libhooker library.
#[derive(WrapperApi)]
pub struct LibHooker {
	/// `LHStrError` function from libhooker
	/// Get a human readable string for debugging purposes.
	#[allow(non_snake_case)]
	LHStrError: unsafe extern "C" fn(err: LibhookerError) -> *const c_char,
}

impl LibHooker {
	pub unsafe extern "C" fn LHStrError_NP(&self, err: LibhookerError) -> *const c_char {
		let pacced: *mut c_void = std::mem::transmute(self.LHStrError);
		let nopac = crate::util::strip_pac(pacced);
		let finale: LHStrError = std::mem::transmute(nopac);
		(finale)(err)
	}
}

/// Describes an error returned by libhooker.
#[repr(C)]
pub enum LibhookerError {
	/// No errors took place.
	Ok,
	/// An Objective-C selector was not found. (This error is from libblackjack)
	ErrSelectorNotFound,
	/// A function was too short to hook.
	ShortFunc,
	/// A problematic instruction was found at the start. We can't preserve the original function due to this instruction getting clobbered.
	BadInstructionAtStart,
	/// An error took place while handling memory pages.
	VM,
	/// No symbol was specified for hooking.
	NoSymbol,
}
