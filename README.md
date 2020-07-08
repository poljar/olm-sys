# `olm-sys`: A Low Level Binding For [Olm](https://git.matrix.org/git/olm/)

This library can either be built by statically or dynamically linking against `libolm`:

## Static

This is the default and requires no further action. `libolm` is built locally and then linked against statically.

### Build dependencies

- `libstdc++`
- GNU make or a compatible variant
- GCC

## Dynamic

For linking against `libolm` dynamically, first make sure that you have the library in your link path.
Then build this library with the `OLM_LINK_VARIANT` environment variable set to `dylib`.

For example, building your project using `olm-sys` as a dependency would look like this:
```
OLM_LINK_VARIANT=dylib cargo build
```