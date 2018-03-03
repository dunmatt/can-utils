//! The `interface` module holds various types for modeling parts of a CAN interface.

/// The intended behavior of a CAN filter.
pub enum MessageFilterType {
  /// Signifies a filter that includes traffic it matches.
  MatchMeansAccept,
  /// Signifies a filter that excludes traffic it matches.
  MatchMeansIgnore,
}

/// A filter that the hardware might apply to incomming traffic.
pub struct MessageFilter {
  /// The CAN id (or common subset of a CAN idea if a mask is specified) to filter or select.
  pub id: u32,
  /// Incomming CAN ids are masked with this mask (if present) before being compared against id.
  pub mask: Option<u32>,
  /// The intent of this filter, is it a "forward if" or a "forward unless"?
  pub filter_type: MessageFilterType,
}

/// The 3 fault confinement states as described in the CAN 2.0 spec.
pub enum FaultConfinementState {
  /// Errors are so few that this interface tells the whole bus when they happen.
  ErrorActive,
  /// Errors are numerous enough that informing the bus of them isn't allowed, but regular Rx
  /// and Tx can still work.
  ErrorPassive,
  /// There are so many bus errors that we're effectively not connected, Rx and Tx are disabled.
  BusOff,
}

/// Operation Modes describe what the interface is currently doing.
pub enum InterfaceOperationMode {
  /// The interface is currently receiving a message from the bus.
  Receiver,
  /// The interface is currently transmitting a message from the bus.
  Transmitter,
  /// The interface is waiting to sync with the bus (detect 11 consecutive recessive bits).
  ///
  /// NOTE: this state was never described in the CAN 2.0 spec, only the CAN FD spec, so
  /// documentation not written with CAN FD in mind may not talk about how to detect it.
  /// That said, it is applicable to regular CAN hardware, they have this state for the same
  /// reason CAN-FD does, so people implementing `CanInterface` for non-FD hardware may have
  /// to do some thinking.
  Integrating,
  /// The interface ready and waiting to either transmit or receive.
  ///
  /// NOTE: this state was never described in the CAN 2.0 spec, only the CAN FD spec, so
  /// documentation not written with CAN FD in mind may not talk about how to detect it.
  /// That said, it is applicable to regular CAN hardware, they have this state for the same
  /// reason CAN-FD does, so people implementing `CanInterface` for non-FD hardware may have
  /// to do some thinking.
  Idle,
}

