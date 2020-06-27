#![allow(non_snake_case)]

use dlopen::wrapper::Container;
use objc::runtime::*;
use objc::*;
use once_cell::sync::Lazy;
use std::os::raw::c_void;
use std::ptr::NonNull;

#[cfg(feature = "libhooker")]
pub mod libhooker;
#[cfg(feature = "substrate")]
pub mod substrate;

#[cfg(feature = "libhooker")]
use libhooker::{LibBlackjack, LibHooker, LibhookerError};

#[cfg(feature = "substrate")]
use substrate::Substrate;

#[cfg(feature = "libhooker")]
static LIBBLACKJACK: Lazy<Option<Container<LibBlackjack>>> = Lazy::new(|| {
	let libblackjack: Result<Container<LibBlackjack>, dlopen::Error> =
		unsafe { Container::load("/usr/lib/libblackjack.dylib") };
	match libblackjack {
		Ok(new_inst) => Some(new_inst),
		Err(_) => None,
	}
});
#[cfg(feature = "libhooker")]
static LIBHOOKER: Lazy<Option<Container<LibHooker>>> = Lazy::new(|| {
	let libhooker: Result<Container<LibHooker>, dlopen::Error> =
		unsafe { Container::load("/usr/lib/libhooker.dylib") };
	match libhooker {
		Ok(new_inst) => Some(new_inst),
		Err(_) => None,
	}
});
#[cfg(feature = "substrate")]
static SUBSTRATE: Lazy<Option<Container<Substrate>>> = Lazy::new(|| {
	let substrate: Result<Container<Substrate>, dlopen::Error> =
		unsafe { Container::load("/usr/lib/libsubstrate.dylib") };
	match substrate {
		Ok(new_inst) => Some(new_inst),
		Err(_) => None,
	}
});

#[allow(unreachable_code, unused_variables)]
pub fn hook(class_name: &str, selector: &str, replacement: *mut c_void) -> Option<NonNull<Imp>> {
	let mut trampoline: Option<NonNull<Imp>> = None;
	let class = crate::objc::get_class(class_name);
	let sel = sel![selector];

	#[cfg(feature = "libhooker")]
	{
		match &*LIBHOOKER {
			Some(_libhooker) => {
				match &*LIBBLACKJACK {
					Some(libblackjack) => {
						let ret = unsafe {
							libblackjack.LBHookMessage(class, sel, replacement, &mut trampoline)
						};
						return match ret {
							LibhookerError::Ok => trampoline,
							LibhookerError::ErrSelectorNotFound => {
								crate::logging::log(format!("Crabapple - libblackjack errored: An Objective-C selector [{}] was not found", selector));
								None
							}
							LibhookerError::ShortFunc => {
								crate::logging::log("Crabapple - libhooker errored: A function was too short to hook.".to_string());
								None
							}
							LibhookerError::BadInstructionAtStart => {
								crate::logging::log("Crabapple - libhooker errored: A problematic instruction was found at the start. We can't preserve the original function due to this instruction getting clobbered.".to_string());
								None
							}
							LibhookerError::VM => {
								crate::logging::log("Crabapple - libhooker errored: An error took place while handling memory pages.".to_string());
								None
							}
							LibhookerError::NoSymbol => {
								crate::logging::log("Crabapple - libhooker errored: No symbol was specified for hooking.".to_string());
								None
							}
						};
					}
					None => {}
				}
			}
			None => {}
		}
	}
	#[cfg(feature = "substrate")]
	{
		return match &*SUBSTRATE {
			Some(substrate) => {
				unsafe { substrate.MSHookMessageEx(class, sel, replacement, &mut trampoline) };
				trampoline
			}
			None => None,
		};
	}
	None
}
