extern crate pkg_config;
extern crate bindgen;
extern crate cc;

use std::env;
use std::path::PathBuf;

fn main() {
    let args = pkg_config::probe_library("raqm").expect("Unable to find libraqm");
    let args = args.include_paths.iter().map(|p| format!("-I{}", p.to_str().expect("Error getting pkg_config include paths")));

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        // Pass freetype2 include path
        .clang_args(args)
        /* Whitelisting raqm stuff */
        // Whitelist functions
        .whitelist_function("raqm_create")
        .whitelist_function("raqm_destroy")
        .whitelist_function("raqm_reference")
        .whitelist_function("raqm_set_language")
        .whitelist_function("raqm_set_par_direction")
        .whitelist_function("raqm_set_text")
        .whitelist_function("raqm_set_text_utf8")
        .whitelist_function("raqm_set_freetype_face")
        .whitelist_function("raqm_set_freetype_face_range")
        .whitelist_function("raqm_set_freetype_load_flags")
        .whitelist_function("raqm_add_font_feature")
        .whitelist_function("raqm_layout")
        .whitelist_function("raqm_get_glyphs")
        .whitelist_function("raqm_index_to_position")
        .whitelist_function("raqm_position_to_index")
        // Whitelist types
        .whitelist_type("raqm_t")
        .whitelist_type("raqm_direction_t")
        .whitelist_type("raqm_glyph_t")
        .whitelist_type("_raqm")
        /* End of whitelist */
        // Disable recursive whitelisting
        .whitelist_recursively(false)
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
