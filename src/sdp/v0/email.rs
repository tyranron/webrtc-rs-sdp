use derive_more::{AsMut, AsRef, Deref, DerefMut, Display, From, FromStr, Into};
use smartstring::alias::String;

// TODO: Provide better newtype validation.
#[derive(AsRef, Clone, Debug, Deref, Display, Eq, From, FromStr, Into, PartialEq)]
#[as_ref(forward)]
#[deref(forward)]
pub struct Address(String);