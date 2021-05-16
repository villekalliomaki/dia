use crate::res::Res;
use std::convert::Infallible;
use warp::{Filter, Rejection, Reply};

pub fn build() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path("health").and(warp::get()).and_then(health)
}

async fn health() -> Result<impl warp::Reply, Infallible> {
    Ok(Res::ok("Server is online.", 1))
}
