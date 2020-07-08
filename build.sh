rm -rf out || true
mkdir out
cd src/crabapple-example
cargo +nightly build --target=aarch64-apple-ios --release
cd ../..
cp target/aarch64-apple-ios/debug/libcrabapple_example.dylib out/arm64.dylib
cd src/crabapple-example
cargo +nightly build --target=aarch64-apple-ios --features arm64e --release
cd ../..
cp target/aarch64-apple-ios/debug/libcrabapple_example.dylib out/_arm64e.dylib
res/arm64patcher out/_arm64e.dylib out/arm64e.dylib
lipo -create out/arm64e.dylib out/arm64.dylib -output out/CrabappleTest.dylib
ldid -S out/CrabappleTest.dylib
