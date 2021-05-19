use crate::{config::Config, gql::DiaSchema};
use actix_web::{guard, web, HttpRequest, HttpResponse, Result, Scope};
use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    Schema,
};
use async_graphql_actix_web::{Request, Response, WSSubscription};
use redis::Client;
use sqlx::{Pool, Postgres};

/**
 * Build GQL routes, currently POST for queries and WS, GET for the playground.
 */
pub fn build() -> Scope {
    web::scope("/gql")
        .route("", web::post().to(index))
        .route("", web::get().to(playground))
        .route(
            "",
            web::get()
                .guard(guard::Header("Upgrade", "websocket"))
                .to(ws),
        )
        .route("/sdl", web::get().to(sdl))
}

/**
 * Normal GraphQL queries as POST requests.
 */
async fn index(
    schema: web::Data<DiaSchema>,
    req: Request,
    pg: web::Data<Pool<Postgres>>,
    rd: web::Data<Client>,
    cfg: web::Data<Config>,
) -> Response {
    let mut request = req.into_inner();

    request = request.data(pg);
    request = request.data(rd);
    request = request.data(cfg);

    schema.execute(request).await.into()
}

/**
 * Websocket queries and subscriptions.
 */
async fn ws(
    schema: web::Data<DiaSchema>,
    req: HttpRequest,
    payload: web::Payload,
    pg: web::Data<Pool<Postgres>>,
    rd: web::Data<Client>,
    cfg: web::Data<Config>,
) -> Result<HttpResponse> {
    WSSubscription::start(Schema::clone(&*schema), &req, payload)
}

/**
 * Load a static playground.
 */
async fn playground() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(
            GraphQLPlaygroundConfig::new("/api/gql").subscription_endpoint("/api/gql"),
        ))
}

/**
 * Server the schema definition as text.
 */
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
