//! [SDP specification][1] definitions.
//!
//! [1]: https://tools.ietf.org/html/rfc4566#section-5

pub mod v0;

/// Representation of an [`o=` field][1], providing the originator of the session (her username and
/// the address of the user's host) plus a session identifier and version number.
///
/// [1]: https://tools.ietf.org/html/rfc4566#section-5.2
#[derive(Debug, Default)]
pub struct Origin {
    username: String,
    session_id: u64,
    session_version: u64,
    network_type: String,
    address_type: String,
    unicast_address: String,
}
