//! Support for reading and writing `docker-compose.yml` files.

#![warn(missing_docs)]

// Compiler plugins only work with Rust nightly builds, not with stable
// compilers.  We want to work with both.
#![cfg_attr(feature = "serde_macros", feature(plugin, custom_derive))]
#![cfg_attr(feature = "serde_macros", plugin(serde_macros))]

extern crate serde;
extern crate serde_yaml;

use std::collections::HashMap;

// This code is run if we have a nightly build of Rust, and hence compiler
// plugins.
#[cfg(feature = "serde_macros")]
include!("serde_types.in.rs");

// This code is run if we have a stable build of Rust.  `serde_types.rs` is
// generated from `serde_types.in.rs` by `build.rs` at build time.
#[cfg(feature = "serde_codegen")]
include!(concat!(env!("OUT_DIR"), "/serde_types.rs"));
