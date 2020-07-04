#![allow(unused_must_use)]

use crabapple::deps::foundation::NSString;
use crabapple::{hook_it, init_hooks};
type NSStr = *const NSString;

hook_it! {
	mod dock_example {
		imports {
			use crabapple::deps::objc::runtime::*;
			use std::os::raw::c_double;
		}
		#[hook(class = "SBDockView", sel = "setBackgroundAlpha:")]
		fn setBackgroundAlpha(orig, this: &Object, cmd: Sel, alpha: c_double) [] {
			crabapple::logging::log(format!("Crabapple dock_example | {:#?} - {:#?} - {:#?}", this, cmd, alpha));
			orig(this, cmd, 0.0);
		}
	}
}

hook_it! {
	mod apps_example {
		imports {
			use crabapple::deps::objc::runtime::*;
			use crabapple::deps::foundation::{INSString, NSString};
			use crate::NSStr;
		}
		#[hook(class = "SBApplicationInfo", sel = "displayName")]
		fn displayName(orig, this: &Object, cmd: Sel) [NSStr] {
			let original_name = orig(this, cmd);
			crabapple::logging::log(format!("Crabapple Log - {}", crabapple::util::from_nsstr(original_name)));
			&*NSString::from_str("test")
		}
	}
}

init_hooks!(dock_example, apps_example);
