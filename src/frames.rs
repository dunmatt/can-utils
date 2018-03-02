//! The [`frames`] module houses various CAN frame representations.
//!
//! Regular CAN is a very low level technology (OSI layers 1 and 2), and as such there's a need for
//! data representations that don't impose usage patterns on the user (since hardware needs vary).
//! But at the same time, one of Rusts great strengths is its type system and most Rustaceans
//! prefer more type level sanity checks.  This module contains frame representations that cater
//! to both use cases, as well as easy (and cheap) layers to convert between them.

pub use ::byte_orders::*;

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

/// A standard representation of the frames that might be sent and received on a CAN FD bus.
///
/// This struct can represent any CAN FD frame, as described in the CAN FD specification
/// version 1.0, published April 2012 by Bosch GmbH.  They can be used for either
/// transmission or reception.
pub struct CanFdFrame {
  /// This contains either the Base Identifier or the Extended Identifier, depending on `ide`.
  pub id: u32,
  /// Number of bytes in the payload.
  ///
  /// # Note
  /// This is *not* the DLC field value, this is the number of bytes of payload in the frame,
  /// in CAN FD those are not the same thing, but the implementation of this HAL should hide that
  /// from you.
  pub data_length: u8,
  /// The frame's data payload, only the first `data_length` bytes are valid.
  pub data: [u8; 64],
  /// True iff the id field is extended (ie 29 bits long, as opposed to 11).
  pub ide: bool,
  /// True iff this is a CAN FD format frame.  Including it here to give implementations the option
  /// to use CanFdFrame for all traffic on the bus, if they so choose.
  pub edl: bool,
  /// True iff the frame was sent with a switched bit rate.
  pub brs: bool,
  /// True iff the sender is in FaultConfinementState::ErrorPassive (or possibly in BusOff).
  pub esi: bool,
  /// At the time of this writing this field isn't specified, but it can be received as either
  /// value and subsequent protocols may end up using it.
  pub reserved0: bool,
  /// At the time of this writing this field isn't specified, but it can be received as either
  /// value and subsequent protocols may end up using it.
  pub reserved1: bool,
}
