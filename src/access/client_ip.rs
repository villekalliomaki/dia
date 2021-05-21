use crate::res::Res;
use actix_web::{dev::Payload, FromRequest, HttpRequest};
use futures::future::{err, ok, Ready};
use std::net::{AddrParseError, IpAddr};

/**
 * Wrapper type to extract the client address from a request.
 * `Forwarded` and `X-Forwarded-For` should be removed or set by a reverse proxy.
 * If no header is set falls back to the sockets address.
 */
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
        match req.connection_info().realip_remote_addr() {
            Some(value) => match ClientIP::new(value) {
                Ok(value) => ok(value),
                Err(error) => err(Res::<()>::error(format!(
                    "Failed to parse client address: {}",
                    error
                ))),
            },
            None => err(Res::<()>::error("Failed to get client address.")),
        }
    }
}
