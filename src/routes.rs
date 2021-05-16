use crate::{gql, handlers};
use warp::{Filter, Rejection, Reply};

pub fn root() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    handlers::health::build()
        .or(gql::handlers::build())
        .or(handlers::not_found::build())
        .with(warp::cors().allow_any_origin())
}
