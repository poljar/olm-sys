# `olm-sys`: A Low Level Binding For [Olm](https://git.matrix.org/git/olm/)

## Supported Platforms

- Linux
- macOS
- Windows
- FreeBSD
- WebAssembly

## Building

This library can either be built by statically or dynamically linking against `libolm`:

### Static

This is the default and requires no further action. `libolm` is built locally and then linked against statically.

#### Build dependencies

- `libstdc++`/`libc++`
- cmake
- GNU make or a compatible variant (WebAssembly only)
- Emscripten (WebAssembly only)

### Dynamic

For linking against `libolm` dynamically, first make sure that you have the library in your link path.
Then build this library with the `OLM_LINK_VARIANT` environment variable set to `dylib`.

For example, building your project using `olm-sys` as a dependency would look like this:
```
OLM_LINK_VARIANT=dylib cargo build
```

### Cross compiling for Android

To enable cross compilation for Android set the environment variable
`ANDROID_NDK` to the location of your NDK installation, for example:

```bash
$ ANRDOID_NDK=/home/user/Android/Sdk/ndk/22.0.7026061/
```

The linker needs to be set to an target specific one as well, for example for
`aarch64-linux-android` set this into your cargo config:

```
[target.aarch64-linux-android]
ar = "/home/user/Android/Sdk/ndk/22.0.7026061/toolchains/llvm/prebuilt/linux-x86_64/bin/ar"
linker = "/home/user/Android/Sdk/ndk/22.0.7026061/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android30-clang"
```

After both of these are set, compilation should work as usual using cargo:

```bash
$ ANDROID_NDK=~/Android/Sdk/ndk/22.0.7026061 cargo build --target aarch64-linux-android
```
