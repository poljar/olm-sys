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

extern crate fs_extra;
use fs_extra::dir::{copy, CopyOptions};

use std::path::Path;
use std::process::{Command, Stdio};
use std::{env, path::PathBuf};

const OLM_LINK_VARIANT_ENV: &str = "OLM_LINK_VARIANT";

fn main() {
    let olm_link_variant = env::var(OLM_LINK_VARIANT_ENV).unwrap_or_else(|_| "static".to_string());
    let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();

    let src_dir =
        PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").expect("Unable to find manifest dir"))
            .join("olm");
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").expect("Unable to find output dir"));
    let dest_dir = out_dir.join("olm");

    // Copy libolm source to build dir
    let mut options = CopyOptions::new();
    options.copy_inside = true;
    options.skip_exist = true;
    let _ = copy(&src_dir, &dest_dir, &options).expect("Failed to copy olm directory");

    if target_arch == "wasm32" {
        if olm_link_variant == "static" {
            wasm_build(&dest_dir);
        } else {
            panic!("WASM32 cannot be linked dynamicly");
        }
    } else {
        native_build(&dest_dir, olm_link_variant);
    }
    // Rebuild if link variant changed
    println!("cargo:rerun-if-env-changed={}", OLM_LINK_VARIANT_ENV);
}

fn native_build<P: AsRef<Path>>(src: P, olm_link_variant: String) {
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();

    // building libolm as a static lib
    let mut cmake = cmake::Config::new(src);
    cmake.define("BUILD_SHARED_LIBS", "NO");
    // disable tests for libolm
    cmake.define("OLM_TESTS", "OFF");

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

    if target_os == "ios" {
        if let Ok(sdk) = std::env::var("IOS_SDK_PATH") {
            let sdk_path = PathBuf::from(sdk);
            cmake.define("CMAKE_SYSTEM_NAME", "iOS");
            cmake.define("CMAKE_XCODE_ATTRIBUTE_CODE_SIGNING_ALLOWED", "NO");
            cmake.define("CMAKE_OSX_SYSROOT", sdk_path);

            let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
            if target_arch.as_str() == "x86_64" {
                cmake.build_arg("-sdk");
                cmake.build_arg("iphonesimulator");
            }

            let osx_architectures = match target_arch.as_str() {
                "aarch64" => "arm64",
                "x86_64" => "x86_64",
                _ => panic!(
                    "Unsupported target arch {} given, if this is an error please report a bug",
                    target_arch
                ),
            };
            cmake.define("CMAKE_OSX_ARCHITECTURES", osx_architectures);

            cmake.generator("Xcode");
        } else {
            panic!(
                "please set the IOS_SDK_PATH environment variable: $ export IOS_SDK_PATH=`xcrun --show-sdk-path --sdk iphoneos`"
            );
        }
    }

    let dst = cmake.build();

    // See https://gitlab.gnome.org/BrainBlasted/olm-sys/-/issues/6 for details why this is required
    if Path::new(&format!("{}/lib64", dst.display())).exists() {
        println!("cargo:rustc-link-search=native={}/lib64", dst.display());
    } else {
        println!("cargo:rustc-link-search=native={}/lib", dst.display());
    }

    println!("cargo:rustc-link-lib={}=olm", olm_link_variant);

    if target_os == "linux" || target_os == "android" || target_os == "illumos" {
        println!("cargo:rustc-link-lib=stdc++");
    }
    if target_os == "freebsd" || target_os == "macos" || target_os == "ios" {
        println!("cargo:rustc-link-lib=c++");
    }
}

fn wasm_build<P: AsRef<Path>>(src: P) {
    let lib_search_path = src.as_ref().join("build/wasm/");

    // building libolm as a static lib
    run(Command::new("make").arg("wasm").current_dir(src));
    println!("cargo:rustc-link-search={}", lib_search_path.display());
    println!("cargo:rustc-link-lib=static=olm");
}

fn run(cmd: &mut Command) {
    assert!(cmd
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .unwrap()
        .success());
}
