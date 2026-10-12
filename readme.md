# nu-engine

A small Rust native library that exposes arithmetic and time utilities to Java through JNI.

## Features

- Add and subtract two 32-bit integers.
- Get the current Unix timestamp in milliseconds.

The JNI entry points target static native methods on `io.rmrfy.nu.NuEngine`:

```java
public static native int add(int a, int b);
public static native int subtract(int a, int b);
public static native long currentTimeMillis();
```

## Build

Install Rust and Cargo with support for the Rust 2024 edition (Rust 1.85 or newer), then run:

```sh
cargo build --release
```

The shared library is generated in `target/release/` as `libnu_engine.so` on Linux, `libnu_engine.dylib` on macOS, or `nu_engine.dll` on Windows. Java applications can load it with `System.loadLibrary("nu_engine")` when its directory is on the Java library path.

## Test

```sh
cargo test
```

## Source layout

- `src/lib.rs`: library module declarations.
- `src/jni.rs`: JNI entry points.
- `src/math.rs`: arithmetic functions.
- `src/utils.rs`: timestamp utility and its unit test.
