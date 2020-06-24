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
			crabapple::objc::log(&format!("Crabapple dock_example | {:#?} - {:#?} - {:#?}", this, cmd, alpha));
			orig(this, cmd, 0.0);
		}
	}
}

// This crashes, so I'm just using this to test panic hooks.
hook_it! {
	mod notification_example {
		imports {
			use crabapple::deps::objc::runtime::*;
			use crabapple::deps::foundation::NSString;
		}
		#[hook(class = "BBServer", sel = "_publishBulletinRequest:forSectionID:forDestinations:")]
		fn publishBulletinRequest(orig, this: &Object,
			cmd: Sel,
			request: &Object,
			appid: &Object,
			arg3: u64)
		[] {
			let title: *const NSString = *request.get_ivar::<*mut Object>("title") as *mut NSString;
			let subtitle: *const NSString = *request.get_ivar::<*mut Object>("subtitle") as *mut NSString;
			let message: *const NSString = *request.get_ivar::<*mut Object>("message") as *mut NSString;
			crabapple::objc::log(&format!("Crabapple notification_example | {:?} - {:?} - {:?}", title, subtitle, message));
			orig(this, cmd, request, appid, arg3);
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
		fn displayName(_orig, _this: &Object, _cmd: Sel) [NSStr] {
			&*NSString::from_str("test")
		}
	}
}

init_hooks!(dock_example, notification_example, apps_example);
