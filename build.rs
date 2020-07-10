use std::process::{Command, Stdio};
use std::{env, fs, path::PathBuf};

const OLM_LINK_VARIANT_ENV: &str = "OLM_LINK_VARIANT";
const DOCS_RS: &str = "DOCS_RS";

fn main() {
    let manifest_dir = match env::var_os("CARGO_MANIFEST_DIR") {
        Some(d) => d,
        None => panic!("Unable to read manifest dir"),
    };
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    let olm_link_variant = env::var(OLM_LINK_VARIANT_ENV).unwrap_or_else(|_| "static".to_string());
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
        if olm_link_variant == "static" {
            // path to olm source code
            let src = PathBuf::from(&manifest_dir).join("olm");
            // where we will put our built library for static linking
            let dst = PathBuf::from(&out_path).join("build");
            let _ = fs::create_dir(&dst);
            // path to our final libolm file
            let dst_file = dst.join("libolm.a");

            // building libolm as a static lib
            if !dst_file.exists() {
                run(Command::new("make").arg("static").current_dir(&src));
                let _ = fs::copy(&src.join("build/libolm.a"), &dst_file);
            }
            println!("cargo:rustc-link-search={}", dst.display());
        }

        // Rebuild if link variant changed
        println!("cargo:rerun-if-env-changed={}", OLM_LINK_VARIANT_ENV);

        // Link to olm library
        println!("cargo:rustc-link-lib={}=olm", olm_link_variant);

        // Olm needs libstdc++
        if cfg!(not(target_os = "macos")) {
            println!("cargo:rustc-link-lib=stdc++");
        }
    }
}

fn run(cmd: &mut Command) {
    assert!(cmd
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .unwrap()
        .success());
}
