//! [SDP origin] definitions.
//!
//! [SDP origin]: https://tools.ietf.org/html/rfc4566#section-5.2

use derive_more::{AsRef, Display, Error, From, Into};
use smartstring::alias::String;

use super::address;

/// Representation of an [SDP origin].
///
/// [SDP origin]: https://tools.ietf.org/html/rfc4566#section-5.2
#[derive(Clone, Debug, Display)]
#[display(
    fmt = "{} {} {} {}",
    "username.as_ref().unwrap_or(\"-\")",
    sess_id,
    sess_version,
    unicast_address
)]
pub struct Origin {
    /// User's login on the originating host, or [`None`] if the originating host doesn't support
    /// the concept of user IDs.
    pub username: Option<Username>,

    /// Numeric string such that the tuple of `<username>`, `<sess-id>`, `<nettype>`, `<addrtype>`,
    /// and `<unicast-address>` forms a globally unique identifier for the session.
    pub sess_id: SessionId,

    /// Version number for this session description.
    pub sess_version: SessionVersion,

    /// Address of the machine from which the session was created, along with `<addrtype>` and
    /// `<nettype>`.
    pub unicast_address: address::Full,
}

/// Representation of [SDP origin]'s `username`.
///
/// `-` (hyphen) doesn't count as a valid [`Username`], but rather as its absence.
///
/// [SDP origin]: https://tools.ietf.org/html/rfc4566#section-5.2
#[derive(AsRef, Clone, Debug, Display)]
#[as_ref(forward)]
pub struct Username(String);

impl Username {
    /// Tries to construct a new [`Username`] out of the given `value`.
    ///
    /// # Errors
    ///
    /// If the given `value` doesn't represent a valid [`Username`].
    /// See [`InvalidUsernameError`] for details.
    fn try_new<S: AsRef<str> + Into<String>>(value: S) -> Result<Self, InvalidUsernameError> {
        match value.as_ref() {
            "" => Err(InvalidUsernameError::IsEmpty),
            "-" => Err(InvalidUsernameError::IsHyphen),
            v if v.contains(' ') => Err(InvalidUsernameError::HasSpaces),
            _ => Ok(Self(value.into())),
        }
    }
}

/// Error of validating a value to be a valid [`Username`].
#[derive(Clone, Copy, Debug, Display, Error)]
pub enum InvalidUsernameError {
    /// [`Username`] contains spaces, while must not.
    #[display(fmt = "cannot contain spaces")]
    HasSpaces,

    /// [`Username`] is a empty string.
    #[display(fmt = "cannot be empty")]
    IsEmpty,

    /// [`Username`] is a hyphen, while hyphen should describe an absence of a [`Username`].
    /// Use [`None`] for hyphens.
    #[display(fmt = "cannot be `-` (hyphen), use `None` instead")]
    IsHyphen,
}

/// Representation of [SDP origin]'s `sess-id`.
///
/// See [`Origin::sess_id`] docs for details.
///
/// [SDP origin]: https://tools.ietf.org/html/rfc4566#section-5.2
#[derive(Clone, Copy, Debug, Display, Eq, From, Into, PartialEq)]
pub struct SessionId(u64);

/// Representation of [SDP origin]'s `sess-version`.
///
/// See [`Origin::sess_version`] docs for details.
///
/// [SDP origin]: https://tools.ietf.org/html/rfc4566#section-5.2
#[derive(Clone, Copy, Debug, Display, Eq, From, Into, Ord, PartialEq, PartialOrd)]
pub struct SessionVersion(u64);
