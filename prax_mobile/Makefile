# Adapted from https://ospfranco.com/post/2024/05/08/react-native-rust-module-guide/

ARCHS_IOS = x86_64-apple-ios aarch64-apple-ios aarch64-apple-ios-sim
ARCHS_ANDROID = aarch64-linux-android armv7-linux-androideabi x86_64-linux-android i686-linux-android
LIB = libprax_mobile.a
DYLIB = libprax_mobile.so
XCFRAMEWORK = prax_mobile.xcframework

all: ios android

ios: $(XCFRAMEWORK)

android: $(ARCHS_ANDROID)
	# After build is done copy files into the android folder
	mkdir -p ../android/app/src/main/jniLibs
	mkdir -p ../android/app/src/main/jniLibs/x86
	mkdir -p ../android/app/src/main/jniLibs/arm64-v8a
	mkdir -p ../android/app/src/main/jniLibs/armeabi-v7a
	mkdir -p ../android/app/src/main/jniLibs/x86_64

	cp ./target/i686-linux-android/release/$(DYLIB) ../android/app/src/main/jniLibs/x86/$(DYLIB)
	cp ./target/aarch64-linux-android/release/$(DYLIB) ../android/app/src/main/jniLibs/arm64-v8a/$(DYLIB)
	cp ./target/arm-linux-androideabi/release/$(DYLIB) ../android/app/src/main/jniLibs/armeabi-v7a/$(DYLIB)
	cp ./target/x86_64-linux-android/release/$(DYLIB) ../android/app/src/main/jniLibs/x86_64/$(DYLIB)

.PHONY: $(ARCHS_IOS)
$(ARCHS_IOS): %:
	cargo build --target $@ --release

.PHONY: $(ARCHS_ANDROID)
$(ARCHS_ANDROID): %:
	cargo ndk --target $@ --platform 31 --release

$(XCFRAMEWORK): $(ARCHS_IOS)
	mkdir -p simulator_fat
	lipo -create target/x86_64-apple-ios/release/$(LIB) target/aarch64-apple-ios-sim/release/$(LIB) -output simulator_fat/$(LIB)
	xcodebuild -create-xcframework -library target/aarch64-apple-ios/release/$(LIB) -headers generated/include -library simulator_fat/$(LIB) -headers generated/include -output $@
	cp -r $@ ../ios/$@

.PHONY: clean
clean:
	rm -rf simulator_fat
	rm -rf $(XCFRAMEWORK)
	rm -rf ../ios/$(XCFRAMEWORK)

.PHONY: all ios android clean