# nu-engine

A Rust native library with JNI bindings for Android. The current API provides
`add(int, int)` and `subtract(int, int)`, both returning an `int`.

## Android support

This repository builds `libnu_engine.so`; an Android app is needed to package and
call it. No Android app or Gradle project is included.

### Prerequisites

- Rust with Cargo (Rust 1.85 or newer for the Rust 2024 edition).
- Android SDK and NDK, installable through Android Studio's SDK Manager.
- [cargo-ndk](https://github.com/bbqsrc/cargo-ndk) to configure the NDK linker and
  produce Android's native library directory layout.

Install the build helper and Rust targets for ARM64 devices and x86-64 emulators:

```sh
cargo install cargo-ndk
rustup target add aarch64-linux-android x86_64-linux-android
```

`cargo-ndk` detects an NDK installed in Android Studio's default location. For a
custom installation, set `ANDROID_NDK_HOME` to the NDK directory.

### Build and package

Run from this repository's root:

```sh
cargo ndk -t arm64-v8a -t x86_64 -o android-libs build --release
```

The generated libraries have this layout:

```text
android-libs/
  arm64-v8a/libnu_engine.so
  x86_64/libnu_engine.so
```

Copy the ABI directories into your Android app module's `src/main/jniLibs/`
directory (typically `app/src/main/jniLibs/`). Gradle packages these libraries
with the app. Include the ABI used by your device or emulator; if the app uses
`abiFilters`, ensure they include the packaged ABIs.

For 32-bit ARM, also install `armv7-linux-androideabi` with `rustup target add`
and add `-t armeabi-v7a` to the build command.

### Call from Android

Create `NuEngine.java` in your app's Java source directory under `io/rmrfy/nu/`:

```java
package io.rmrfy.nu;

public final class NuEngine {
    static {
        System.loadLibrary("nu_engine");
    }

    private NuEngine() {}

    public static native int add(int a, int b);
    public static native int subtract(int a, int b);
}
```

Use it from Java or Kotlin:

```java
int sum = NuEngine.add(2, 3);        // 5
int difference = NuEngine.subtract(7, 4); // 3
```

Import `io.rmrfy.nu.NuEngine` when calling from another package. The package,
class, and native method names must match the exported symbols in
[`src/jni.rs`](src/jni.rs). Load `nu_engine` without the `lib` prefix or `.so`
extension.

If you enable R8/ProGuard, preserve the JNI class and method names in your app's
ProGuard rules:

```proguard
-keep class io.rmrfy.nu.NuEngine { *; }
```

For `UnsatisfiedLinkError`, check that the library for the device's ABI is
packaged, that it is loaded before calling a native method, and that the Java
names match the JNI exports. See the Android documentation for
[JNI integration](https://developer.android.com/ndk/guides/jni-tips).
