rm -rfv output/

rustc src/lib.rs \
    --target aarch64-apple-ios \
    --crate-type staticlib \
    --out-dir output/ios_arm64

rustc src/lib.rs \
    --target aarch64-apple-ios-sim \
    --crate-type staticlib \
    --out-dir output/ios_arm64_sim

cbindgen --crate httpclient --output output/include/httpclient.h

xcodebuild -create-xcframework \
    -library output/ios_arm64/liblib.a -headers output/include \
    -library output/ios_arm64_sim/liblib.a -headers output/include \
    -output output/httpclient.xcframework

#rm -rfv ../rust_lib_app/external/httpclient.xcframework
#cp -rv output/httpclient.xcframework ../rust_lib_app/external/httpclient.xcframework
