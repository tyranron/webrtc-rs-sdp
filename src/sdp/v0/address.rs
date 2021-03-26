use std::{fmt, net::IpAddr};

use derive_more::Display;
use smartstring::alias::String;

#[derive(Clone, Debug, Display, Eq, PartialEq)]
pub enum Full {
    #[display(fmt = "IN {} {}", addrtype, domain)]
    Fqdn { addrtype: Type, domain: String },

    #[display(fmt = "IN {} {}", "self.addrtype()", _0)]
    Ip(IpAddr),
}

impl Typed for Full {
    fn addrtype(&self) -> Type {
        match self {
            Self::Fqdn { addrtype, .. } => *addrtype,
            Self::Ip(ip) => ip.addrtype(),
        }
    }
}

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
