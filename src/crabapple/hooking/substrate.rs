use dlopen::wrapper::WrapperApi;
use objc::runtime::*;
use std::os::raw::c_void;
use std::ptr::NonNull;

pub type MSHookMessageEx = unsafe extern "C" fn(
	class: *const Class,
	selector: Sel,
	replacement: *mut c_void,
	result: &mut Option<NonNull<Imp>>,
);

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

impl Substrate {
	pub unsafe fn MSHookMessageEx_NP(
		&self,
		class: *const Class,
		selector: Sel,
		replacement: *mut c_void,
		result: &mut Option<NonNull<Imp>>,
	) {
		let pacced: *mut c_void = std::mem::transmute(self.MSHookMessageEx);
		let nopac = crate::util::strip_pac(pacced);
		let finale: MSHookMessageEx = std::mem::transmute(nopac);
		(finale)(class, selector, replacement, result)
	}
}
