use std::env;
use std::path::PathBuf;

fn main() {
	if let Ok(dragonbuild) = env::var("DRAGONBUILD") {
		let dragonbuild_frameworks: PathBuf = [dragonbuild.to_owned(), "frameworks".to_string()]
			.iter()
			.collect();
		println!(
			"cargo:rustc-link-search=framework={}",
			dragonbuild_frameworks.display()
		);
	} else if let Ok(theos) = env::var("THEOS") {
		let theos_frameworks: PathBuf = [theos.to_owned(), "vendor".to_string(), "lib".to_string()]
			.iter()
			.collect();
		println!(
			"cargo:rustc-link-search=framework={}",
			theos_frameworks.display()
		);
	}
	println!("cargo:rustc-link-search=framework=.");
}
