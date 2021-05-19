use crate::res::Res;
use actix_web::{web, Scope};

pub fn build() -> Scope {
    Scope::new("/ping").route("", web::get().to(ping))
}

async fn ping() -> Res<()> {
    Res::info("pong", None)
}
