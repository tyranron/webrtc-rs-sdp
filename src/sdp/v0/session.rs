use derive_more::{AsMut, AsRef, Deref, DerefMut, Display, From, Into};
use smart_default::SmartDefault;
use smartstring::alias::String;

/// Representation of [SDP origin]'s `sess-id`.
///
/// See [`Origin::sess_id`] docs for details.
///
/// [SDP origin]: https://tools.ietf.org/html/rfc4566#section-5.2
/// [`Origin::sess_id`]: super::Origin::sess_id
#[derive(Clone, Copy, Debug, Display, Eq, From, Into, PartialEq)]
pub struct Id(u64);

/// Representation of [SDP origin]'s `sess-version`.
///
/// See [`Origin::sess_version`] docs for details.
///
/// [SDP origin]: https://tools.ietf.org/html/rfc4566#section-5.2
/// [`Origin::sess_version`]: super::Origin::sess_version
#[derive(Clone, Copy, Debug, Display, Eq, From, Into, Ord, PartialEq, PartialOrd)]
pub struct Version(u64);

/// Representation of [SDP session name][1].
///
/// # Default
///
/// > If a session has no meaningful name, the value "s= " SHOULD be used (i.e., a single space as
/// > the session name).
///
/// [1]: https://tools.ietf.org/html/rfc4566#section-5.3
#[derive(AsRef, Clone, Debug, Deref, Display, Eq, Into, PartialEq, SmartDefault)]
#[as_ref(forward)]
#[deref(forward)]
pub struct Name(#[default = " "] String);

impl Name {
    /// Tries to construct a new [`session::Name`](Name) out of the given `value`.
    ///
    /// # Errors
    ///
    /// If the given `value` doesn't represent a valid [`session::Name`](Name).
    /// See [`InvalidNameError`] for details.
    fn try_new<S: AsRef<str> + Into<String>>(value: S) -> Result<Self, EmptyNameError> {
        if value.as_ref().is_empty() {
            Err(EmptyNameError)
        } else {
            Ok(Self(value.into()))
        }
    }
}

/// Error of a [`session::Name`](Name) value being an empty string.
#[derive(Clone, Copy, Debug, Display, Error)]
#[display(fmt = "cannot be empty")]
pub struct EmptyNameError;

/// Representation of [SDP session information][1].
///
/// [1]: https://tools.ietf.org/html/rfc4566#section-5.4
#[derive(AsMut, AsRef, Clone, Debug, Deref, DerefMut, Display, Eq, From, Into, PartialEq)]
#[as_mut(forward)]
#[as_ref(forward)]
#[deref(forward)]
#[deref_mut(forward)]
pub struct Information(String);