//! The `timing_calculator` module computes CAN interface timing parameters.

use core::cmp;

type BaudRatePrescalaInnerType = u32;

type BitSamplePointInnerType = u16;

type SegmentLengthInnerType = u8;

/// HACK: once uom supports no_std replace this with a more sane approach to specifying bitrates
pub struct BitsPerSecond(u32);

/// HACK: Once uom supports no_std, replace this with a more standard frequency representation.
pub struct MegaHertz(u32);

/// CAN timing is controlled in units of Time Quanta, this codifies that.
pub struct SegmentLength(SegmentLengthInnerType);

/// This struct bundles various maximum limits all CAN interfaces have.
pub struct CanTimingLimits {
  /// The largest supported BRP, almost always a power of two.
  pub max_baud_rate_prescaler: u32,
  /// The max length of seg1, almost always a power of two.
  pub max_segment_1_length: SegmentLength,
  /// The max length of seg2, almost always a power of two.
  pub max_segment_2_length: SegmentLength,
  /// The maximum value it is possible to configure the interface to set the maximum jump to.
  pub max_jump_width: SegmentLength,
}

/// The results of an interface timing calculation.
pub struct CanBitTimingParameters {
  /// The CAN interface will scale the clock frequency back by this amount.
  pub baud_rate_prescaler: BaudRatePrescalaInnerType,
  /// The number of time quanta before the sample point (not counting the Sync Seg Quantum).
  pub seg1: SegmentLength,
  /// The number of time quanta after the sample point.
  pub seg2: SegmentLength,
  /// The number of time quanta to adjust seg1 and seg2 by at each synchronization step.
  pub jump_width: SegmentLength,
}

/// `BitSamplePoint` is the fraction of the way through each bit the interface takes its sample.
pub struct BitSamplePoint {
  /// Decipercentage of the way through the bit that the sample point should be.
  tenths_of_a_percent: BitSamplePointInnerType,
}

impl BitSamplePoint {
  /// The minimum sample point is 50%, per the spec.
  pub const MINIMUM_SAMPLE_POINT: BitSamplePoint = BitSamplePoint { tenths_of_a_percent: 500 };
  /// There is no official maximum, except that seg2 must be at least one time quantum.
  pub const MAXIMUM_SAMPLE_POINT: BitSamplePoint = BitSamplePoint { tenths_of_a_percent: 990 };

  /// Constructs a target sample point after some sanity checking.
  pub fn new(tenths_of_a_percent: BitSamplePointInnerType) -> BitSamplePoint {
    // TODO: add sanity checking here that the value is between 500 and 990
    BitSamplePoint { tenths_of_a_percent }
  }
}

/// Iterates over all timing parameter solutions.
pub struct CanBitTimingParameterIter<'a> {
  bit_width: u32,
  jump_width: SegmentLengthInnerType,
  target_sample_point: BitSamplePointInnerType,
  last_attempted_prescaler: BaudRatePrescalaInnerType,

  interface_limits: &'a CanTimingLimits,
}

impl<'a> Iterator for CanBitTimingParameterIter<'a> {
  type Item = CanBitTimingParameters;

  fn next(&mut self) -> Option<CanBitTimingParameters> {
    let mut result: Option<CanBitTimingParameters> = None;

    while result.is_none() &&
          self.last_attempted_prescaler < self.interface_limits.max_baud_rate_prescaler {
      self.last_attempted_prescaler += 1;
      if self.bit_width % self.last_attempted_prescaler != 0 {
        continue;
      }
      let tq = self.bit_width / self.last_attempted_prescaler;
      if tq < 8 {  // not sure why this is 8, but that's what the can docs and calculators say
        continue;
      }
      // TODO: sorry for all the casting here, clean it up once uom builds with no_std
      let sample = tq * 1000 / (self.target_sample_point as u32);
      let seg2 = cmp::max(1, tq - sample) as SegmentLengthInnerType;  // seg2 must be >= 1 quantum
      let seg1 = tq as SegmentLengthInnerType - seg2 - 1;  // magic 1 here is the Sych segment.

      // we dont' need to verify that seg1 >= seg2 because target_sample_point >= 50%
      if seg1 > self.interface_limits.max_segment_1_length.0 ||
          seg2 > self.interface_limits.max_segment_2_length.0 {
        continue;
      }
      result = Some(CanBitTimingParameters {
        baud_rate_prescaler: self.last_attempted_prescaler,
        seg1: SegmentLength(seg1),
        seg2: SegmentLength(seg2),
        jump_width: SegmentLength(self.jump_width),
      });
    }
    result
  }
}

// TODO: spend a lot more time documenting this
/// Returns an iterator that iterates over all timing solutions to the given problem.
pub fn compute_timing_parameters<'a>(clock_speed: MegaHertz,
                                     nominal_bitrate: BitsPerSecond,
                                     target_sample_point: BitSamplePoint,
                                     jump_width: SegmentLength,
                                     limits: &'a CanTimingLimits) -> CanBitTimingParameterIter {
  CanBitTimingParameterIter {
    bit_width: 1_000_000 * clock_speed.0 / nominal_bitrate.0,
    jump_width: jump_width.0,
    target_sample_point: target_sample_point.tenths_of_a_percent,
    last_attempted_prescaler: 1,
    interface_limits: limits,
  }
}

/// The recommended bit sampling points for various protocols.  Get your defaults here.
pub mod recommended_sample_points {
  use super::BitSamplePoint;
  ///
  pub const ARINC825: BitSamplePoint = BitSamplePoint { tenths_of_a_percent: 750 };
  ///
  pub const CANOPEN: BitSamplePoint = BitSamplePoint { tenths_of_a_percent: 875 };
  ///
  pub const DEVICENET: BitSamplePoint = BitSamplePoint { tenths_of_a_percent: 875 };
  ///
  pub const J1939: BitSamplePoint = BitSamplePoint { tenths_of_a_percent: 900 };
  ///
  pub const J2284: BitSamplePoint = BitSamplePoint { tenths_of_a_percent: 900 };
  // TODO: add more of these
}
