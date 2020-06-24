use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
	let mut sdk_path = String::new();
	if let Ok(dragonbuild) = env::var("DRAGONBUILD") {
		let dragonbuild_frameworks: PathBuf = [dragonbuild.to_owned(), "frameworks".to_string()]
			.iter()
			.collect();
		println!(
			"cargo:rustc-link-search=framework={}",
			dragonbuild_frameworks.display()
		);
		let dragonbuild_sdks: PathBuf =
			[dragonbuild, "sdks".to_string(), "iPhoneOS.sdk".to_string()]
				.iter()
				.collect();
		sdk_path = dragonbuild_sdks.into_os_string().into_string().unwrap();
	} else if let Ok(theos) = env::var("THEOS") {
		let theos_frameworks: PathBuf = [theos.to_owned(), "vendor".to_string(), "lib".to_string()]
			.iter()
			.collect();
		println!(
			"cargo:rustc-link-search=framework={}",
			theos_frameworks.display()
		);
		let theos_sdks: PathBuf = [theos, "sdks".to_string(), "iPhoneOS.sdk".to_string()]
			.iter()
			.collect();
		sdk_path = theos_sdks.into_os_string().into_string().unwrap();
	}

	// we only care if this exists or not, don't bother unwrapping it
	if env::var("CARGO_FEATURE_ARM64E").is_ok() {
		let crabapple_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
		let librustsupport: PathBuf = [
			crabapple_dir,
			"..".to_string(),
			"..".to_string(),
			"librustsupport".to_string(),
			"library.c".to_string(),
		]
		.iter()
		.collect();
		cc::Build::new()
			.file(librustsupport.as_path())
			.compiler("clang")
			.no_default_flags(true)
			.static_flag(true)
			.warnings(false)
			.flag("-isysroot")
			.flag(&sdk_path)
			.flag("-target")
			.flag("arm64e-apple-ios")
			.flag("-arch")
			.flag("arm64e")
			.compile("rustsupport_arm64e");
		let afile: PathBuf = [
			env::var("OUT_DIR").unwrap(),
			"librustsupport_arm64e.a".to_string(),
		]
		.iter()
		.collect();

		// Now, time for the big hack.
		// Forgive me for this.
		if let Ok(mut bytes) = fs::read(&afile) {
			let to_find: &[u8] = &[0xCF, 0xFA, 0xED, 0xFE, 0x0C, 0x00, 0x00, 0x01, 0x02]; // MACH-O magic + CPUTYPE_ARM_ANY + CPUSUBTYPE_ARM64E
			let headerstart = bytes
				.windows(to_find.len())
				.position(|w| w == to_find)
				.unwrap_or(0xC8);
			bytes[headerstart + (to_find.len() - 1)] = 0x00;
			fs::write(&afile, &bytes).expect("Errored writing librustsupport file!");
		}
	}
	println!("cargo:rustc-link-search=framework=.");
}
