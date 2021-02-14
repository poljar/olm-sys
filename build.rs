// Copyright 2020 Johannes Hayeß
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::path::Path;
use std::process::{Command, Stdio};
use std::{env, fs, path::PathBuf};

const DOCS_RS: &str = "DOCS_RS";
const OLM_LINK_VARIANT_ENV: &str = "OLM_LINK_VARIANT";

fn main() {
    let olm_link_variant = env::var(OLM_LINK_VARIANT_ENV).unwrap_or_else(|_| "static".to_string());
    let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
    // When building on docs.rs, this environment variable is set, and since write access outside
    // of the build dir is disabled, building locally is not an option.
    // The only thing that we want however is documentation for things defined in lib.rs, so
    // we can simply skip everything that this file does.
    // More information: https://docs.rs/about
    let docs_rs = match env::var(DOCS_RS) {
        Ok(x) => x == "1",
        _ => false,
    };

    // Skip building and/or linking of libolm for docs.rs.
    if !docs_rs {
        if target_arch == "wasm32" {
            if olm_link_variant == "static" {
                wasm_build(olm_link_variant);
            } else {
                panic!("WASM32 cannot be linked dynamicly");
            }
        } else {
            native_build(olm_link_variant);
        }
        // Rebuild if link variant changed
        println!("cargo:rerun-if-env-changed={}", OLM_LINK_VARIANT_ENV);
    }
}

fn native_build(olm_link_variant: String) {
    let manifest_dir = match env::var_os("CARGO_MANIFEST_DIR") {
        Some(d) => d,
        None => panic!("Unable to read manifest dir"),
    };
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();

    // path to olm source code
    let src = PathBuf::from(&manifest_dir).join("olm");

    // building libolm as a static lib
    let mut cmake = cmake::Config::new(src);
    cmake.define("BUILD_SHARED_LIBS", "NO");

    if target_os == "android" {
        if let Ok(ndk) = std::env::var("ANDROID_NDK") {
            let ndk_root = PathBuf::from(ndk);
            let toolchain_path = PathBuf::from("build/cmake/android.toolchain.cmake");
            let toolchain_file = ndk_root.join(toolchain_path);

            cmake
                .define("CMAKE_SYSTEM_NAME", "Android")
                .define("CMAKE_SYSTEM_VERSION", "30")
                .define("CMAKE_ANDROID_NDK", ndk_root)
                .define("CMAKE_TOOLCHAIN_FILE", toolchain_file);
        } else {
            panic!(
                "please set the ANDROID_NDK environment variable to your Android NDK instalation"
            );
        }

        let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();

        let abi = match target_arch.as_str() {
            "aarch64" => "arm64-v8a",
            "arm" => "armeabi-v7a",
            "x86_64" => "x86_64",
            "x86" => "x86",
            _ => panic!(
                "Unsupported target arch {} given, if this is an error please report a bug",
                target_arch
            ),
        };

        cmake.define("ANDROID_ABI", abi);
    }

    let dst = cmake.build();

    // See https://gitlab.gnome.org/BrainBlasted/olm-sys/-/issues/6 for details why this is required
    if Path::new(&format!("{}/lib64", dst.display())).exists() {
        println!("cargo:rustc-link-search=native={}/lib64", dst.display());
    } else {
        println!("cargo:rustc-link-search=native={}/lib", dst.display());
    }

    println!("cargo:rustc-link-lib={}=olm", olm_link_variant);

    if target_os == "linux" || target_os == "android" {
        println!("cargo:rustc-link-lib=stdc++");
    }
    if target_os == "freebsd" || target_os == "macos" {
        println!("cargo:rustc-link-lib=c++");
    }
}

fn wasm_build(olm_link_variant: String) {
    let manifest_dir = match env::var_os("CARGO_MANIFEST_DIR") {
        Some(d) => d,
        None => panic!("Unable to read manifest dir"),
    };

    let src_file = "build/wasm/libolm.a";

    // path to olm source code
    let src = PathBuf::from(&manifest_dir).join("olm");
    // where we will put our built library for static linking
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let dst = PathBuf::from(&out_path).join("build");
    let _ = fs::create_dir(&dst);
    // path to our final libolm file
    let dst_file = dst.join("libolm.a");

    // building libolm as a static lib
    if !dst_file.exists() {
        run(Command::new("make").arg("wasm").current_dir(&src));
        let _ = fs::copy(&src.join(src_file), &dst_file);
    }
    println!("cargo:rustc-link-search={}", dst.display());
    println!("cargo:rustc-link-lib={}=olm", olm_link_variant);
}

fn run(cmd: &mut Command) {
    assert!(cmd
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .unwrap()
        .success());
}
