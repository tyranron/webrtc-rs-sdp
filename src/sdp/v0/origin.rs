//! [SDP origin] definitions.
//!
//! [SDP origin]: https://tools.ietf.org/html/rfc4566#section-5.2

use derive_more::{AsRef, Display, Error, Into};
use smartstring::alias::String;

use super::{connection, session};

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
    pub sess_id: session::Id,

    /// Version number for this session description.
    pub sess_version: session::Version,

    /// Address of the machine from which the session was created, along with `<addrtype>` and
    /// `<nettype>`.
    pub unicast_address: connection::Data,
}

/// Representation of [SDP origin]'s `username`.
///
/// `-` (hyphen) doesn't count as a valid [`Username`], but rather as its absence.
///
/// [SDP origin]: https://tools.ietf.org/html/rfc4566#section-5.2
#[derive(AsRef, Clone, Debug, Deref, Display, Eq, Into, PartialEq)]
#[as_ref(forward)]
#[deref(forward)]
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
            "" => Err(InvalidUsernameError::Empty),
            "-" => Err(InvalidUsernameError::Hyphen),
            v if v.contains(' ') => Err(InvalidUsernameError::WithSpaces),
            _ => Ok(Self(value.into())),
        }
    }
}

/// Error of validating a value to be a valid [`Username`].
#[derive(Clone, Copy, Debug, Display, Error)]
pub enum InvalidUsernameError {
    /// [`Username`] contains spaces, while must not.
    #[display(fmt = "cannot contain spaces")]
    WithSpaces,

    /// [`Username`] is an empty string.
    #[display(fmt = "cannot be empty")]
    Empty,

    /// [`Username`] is a hyphen, while hyphen should describe an absence of a [`Username`].
    /// Use [`None`] for hyphens.
    #[display(fmt = "cannot be `-` (hyphen), use `None` instead")]
    Hyphen,
}
