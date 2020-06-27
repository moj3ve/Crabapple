pub mod oslog;
#[cfg(feature = "remotelog")]
pub mod remote;

/// The default log function.
/// If the `remotelog` feature is enabled, crabapple::logging::remote::log will be used, otherwise crabapple::logging::oslog::log will be used.
/// Example:
/// ```
/// crabapple::logging::log(format!("Hello {}, testing {}", "World!", 123));
/// ```
#[cfg(feature = "remotelog")]
pub fn log<T: ToString>(data: T) {
	if let Err(e) = remote::log(data) {
		oslog::log(&format!("[Crabapple] remotelog errored: {:?}", e));
	}
}

/// The default log function.
/// If the `remotelog` feature is enabled, crabapple::logging::remote::log will be used, otherwise crabapple::logging::oslog::log will be used.
/// Example:
/// ```
/// crabapple::logging::log(format!("Hello {}, testing {}", "World!", 123));
/// ```
#[cfg(not(feature = "remotelog"))]
pub fn log<T: ToString>(data: T) {
	oslog::log(&data.to_string());
}
