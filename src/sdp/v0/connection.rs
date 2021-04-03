use std::net::IpAddr;

use derive_more::Display;
use smartstring::alias::String;

use super::address::{self, Typed as _};

#[derive(Clone, Debug, Display, Eq, PartialEq)]
pub enum Data {
    #[display(fmt = "IN {} {}", addrtype, domain)]
    Fqdn {
        addrtype: address::Type,
        domain: String,
    },

    #[display(fmt = "IN {} {}", "self.addrtype()", _0)]
    Ip(IpAddr),
}

impl address::Typed for Data {
    fn addrtype(&self) -> address::Type {
        match self {
            Self::Fqdn { addrtype, .. } => *addrtype,
            Self::Ip(ip) => ip.addrtype(),
        }
    }
}
