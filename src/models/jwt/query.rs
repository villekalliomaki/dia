use crate::{
    access::jwt::{JwtClaims, JWT},
    gql::E,
};
use async_graphql::*;

#[derive(Default)]
pub struct JwtQuery;

#[Object]
impl JwtQuery {
    /// The public key can be used to validate signed and issued JSON web tokens.
    async fn jwt_public_key<'ctx>(&self, ctx: &Context<'ctx>) -> std::result::Result<&'ctx str, E> {
        Ok(std::str::from_utf8(&ctx.data::<JWT>()?.public_key)?)
    }

    /// Check if the server would accept the token.
    /// Note that the token might expire right after validating it here.
    async fn is_jwt_valid(&self, ctx: &Context<'_>, token: String) -> std::result::Result<bool, E> {
        let jwt = ctx.data::<JWT>()?;

        match jwt.decode(&token) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    /// Decode a signed JWT. Only if the token is usable and valid the claims are returned.
    async fn decode_jwt(
        &self,
        ctx: &Context<'_>,
        jwt: String,
    ) -> std::result::Result<JwtClaims, E> {
        Ok(ctx.data::<JWT>()?.decode(&jwt)?.claims)
    }
}
