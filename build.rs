// Build script that (optionally) compiles protobufs from `src/shared/contracts/base.proto`
// using prostbuild. This script runs only if the repository root is a Rust crate that
// declares `build = "build.rs"` in its Cargo.toml.

use std::path::PathBuf;

fn main() {
    // Path to the shared base proto inside the repo
    let proto_path = PathBuf::from("src/shared/contracts/base.proto");
    println!("cargo:rerun-if-changed=src/shared/contracts/base.proto");

    if !proto_path.exists() {
        println!("cargo:warning=Project_B build.rs: base.proto not found at {:?}; skipping proto build", proto_path);
        return;
    }

    // Compile with prostbuild
    let include_dir = PathBuf::from("src/shared");
    let files = &[proto_path];
    match prost_build::compile_protos(files, &[include_dir]) {
        Ok(_) => {
            // By default prostbuild will generate a module file named after the protobuf package,
            // e.g., `Project_B.shared.rs`. The top-level lib.rs includes it behind the `with_protos` feature.
            println!("cargo:warning=Project_B build.rs: compiled protobufs successfully");
        }
        Err(e) => {
            // Emit a warning instead of panicking to avoid breaking non-proto builds
            println!("cargo:warning=Project_B build.rs: prostbuild failed: {e}");
        }
    }
}