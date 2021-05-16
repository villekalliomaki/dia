use crate::res::Res;
use std::convert::Infallible;
use warp::{Filter, Reply};

pub fn build() -> impl Filter<Extract = impl Reply, Error = Infallible> + Clone {
    warp::any().and_then(not_found)
}

async fn not_found() -> Res<()> {
    ErrRes::<()>::error("Not found.").status_from_u16(404)
}
