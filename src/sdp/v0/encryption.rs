use secrecy::{ExposeSecret as _, SecretString};
use url::Url;

// TODO: Consider new key exchange mechanisms for use with SDP from RFC 4567 and RFC 4568.
/// Representation of an encryption key conveyed by a session as defined in
/// [Section 5.12 of RFC 4566][1].
///
/// > If transported over a secure and trusted channel, the Session Description Protocol MAY be used
/// > to convey encryption keys. A simple mechanism for key exchange is provided by the key field
/// > ("k="), although this is primarily supported for compatibility with older implementations and
/// > its use is NOT RECOMMENDED.
///
/// [1]: https://tools.ietf.org/html/rfc4566#section-5.12
#[derive(Clone, Debug, Display)]
pub enum Key {
    /// Untransformed encryption key.
    ///
    /// From [Section 5.12 of RFC 4566][1]:
    /// > ```ignore
    /// > k=clear:<encryption key>
    /// > ```
    /// > This method MUST NOT be used unless it can be guaranteed that the SDP is conveyed over a
    /// > secure channel. The encryption key is interpreted as text according to the charset
    /// > attribute; use the "k=base64:" method to convey characters that are otherwise prohibited
    /// > in SDP.
    ///
    /// [1]: https://tools.ietf.org/html/rfc4566#section-5.12
    #[display(fmt = "clear:{}", "_0.expose_secret()")]
    Clear(SecretString),

    /// [Base64] encoded encryption key.
    ///
    /// From [Section 5.12 of RFC 4566][1]:
    /// > ```ignore
    /// > k=base64:<encoded encryption key>
    /// > ```
    /// > This method MUST NOT be used unless it can be guaranteed that the SDP is conveyed over a
    /// > secure channel.
    ///
    /// [1]: https://tools.ietf.org/html/rfc4566#section-5.12
    /// [Base64]: https://en.wikipedia.org/wiki/Base64
    #[display(fmt = "base64:{}", "_0.expose_secret()")]
    Base64(SecretString),

    /// URI referring to the data containing the encryption key, and may require additional
    /// authentication before the key can be returned.
    ///
    /// From [Section 5.12 of RFC 4566][1]:
    /// > ```ignore
    /// > k=uri:<URI to obtain key>
    /// > ```
    /// > The URI is often an Secure Socket Layer/Transport Layer Security (SSL/TLS)-protected HTTP
    /// > URI ("https:"), although this is not required.
    ///
    /// [1]: https://tools.ietf.org/html/rfc4566#section-5.12
    /// [URI]: https://en.wikipedia.org/wiki/Uniform_Resource_Identifier
    #[display(fmt = "uri:{}", _0)]
    Uri(Url),

    /// No key, but a user should be prompted for the encryption key when attempting to join a
    /// session.
    ///
    /// From [Section 5.12 of RFC 4566][1]:
    /// > ```ignore
    /// > k=prompt
    /// > ```
    /// > The use of user-specified keys is NOT RECOMMENDED, since such keys tend to have weak
    /// security properties.
    ///
    /// [1]: https://tools.ietf.org/html/rfc4566#section-5.12
    #[display(fmt = "prompt")]
    Prompt,
}
