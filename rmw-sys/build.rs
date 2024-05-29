use std::{env, fs, path::Path};

fn main() {
    r2r_common::print_cargo_watches();
    r2r_common::print_cargo_ros_distro();
    run_bindgen();
    run_dynlink()
}

fn run_dynlink() {
    r2r_common::print_cargo_link_search();
    println!("cargo:rustc-link-lib=dylib=rcl");
    println!("cargo:rustc-link-lib=dylib=rcl_logging_spdlog");
    println!("cargo:rustc-link-lib=dylib=rcl_yaml_param_parser");
    println!("cargo:rustc-link-lib=dylib=rcutils");
    println!("cargo:rustc-link-lib=dylib=rmw");
    println!("cargo:rustc-link-lib=dylib=rmw_implementation");
    println!("cargo:rustc-link-lib=dylib=rosidl_typesupport_c");
    println!("cargo:rustc-link-lib=dylib=rosidl_runtime_c");
}

fn run_bindgen() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let saved_file = manifest_dir.join("bindings.rs");
    gen_bindings(&saved_file);
}

fn gen_bindings(out_file: &Path) {
    if let Some(dir) = out_file.parent() {
        fs::create_dir_all(dir)
            .unwrap_or_else(|_| panic!("Unable to create directory '{}'", dir.display()));
    }

    let bindings = r2r_common::setup_bindgen_builder()
        .header("src/wrapper.h")
        .blocklist_type("rmw_context_impl_s")
        // .allowlist_type("rcl_.*")
        // .allowlist_type("rcutils_.*")
        // .allowlist_type("rmw_.*")
        // .allowlist_type("rosidl_.*")
        // .allowlist_type("RCUTILS_.*")
        // .allowlist_var("RCL_.*")
        // .allowlist_var("RCUTILS_.*")
        // .allowlist_var("RMW_.*")
        // .allowlist_var("rosidl_.*")
        // .allowlist_var("g_rcutils_.*")
        // .allowlist_function("rcl_.*")
        // .allowlist_function("rcutils_.*")
        // .allowlist_function("rmw_.*")
        // .allowlist_function("rosidl_.*")
        // .allowlist_function(".*_typesupport_.*")
        // .allowlist_function(".*_sequence_bound_.*")
        // .no_debug("_OSUnaligned.*")
        .derive_partialeq(true)
        .derive_copy(true)
        .generate_comments(false)
        .derive_default(true)
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_file)
        .expect("Couldn't write bindings!");
}
