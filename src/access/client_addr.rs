use actix_web::{dev::Payload, FromRequest, HttpRequest};
use std::net::{AddrParseError, IpAddr};
use futures::future::{err, ok, Ready};
use crate::res::Res;

pub struct ClientIP(IpAddr);

impl ClientIP {
    pub fn new(from: &str) -> Result<Self, AddrParseError> {
        Ok(ClientIP(from.parse()?))
    }

    pub fn into_inner(self) -> IpAddr {
        self.0
    }
}

impl FromRequest for ClientIP {
    type Error = Res<()>;
    type Future = Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        
    }
}