use crate::{access::JWT, models::user::User, res::Res};
use actix_web::{dev::Payload, FromRequest, HttpRequest};
use anyhow::Result;
use futures::future::{err, ok, Ready};

/// An user decoded from a valid JWT.
/// In case there is no `Authorization` header, the user in `None`.
/// When the header exists, it's value has to be valid.
#[derive(Clone)]
pub struct UserFromJWT(pub Option<User>);

impl FromRequest for UserFromJWT {
    type Error = Res<()>;
    type Future = Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        // Get header value if it exists
        // If there is no header, continue wihtout user
        let header_val = match req.headers().get("Authorization") {
            Some(header) => header,
            None => return ok(UserFromJWT(None)),
        };

        // Header value to &str
        let header_str = match header_val.to_str() {
            Ok(value) => value,
            Err(error) => {
                return err(Res::<()>::error(format!(
                    "Failed to parse Authorization -header: {}.",
                    error
                )))
            }
        };

        // Get the JWT provider
        let jwt = match req.app_data::<JWT>() {
            Some(jwt) => jwt,
            None => {
                error!("JWT doesn't exist in actix state");

                return err(Res::<()>::error("JWT doesn't exist in state."));
            }
        };

        // Decode claims
        let claims = match jwt.decode(header_str) {
            Ok(claims) => claims,
            Err(error) => return err(Res::<()>::error("JWT is invalid.")),
        };

        // Get the user data
        return ok(UserFromJWT(Some(claims.claims.user)));
    }
}
