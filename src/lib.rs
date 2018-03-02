//! This is a crate of handy modules and utilities for manipulating CAN data and interfaces.
//!
//! NOTE: this crate is not yet ready for prime time, but it will be soon, just publishing now to claim the name
//!
//! TODO: add real documentation here

#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]
// #![cfg_attr(not(feature = "std"), no_std)]

mod frames;

pub use frames::*;

/// A collection of [`byteorder`] types to facilitate unpacking values from frames.
pub mod byte_orders {
  extern crate byteorder;
  /// Defines the byte order for CANOpen frames, for later use decoding values.
  pub type CanOpenByteOrder = byteorder::LittleEndian;

  /// Defines the byte order for J1939 frames, for later use decoding values.
  pub type J1939ByteOrder = byteorder::LittleEndian;
}
