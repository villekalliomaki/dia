use crate::access::JWT;
use async_graphql::*;

#[derive(Default)]
pub struct JwtQuery;

#[Object]
impl JwtQuery {
    /// The public key can be used to validate signed and issued JSON web tokens.
    async fn jwt_public_key<'ctx>(&self, ctx: &Context<'ctx>) -> Result<&'ctx str> {
        Ok(std::str::from_utf8(&ctx.data::<JWT>()?.public_key)?)
    }
}
