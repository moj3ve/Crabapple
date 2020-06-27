use anyhow::Result;
use lazy_static::*;
use std::net::{SocketAddr, ToSocketAddrs, UdpSocket};
use std::sync::RwLock;

lazy_static! {
	static ref TARGET: RwLock<Option<SocketAddr>> = RwLock::new(None);
}

/// Sets the RemoteLog server target. Should probably run this in `init_hooks!`
/// Examples:
/// ```
/// set_remote_target("192.168.0.195:1337");
/// ```
/// ```
/// set_remote_target(("192.168.0.3", 42069));
/// ```
pub fn set_remote_target<T: ToSocketAddrs>(addr: T) -> Result<()> {
	if let Ok(mut t) = TARGET.write() {
		if let Some(addr) = addr.to_socket_addrs()?.next() {
			*t = Some(addr);
		}
	}
	Ok(())
}

/// Sends the log message to the RemoteLog server target, if set.
/// Also outputs to OSLog.
pub fn log<T: ToString>(data: T) -> Result<()> {
	crate::logging::oslog::log(&data.to_string());
	if let Ok(t) = TARGET.read() {
		if let Some(addr) = *t {
			let socket = UdpSocket::bind("0.0.0.0:0")?;
			socket.send_to(data.to_string().as_bytes(), addr)?;
		}
	}
	Ok(())
}
