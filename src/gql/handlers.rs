use crate::gql::{build_schema, query::Query};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use async_graphql_warp::Response;
use std::convert::Infallible;
use warp::{http::Response as HttpResponse, Filter, Rejection, Reply};

pub fn build() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let schema = build_schema();

    let graphql_post = async_graphql_warp::graphql(schema).and_then(
        |(schema, request): (
            Schema<Query, EmptyMutation, EmptySubscription>,
            async_graphql::Request,
        )| async move { Ok::<_, Infallible>(Response::from(schema.execute(request).await)) },
    );

    // GQL playground
    warp::path("gql")
        .and(warp::get())
        .map(|| {
            HttpResponse::builder()
                .header("content-type", "text/html")
                .body(playground_source(GraphQLPlaygroundConfig::new("/gql")))
        })
        // Actual GQL handler
        .or(graphql_post)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn load_playground() {
        let f = build();

        let res = warp::test::request()
            .method("GET")
            .path("/gql")
            .reply(&f)
            .await;

        assert_eq!(res.status(), 200);
    }

    #[tokio::test]
    async fn ping_gql() {
        let f = build();

        let res = warp::test::request()
            .method("POST")
            .header("content-type", "application/json")
            .body(r#"{"operationName":null,"variables":{},"query":"{\n  ping\n}\n"}"#)
            .path("/gql")
            .reply(&f)
            .await;

        println!("{:?}", res.body());

        assert_eq!(res.status(), 200);
    }
}
