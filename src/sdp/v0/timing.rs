use std::{fmt, time::Duration as StdDuration};

use derive_more::{Display, From, Into};
use smallvec::SmallVec;
use std::intrinsics::offset;

/// Representation of `t=` and `r=` fields of [`SessionDescription`] as defined in [Section 5.9] and
/// [Section 5.10] of [RFC 4566].
///
/// These fields are used to specify the start and stop times for a session as well as repeat
/// intervals and durations for the scheduled session.
///
/// [`SessionDescription`]: super::SessionDescription
/// [RFC 4566]: https://tools.ietf.org/html/rfc4566
/// [Section 5.9]: https://tools.ietf.org/html/rfc4566#section-5.9
/// [Section 5.10]: https://tools.ietf.org/html/rfc4566#section-5.10
#[derive(Clone, Debug)]
pub struct Description {
    /// Start and stop times of a session.
    ///
    /// From [Section 5.9 of RFC 4566][1]:
    /// > ```ignore
    /// > t=<start-time> <stop-time>
    /// > ```
    ///
    /// [1]: https://tools.ietf.org/html/rfc4566#section-5.9
    pub timing: Timing,

    /// Repeat intervals and durations of a scheduled session.
    ///
    /// From [Section 5.10 of RFC 4566][1]:
    /// > ```ignore
    /// > r=<repeat interval> <active duration> <offsets from start-time>
    /// > ```
    ///
    /// [1]: https://tools.ietf.org/html/rfc4566#section-5.10
    pub repeat_times: Vec<RepeatTime>,
}

/// Representation of a timing as defined in [Section 5.9 of RFC 4566][1].
///
/// [1]: https://tools.ietf.org/html/rfc4566#section-5.9
#[derive(Clone, Copy, Debug, Display, Eq, PartialEq)]
#[display(fmt = "{} {}", start_time, stop_time)]
pub struct Timing {
    /// Start time of a session.
    ///
    /// From [Section 5.9 of RFC 4566][1]:
    /// > If the `<stop-time>` is set to zero... If the `<start-time>` is also zero, the session is
    /// > regarded as permanent.
    ///
    /// [1]: https://tools.ietf.org/html/rfc4566#section-5.9
    pub start_time: Time,

    /// Stop time of a session.
    ///
    /// From [Section 5.9 of RFC 4566][1]:
    /// > If the `<stop-time>` is set to zero, then the session is not bounded, though it will not
    /// > become active until after the `<start-time>`.
    ///
    /// [1]: https://tools.ietf.org/html/rfc4566#section-5.9
    pub stop_time: Time,
}

impl Timing {
    /// Indicates whether this [`Timing`] is unbounded (has no stop time).
    ///
    /// From [Section 5.9 of RFC 4566][1]:
    /// > If the `<stop-time>` is set to zero, then the session is not bounded, though it will not
    /// > become active until after the `<start-time>`.
    ///
    /// [1]: https://tools.ietf.org/html/rfc4566#section-5.9
    #[inline]
    #[must_use]
    pub fn is_unbounded(self) -> bool {
        self.stop_time.iz_zero()
    }

    /// Indicates whether this [`Timing`] is permanent.
    ///
    /// From [Section 5.9 of RFC 4566][1]:
    /// > If the `<stop-time>` is set to zero... If the `<start-time>` is also zero, the session is
    /// > regarded as permanent.
    ///
    /// [1]: https://tools.ietf.org/html/rfc4566#section-5.9
    #[inline]
    #[must_use]
    pub fn is_permanent(self) -> bool {
        self.start_time.iz_zero() && self.stop_time.iz_zero()
    }
}

/// Representation of a time as defined in [Section 5.9 of RFC 4566][1].
///
/// > These values are the decimal representation of Network Time Protocol (NTP) time values in
/// > seconds since 1900 [13]. To convert these values to UNIX time, subtract decimal 2208988800.
/// >
/// > NTP timestamps are elsewhere represented by 64-bit values, which wrap sometime in the year
/// > 2036. Since SDP uses an arbitrary length decimal representation, this should not cause an
/// > issue (SDP timestamps MUST continue counting seconds since 1900, NTP will use the value modulo
/// > the 64-bit limit).
///
/// [1]: https://tools.ietf.org/html/rfc4566#section-5.9
/// [13]: https://tools.ietf.org/html/rfc1305
#[derive(Clone, Copy, Debug, Default, Display, Eq, From, Into, PartialEq)]
pub struct Time(u64);

impl Time {
    /// Indicate whether this [`Time`] equals to its default zero value.
    #[inline]
    #[must_use]
    pub fn iz_zero(self) -> bool {
        self.0 == 0
    }
}
/// Representation of a repeat time for repeated scheduled sessions as defined in
/// [Section 5.10 of RFC 4566][1].
///
/// [1]: https://tools.ietf.org/html/rfc4566#section-5.10
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RepeatTime {
    /// Repeat interval of a session.
    repeat_interval: Duration,

    /// Active duration of a session.
    active_duration: Duration,

    /// Offsets from [`Timing::start_time`] of a session.
    offsets: SmallVec<[Offset; 2]>,
}

// Manual implementation here allows to omit redundant allocation
// when `Display`ing `RepeatTime::offsets`.
impl fmt::Display for RepeatTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.active_duration, self.repeat_interval)?;
        for o in &self.offsets {
            write!(f, " {}", o)?;
        }
        Ok(())
    }
}

/// Representation of a [`RepeatTime`] duration in seconds as defined in
/// [Section 5.10 of RFC 4566][1].
///
/// [1]: https://tools.ietf.org/html/rfc4566#section-5.10
#[derive(Clone, Copy, Debug, Default, Display, Eq, From, Into, PartialEq)]
pub struct Duration(u64);

impl Duration {
    /// Indicates whether this [`Duration`] equals to its default zero value.
    #[inline]
    #[must_use]
    pub fn is_zero(self) -> bool {
        self.0 == 0
    }

    /// Converts this [`Duration`] to a value of [`StdDuration`].
    #[inline]
    #[must_use]
    pub fn to_std(self) -> StdDuration {
        StdDuration::from_secs(self.0)
    }
}

impl From<Duration> for StdDuration {
    #[inline]
    fn from(d: Duration) -> Self {
        d.to_std()
    }
}

/// Representation of a [`RepeatTime`]/[`TimeZone`] offset in seconds as defined in
/// [Section 5.10] and [Section 5.11] of [RFC 4566].
///
/// [RFC 4566]: https://tools.ietf.org/html/rfc4566
/// [Section 5.10]: https://tools.ietf.org/html/rfc4566#section-5.10
/// [Section 5.11]: https://tools.ietf.org/html/rfc4566#section-5.11
#[derive(Clone, Copy, Debug, Default, Display, Eq, From, Into, PartialEq)]
pub struct Offset(i64);

impl Offset {
    /// Indicates whether this [`Offset`] equals to its default zero value.
    #[inline]
    #[must_use]
    pub fn is_zero(self) -> bool {
        self.0 == 0
    }
}

/// Representation of a time zone adjustment for a repeated sessions scheduling as defined in
/// [Section 5.11 of RFC 4566][1].
///
/// [1]: https://tools.ietf.org/html/rfc4566#section-5.11
#[derive(Clone, Copy, Debug, Display)]
#[display(fmt = "{} {}", adjustment_time, offset)]
pub struct TimeZone {
    /// Adjustment time base by which the session's repeat times are calculated considering the
    /// [`TimeZone::offset`].
    ///
    /// From [Section 5.11 of RFC 4566][1]:
    /// > Adjustments are always relative to the specified start time -- they are not cumulative.
    /// > Adjustments apply to all "t=" and "r=" lines in a session description.
    ///
    /// [1]: https://tools.ietf.org/html/rfc4566#section-5.11
    pub adjustment_time: Time,

    /// Offset to be applied after the [`TimeZone::adjustment_time`] base.
    pub offset: Offset,
}