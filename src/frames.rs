//! The `frames` module houses various CAN frame representations.
//!
//! Regular CAN is a very low level technology (OSI layers 1 and 2), and as such there's a need for
//! data representations that don't impose usage patterns on the user (since hardware needs vary).
//! But at the same time, one of Rusts great strengths is its type system and most Rustaceans
//! prefer more type level sanity checks.  This module contains frame representations that cater
//! to both use cases, as well as easy (and cheap) layers to convert between them.

/// A low level representation of the frames that might be sent and received on a CAN bus.
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

/// A low level representation of the frames that might be sent and received on a CAN FD bus.
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

/// Converts a CAN-FD DLC into a byte count.
///
/// NOTE: According to the CAN 2.0 spec a data length of 8 can be encoded as any DLC >= 8.
/// This function has no way of knowing the frame type, so be sure to only call it after
/// you've verified that it's a CAN-FD frame you're dealing with.
pub fn can_fd_dlc_to_byte_count(dlc: u8) -> u8 {
  match dlc & 0xF {
    0...8 => dlc,
    9...12 => 8 + 4 * (dlc & 0b111),
    13 => 32,
    14 => 48,
    15 => 64,
    _ => unreachable!(),
  }
}

/// Converts a byte count into a CAN-FD DLC.
///
/// NOTE: Not all byte counts can be represented as DLCs, which by implication means that not all
/// byte counts are valid CAN-FD frame sizes.  This function accounts for the truncation and
/// padding that may be incurred as a result of that.
///
/// If n != byte_count_to_can_fd_dlc(can_fd_dlc_to_byte_count(n)) truncation or padding will occur.
pub fn byte_count_to_can_fd_dlc(byte_count: u8) -> u8 {
  match byte_count {
    0...8 => byte_count,
    9...12 => 0b1001,
    13...16 => 0b1010,
    17...20 => 0b1011,
    21...24 => 0b1100,
    25...32 => 0b1101,
    32...48 => 0b1110,
    _ => 0b1111,
  }
}
