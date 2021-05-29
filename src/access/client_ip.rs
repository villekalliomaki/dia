use crate::res::Res;
use actix_web::{dev::Payload, FromRequest, HttpRequest};
use futures::future::{err, ok, Ready};
use std::net::{AddrParseError, IpAddr, SocketAddr};

/// Wrapper type to extract the client address from a request.
/// `Forwarded` and `X-Forwarded-For` should be removed or set by a reverse proxy.
/// If no header is set, falls back to the sockets address.
/// Using `Clone` -derive here is fine, since `IpAddr` implements one.
#[derive(Clone)]
pub struct ClientIP(IpAddr);

impl ClientIP {
    pub fn new(from: &str) -> Result<Self, AddrParseError> {
        Ok(match from.parse::<IpAddr>() {
            Ok(value) => ClientIP(value),
            Err(_) => {
                // Try to parse into a socket address
                match from.parse::<SocketAddr>() {
                    Ok(value) => ClientIP(value.ip()),
                    Err(error) => return Err(error),
                }
            }
        })
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
