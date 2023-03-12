rm -rfv output/

cargo build --target aarch64-apple-ios --out-dir output/aarch64-apple-ios -Z unstable-options
cargo build --target aarch64-apple-ios-sim --out-dir output/aarch64-apple-ios-sim -Z unstable-options

cbindgen --crate httpclient --output output/include/httpclient.h

xcodebuild -create-xcframework \
    -library output/aarch64-apple-ios/libhttpclient.a -headers output/include \
    -library output/aarch64-apple-ios-sim/libhttpclient.a -headers output/include \
    -output output/httpclient.xcframework

rm -rfv ../test_app/external/httpclient.xcframework
cp -rv output/httpclient.xcframework ../test_app/external/httpclient.xcframework
