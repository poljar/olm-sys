# Contributing to olm-sys

### libolm API references
The header `olm.h` from which we generate the bindings used in this library is well documented. If you want to know what a function used from libolm does, it's best to look there.

### Bindgen
The C bindings to `libolm` are generated statically, and not during building of this crate, to reduce dependencies. The generated bindings can be found at `src/lib.rs`. When generating bindings for `libolm`, have a look at `generate_bindings.sh`, to understand the process.

### Contributing guidelines
Before filing a merge request, make sure of the following things:

* all unit tests pass
* your added code is well documented
* your code is formatted using `rustfmt`, for consistency

Alternatively you can submit patches via email. Send your patches and questions to `jhaye[at]mailbox.org`.
