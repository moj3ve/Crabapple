use objc_foundation::NSString;
#[cfg(feature = "arm64e")]
use std::os::raw::c_void;

#[link(name = "Foundation", kind = "framework")]
extern "C" {
	/// `NSLog` function, outputs to iOS OSLog.const
	/// `func NSLogv(_ format: String, _ args: CVaListPointer)`
	#[allow(improper_ctypes)]
	pub fn NSLogv(nsFormat: *const NSString); // format from inside rust or it dies
}
