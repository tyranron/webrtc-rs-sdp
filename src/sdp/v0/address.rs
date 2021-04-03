use std::net::IpAddr;

use derive_more::Display;

#[derive(Clone, Copy, Debug, Display, Eq, PartialEq)]
pub enum Type {
    #[display(fmt = "IP4")]
    Ip4,

    #[display(fmt = "IP6")]
    Ip6,
}

pub trait Typed {
    fn addrtype(&self) -> Type;
}

impl Typed for IpAddr {
    fn addrtype(&self) -> Type {
        match self {
            Self::V4(_) => Type::Ip4,
            Self::V6(_) => Type::Ip6,
        }
    }
}
