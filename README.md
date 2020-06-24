# Crabapple framework

[![GitHub issues](https://img.shields.io/github/issues/Crabapple-iOS/crabapple?style=for-the-badge)](https://github.com/Crabapple-iOS/crabapple/issues)
[![GitHub pull requests](https://img.shields.io/github/issues-pr/Crabapple-iOS/crabapple?style=for-the-badge)](https://github.com/Crabapple-iOS/crabapple/pulls)
[![GitHub Workflow Status](https://img.shields.io/github/workflow/status/Crabapple-iOS/crabapple/Lint%20%26%20Compile?style=for-the-badge)](https://github.com/Crabapple-iOS/crabapple/actions)
[![GitHub](https://img.shields.io/github/license/Crabapple-iOS/crabapple?style=for-the-badge)](https://github.com/Crabapple-iOS/crabapple/blob/master/LICENSE)
[![Discord](https://img.shields.io/discord/725210900260454471?style=for-the-badge)](https://discord.gg/QHkCkRP)

[![forthebadge](https://forthebadge.com/images/badges/made-with-crayons.svg)](https://forthebadge.com)

The Crabapple framework is a library for building iOS tweaks in the Rust programming language. Normally this is done in Objective-C, or perhaps Swift, but now it is possible in Rust!

**This is very experimental. Use at your own risk. Please do report any issues, though.**

If you have any questions, join our [official Discord server](https://discord.gg/QHkCkRP)!
## Requirements
 - [macOS](https://www.apple.com/macos) with [Xcode](https://developer.apple.com/xcode/)
 - Nightly [Rust](https://www.rust-lang.org/) (must have [rust#73516](https://github.com/rust-lang/rust/pull/73516) merged)
 - [DragonBuild](https://github.com/DragonBuild/DragonBuild) or [Theos](https://github.com/theos/theos)
   - The `DRAGONBUILD`/`THEOS` environmental variable must be set.
   - You may also point to the iOS SDK and CydiaSubstrate.framework manually.

## Examples

Multiple examples can be found in the [crabapple-example crate](src/crabapple-example/lib.rs).

### Example tweak that makes the home screen dock invisible
```rs
use crabapple::{hook_it, init_hooks};

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

init_hooks!(dock_example);
```

## Contributing

Contributions are very much welcome and encouraged! Feel free to join us on our [official Discord server](https://discord.gg/QHkCkRP) if you have any questions.

## License

All Crabapple projects are licensed under the [Zlib license](https://tldrlegal.com/license/zlib-libpng-license-(zlib)) unless otherwise specified.

>Copyright (c) 2020, aspen/luxxxy
>
>This software is provided 'as-is', without any express or implied
>warranty. In no event will the authors be held liable for any damages
>arising from the use of this software.
>
>Permission is granted to anyone to use this software for any purpose,
>including commercial applications, and to alter it and redistribute it
>freely, subject to the following restrictions:
>
>1. The origin of this software must not be misrepresented; you must not
>   claim that you wrote the original software. If you use this software
>   in a product, an acknowledgment in the product documentation would be
>   appreciated but is not required.
>2. Altered source versions must be plainly marked as such, and must not be
>   misrepresented as being the original software.
>3. This notice may not be removed or altered from any source distribution.
