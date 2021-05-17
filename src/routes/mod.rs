mod gql;

use actix_web::Scope;

pub fn build() -> Scope {
    Scope::new("/api").service(gql::build())
}
