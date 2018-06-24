#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
extern crate freetype_sys;

pub use freetype_sys::FT_Face;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
