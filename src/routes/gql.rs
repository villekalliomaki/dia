use crate::{
    access::{ClientIP, RateLimiter, UserFromJWT, JWT},
    db::{RedisConn, SqlxConn},
    gql::DiaSchema,
    Config,
};
use actix_web::{guard, web, HttpRequest, HttpResponse, Result, Scope};
use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    Data, Schema,
};
use async_graphql_actix_web::{Request, Response, WSSubscription};

/// Build GQL routes, currently POST for queries and WS, GET for the playground.
pub fn build() -> Scope {
    web::scope("/gql")
        .route("", web::post().to(index))
        .route(
            "",
            web::get()
                .guard(guard::Header("Upgrade", "websocket"))
                .to(ws),
        )
        .route("", web::get().to(playground))
        .route("/sdl", web::get().to(sdl))
}

/// Normal GraphQL queries as POST requests.
async fn index(
    schema: web::Data<DiaSchema>,
    req: Request,
    pg: SqlxConn,
    rd: RedisConn,
    cfg: Config,
    ip: ClientIP,
    rl: RateLimiter,
    jwt: JWT,
    user_jwt: UserFromJWT,
) -> Response {
    let mut request = req.into_inner();

    let mut data = Data::default();

    data.insert(pg.into_inner());
    data.insert(rd.into_inner());
    data.insert(ip.into_inner());
    data.insert(cfg);
    data.insert(rl);
    data.insert(jwt);

    // Convert UserFromJWT to User, since context will error out if it doesn't exist
    if let Some(user) = user_jwt.0 {
        data.insert(user);
    }

    request.data = data;

    schema.execute(request).await.into()
}

/// Websocket queries and subscriptions.
async fn ws(
    schema: web::Data<DiaSchema>,
    req: HttpRequest,
    payload: web::Payload,
    pg: SqlxConn,
    rd: RedisConn,
    cfg: Config,
    ip: ClientIP,
    rl: RateLimiter,
    jwt: JWT,
    user_jwt: UserFromJWT,
) -> Result<HttpResponse> {
    WSSubscription::start_with_initializer(Schema::clone(&*schema), &req, payload, |_| async {
        let mut data = Data::default();

        data.insert(pg.into_inner());
        data.insert(rd.into_inner());
        data.insert(ip.into_inner());
        data.insert(cfg);
        data.insert(rl);
        data.insert(jwt);

        // Convert UserFromJWT to User, since context will error out if it doesn't exist
        if let Some(user) = user_jwt.0 {
            data.insert(user);
        }

        Ok(data)
    })
}

/// Load a static playground.
async fn playground() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(
            GraphQLPlaygroundConfig::new("/api/gql").subscription_endpoint("/api/gql"),
        ))
}

/// Server the schema definition as text.
async fn sdl(schema: web::Data<DiaSchema>) -> HttpResponse {
    HttpResponse::Ok().body(schema.sdl())
}

#[cfg(test)]
mod tests {
    use crate::routes::build;
    use actix_web::{http, test, App};

    #[actix_rt::test]
    async fn gql_load_playground() {
        let mut app = test::init_service(App::new().service(build())).await;

        let req = test::TestRequest::get().uri("/api/gql").to_request();

        let response = test::call_service(&mut app, req).await;

        assert_eq!(response.status(), http::StatusCode::OK);
    }
}
