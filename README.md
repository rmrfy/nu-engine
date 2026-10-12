# nu-engine

A Rust native library that exposes integer arithmetic to Java through JNI.

## Requirements

- Rust and Cargo with support for the Rust 2024 edition (Rust 1.85 or later).

## Build

```sh
cargo build --release
```

The shared library is generated in `target/release`: `libnu_engine.so` on
Linux, `libnu_engine.dylib` on macOS, or `nu_engine.dll` on Windows.

## Java integration

The JNI exports correspond to static native methods on `io.rmrfy.nu.NuEngine`:

```java
package io.rmrfy.nu;

public class NuEngine {
    static {
        System.loadLibrary("nu_engine");
    }

    public static native int add(int a, int b);
    public static native int subtract(int a, int b);
}
```

Set `java.library.path` to the directory containing the built library when
running your Java application, for example `-Djava.library.path=target/release`.
The native library must be built for the platform and architecture of the JVM.

`add(a, b)` returns the sum; `subtract(a, b)` returns the difference. Both
operate on 32-bit signed integers. Keep inputs within the range where the
result fits in a Java `int`.

## Checks

```sh
cargo check
cargo test
```
