//! This is a crate of handy modules and utilities for manipulating CAN data and interfaces.
//!
//! NOTE: this crate is not yet ready for prime time, but it will be soon, just publishing now to claim the name
//!
//! TODO: add real documentation here

#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]

mod frames;

pub use frames::*;
