use crate::gql;
use warp::{Filter, Rejection, Reply};

pub fn root() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    ping().or(gql::handlers::build())
}

pub fn ping() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("ping").and(warp::get()).map(|| "pong")
}
