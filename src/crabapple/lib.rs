#[macro_use]
extern crate dlopen_derive; // move your damn shit to Rust 2018

pub mod ffi;
pub mod hooking;
pub mod logging;
pub mod objc;
pub mod util;

/// Re-exported dependencies.
pub mod deps {
	pub use ::objc;
	pub use objc_foundation as foundation;
	pub use paste;
}

#[macro_export]
macro_rules! sel {
	($name:expr) => {
		$crate::deps::objc::sel_impl!(concat!($name, '\0'))
	};
}

/// Sets up Objective-C hooks, based on the contained module and functions.
///
/// # Example
/// ```
/// hook_it! {
///		mod dock_example {
///				imports {
///					use crabapple::deps::objc::runtime::*;
///					use std::os::raw::c_double;
///				}
///				#[hook(class = "SBDockView", sel = "setBackgroundAlpha:")]
///				fn setBackgroundAlpha(orig, this: &Object, cmd: Sel, alpha: c_double) [] {
///					orig(this, cmd, 0.0);
///				}
///			}
///		}
/// }
/// ```
#[macro_export]
macro_rules! hook_it {
	(mod $mod_name:ident {
		imports {
			$($prefix:item)*
		}
		$(
			#[hook(class = $class:expr, sel = $sel:expr)]
			fn $fn_name:ident($orig:ident, $($arg:ident: $ty_:ty),*) [$($ret:ty)?] $body:tt
		)*
	}) => {
		mod $mod_name {
			$($prefix)*
			$(
				$crate::deps::paste::item! {
					type [<$fn_name _fn>] = unsafe extern "C" fn($($arg: $ty_),*) $(-> $ret)*;
					type [<$fn_name _callfn>] = fn($($arg: $ty_),*) $(-> $ret)*;
					pub static [<$fn_name _orig>]: std::sync::atomic::AtomicPtr<$crate::deps::objc::runtime::Imp> = std::sync::atomic::AtomicPtr::new(std::ptr::null_mut());

					// Wrapper so unsafe isn't needed in the main hook.
					fn [<$fn_name _call>]($($arg: $ty_),*) $(-> $ret)* {
						unsafe {
							let [<$fn_name _ptr>]: *mut std::os::raw::c_void = [<$fn_name _orig>].load(std::sync::atomic::Ordering::Relaxed) as *mut _ as *mut std::os::raw::c_void;
							let [<$fn_name _nopac>] = $crate::util::strip_pac([<$fn_name _ptr>]);
							let orig: [<$fn_name _fn>] = std::mem::transmute([<$fn_name _nopac>]);
							orig($($arg),*)
						}
					}

					#[no_mangle]
					extern "C" fn $fn_name($($arg: $ty_),*) $(-> $ret)* {
						let $orig: [<$fn_name _callfn>] = [<$fn_name _call>];
						$body
					}
				}
			)*

			pub fn _INIT_HOOKS() {
				unsafe {
					$(
						$crate::deps::paste::expr! {
							let target_sel = $crate::sel!($sel);
							let [<$fn_name _ptr>] = $fn_name as [<$fn_name _fn>] as usize as *mut std::os::raw::c_void;
							$crate::logging::log(format!("Crabapple - Initializing class {}[{}] with hook {:#?}", $class, $sel, [<$fn_name _ptr>]));
							let trampoline = $crate::hooking::hook($class, target_sel, [<$fn_name _ptr>]);
							match trampoline {
								Some(t) => {
									let trampoline_ptr = t.as_ptr();
									[<$fn_name _orig>].store(trampoline_ptr, std::sync::atomic::Ordering::Relaxed);
									$crate::logging::log(format!("Crabapple - Hooked class {}[{}] with trampoline {:#?}", $class, $sel, trampoline_ptr));
								},
								_ => {
									$crate::logging::log(format!("Crabapple - Failed to hook class {}[{}]", $class, $sel));
								}
							}
						};
					)*
				}
			}
		}
	}
}

/// Initializes the hook modules passed to it, and sets up a ctor function which will
/// Also sets up a panic handler, which will output panic data to OSLog.
/// Can also have pre-hook and post-hook code.
#[macro_export]
macro_rules! init_hooks {
	($($hook_mod:ident),*) => {
		#[used]
		#[cfg_attr(target_os = "ios", link_section = "__DATA,__mod_init_func")]
		static LOAD: extern "C" fn() = {
			extern "C" fn ctor() {
				// Set up our panic hook, to ensure backtraces go to the oslog.
				std::panic::set_hook(Box::new(|panic_info| {
					if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
						$crate::logging::log(format!("[Crabapple] Caught panic!: {:#?}", s));
					} else {
						$crate::logging::log("[Crabapple] Caught panic!".to_string());
					}
					if let Some(location) = panic_info.location() {
						$crate::logging::log(format!("[Crabapple] Panic occurred in file '{}' at line {}",
							location.file(),
							location.line(),
						));
					}
				}));
				$(
					$hook_mod::_INIT_HOOKS();
				)*
			}
		ctor
		};
	};
	($pre:tt, $($hook_mod:ident),*) => {
		#[used]
		#[cfg_attr(target_os = "ios", link_section = "__DATA,__mod_init_func")]
		static LOAD: extern "C" fn() = {
			extern "C" fn ctor() {
				// Set up our panic hook, to ensure backtraces go to the oslog.
				std::panic::set_hook(Box::new(|panic_info| {
					if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
						$crate::logging::log(format!("[Crabapple] Caught panic!: {:#?}", s));
					} else {
						$crate::logging::log("[Crabapple] Caught panic!".to_string());
					}
					if let Some(location) = panic_info.location() {
						$crate::logging::log(format!("[Crabapple] Panic occurred in file '{}' at line {}",
							location.file(),
							location.line(),
						));
					}
				}));
				$pre
				$(
					$hook_mod::_INIT_HOOKS();
				)*
			}
		ctor
		};
	};
	($pre:tt, $post:tt, $($hook_mod:ident),*) => {
		#[used]
		#[cfg_attr(target_os = "ios", link_section = "__DATA,__mod_init_func")]
		static LOAD: extern "C" fn() = {
			extern "C" fn ctor() {
				// Set up our panic hook, to ensure backtraces go to the oslog.
				std::panic::set_hook(Box::new(|panic_info| {
					if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
						$crate::logging::log(format!("[Crabapple] Caught panic!: {:#?}", s));
					} else {
						$crate::logging::log("[Crabapple] Caught panic!".to_string());
					}
					if let Some(location) = panic_info.location() {
						$crate::logging::log(format!("[Crabapple] Panic occurred in file '{}' at line {}",
							location.file(),
							location.line(),
						));
					}
				}));
				$pre
				$(
					$hook_mod::_INIT_HOOKS();
				)*
				$post
			}
		ctor
		};
	}
}
