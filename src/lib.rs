#![deny(missing_docs)]
#![warn(clippy::unwrap_used)]

//! `libproc` is a library for getting information about running processes on Mac and Linux.
//!
//! Not all methods are available on both Operating Systems yet, but more will
//! be made cross-platform over time.
//!
extern crate libc;
extern crate errno;

pub mod libproc;
/// List processes by type and / or by path
pub mod processes;

#[cfg(target_os = "macos")]
#[allow(warnings, missing_docs)]
mod osx_libproc_bindings {
include!(concat!(env!("OUT_DIR"), "/osx_libproc_bindings.rs"));
}
