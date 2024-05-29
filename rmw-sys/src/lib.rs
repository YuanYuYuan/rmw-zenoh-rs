// Suppress the warnings
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]
#![allow(clippy::all)]
#![allow(improper_ctypes_definitions)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/bindings.rs"));

use zenoh::Session;
// use std::sync::Arc;

#[repr(C)]
// #[derive(Debug, Clone)]
#[derive(Debug)]
pub struct rmw_context_impl_s {
    pub sess: Session,
    // pub sess: Arc<Session>,
}

impl Drop for rmw_context_impl_t {
    fn drop(&mut self) {
        panic!("rmw_context_impl_t is dropped!");
    }
}
