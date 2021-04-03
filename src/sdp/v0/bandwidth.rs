use derive_more::{AsMut, AsRef, Deref, DerefMut, Display, From, FromStr, Into};
use once_cell::sync::Lazy;
use smartstring::alias::String;
use regex::Regex;

/// Representation of a [SDP bandwidth].
///
/// Denotes the proposed bandwidth to be used by the session or media.
///
/// [SDP origin]: https://tools.ietf.org/html/rfc4566#section-5.8
#[derive(Clone, Debug, Display, Eq, PartialEq)]
pub enum Bandwidth {
    /// Bandwidth figure for a single media at a single site (although there may be many sites
    /// sending simultaneously).
    ///
    /// > The bandwidth is interpreted to be application specific (it will be the application's
    /// > concept of maximum bandwidth). Normally, this will coincide with what is set on the
    /// > application's "maximum bandwidth" control if applicable. For RTP-based applications, AS
    /// > gives the RTP "session bandwidth" as defined in [Section 6.2 of RFC 3550][1].
    ///
    /// [1]: https://tools.ietf.org/html/rfc3550#section-6.2
    #[display(fmt = "AS:{}", _0)]
    As(u32),

    /// Total bandwidth figure for all the media at all sites.
    ///
    /// > If the bandwidth of a session or media in a session is different from the bandwidth
    /// > implicit from the scope, a "b=CT:..." line SHOULD be supplied for the session giving the
    /// > proposed upper limit to the bandwidth used (the "conference total" bandwidth). The primary
    /// > purpose of this is to give an approximate idea as to whether two or more sessions can
    /// > coexist simultaneously. When using the CT modifier with RTP, if several RTP sessions are
    /// > part of the conference, the conference total refers to total bandwidth of all RTP
    /// > sessions.
    #[display(fmt = "CT:{}", _0)]
    Ct(u32),

    /// Transport Independent Application Specific Maximum (TIAS) bandwidth modifier that does not
    /// include transport overhead, defined by [RFC 3890].
    ///
    /// > At the SDP session level, the TIAS value is the maximal amount of bandwidth needed when
    /// > all declared media streams are used. This MAY be less than the sum of all the individual
    /// > media streams values. This is due to the possibility that not all streams have their
    /// > maximum at the same point in time. This can normally only be verified for stored media
    /// > streams.
    ///
    /// [RFC 3890]: https://tools.ietf.org/html/rfc3890
    #[display(fmt = "TIAS:{}", _0)]
    Tias(u32),

    /// Custom bandwidth type, not defined by [RFC 4566] or [RFC 3890].
    ///
    /// [RFC 3890]: https://tools.ietf.org/html/rfc3890
    /// [RFC 4566]: https://tools.ietf.org/html/rfc4566
    #[display(fmt = "{}:{}", bwtype, bandwidth)]
    Custom {
        /// Alphanumeric modifier giving the meaning of the `bandwidth` figure.
        bwtype: Type,

        /// Kilobits per second, by default.
        ///
        /// > The definition of a new `<bwtype>` modifier MAY specify that the bandwidth is to be
        /// > interpreted in some alternative unit.
        bandwidth: u32,
    },
}

/// Alphanumeric modifier giving the meaning of a [`Bandwidth::Custom::bandwidth`] figure, as
/// described in [Section 5.8 of RFC 4566][1].
///
/// [1]: https://tools.ietf.org/html/rfc4566#section-5.8
#[derive(AsRef, Clone, Debug, Deref, Display, Eq, Into, PartialEq)]
#[as_ref(forward)]
#[deref(forward)]
pub struct Type(String);

impl Type {
    /// Tries to construct a new [`Type`] out of the given `value`.
    ///
    /// # Errors
    ///
    /// If the given `value` doesn't represent a valid [`Type`].
    /// See [`InvalidTypeError`] for details.
    fn try_new<S: AsRef<str> + Into<String>>(value: S) -> Result<Self, InvalidTypeError> {
        // TODO: Use custom parser baked into a crate.
        static REGEX: Lazy<Regex> = Lazy::new(|| Regex::new("^(X-)?[a-zA-Z0-9]+$").unwrap());

        match value.as_ref() {
            "" => Err(InvalidTypeError::Empty),
            v if !REGEX.is_match(v) => Err(InvalidTypeError::Invalid),
            _ => Ok(Self(value.into())),
        }
    }
}

/// Error of validating a value to be a valid [`Type`].
#[derive(Clone, Copy, Debug, Display, Error)]
pub enum InvalidTypeError {
    /// [`Type`] is an empty string.
    #[display(fmt = "cannot be empty")]
    Empty,

    /// [`Type`] string contains non-alphanumeric symbols, which doesn't comply with a
    /// [Section 5.8 of RFC 4566][1].
    ///
    /// [1]: https://tools.ietf.org/html/rfc4566#section-5.8
    #[display(fmt = "cannot contain non-alphanumeric symbols")]
    Invalid,
}
