//! The [`frames`] module houses various CAN frame representations.
//!
//! Regular CAN is a very low level technology (OSI layers 1 and 2), and as such there's a need for
//! data representations that don't impose usage patterns on the user (since hardware needs vary).
//! But at the same time, one of Rusts great strengths is its type system and most Rustaceans
//! prefer more type level sanity checks.  This module contains frame representations that cater
//! to both use cases, as well as easy / cheap layers to convert between them.

/// A standard representation of the frames that might be sent and received on a CAN bus.
///
/// This struct can represent any CAN frame, as described in the CAN specification
/// version 2.0, published September 1991 by Bosch GmbH.  They can be used for either
/// transmission or reception.
pub struct CanFrame {
  /// This contains either the Base Identifier or the Extended Identifier, depending on `ide`.
  pub id: u32,
  /// Number of bytes in the payload.
  pub dlc: u8,
  /// The frame's data payload, only the first `dlc` bytes are valid.
  pub data: [u8; 8],
  /// True iff this frame is a Remote Transmission Request.
  pub rtr: bool,
  /// True iff the id field is extended (ie 29 bits long, as opposed to 11).
  pub ide: bool,
  /// At the time of this writing this field isn't specified, but it can be received as either
  /// value and subsequent protocols may end up using it.
  pub reserved0: bool,
  /// At the time of this writing this field isn't specified, but it can be received as either
  /// value and subsequent protocols may end up using it.
  pub reserved1: bool,
}

// TODO: write some data access methods for CanFrame
