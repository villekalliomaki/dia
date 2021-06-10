use actix_cors::Cors;

/// Creates the CORS extension with configuration depending
/// on the build mode. Permissive on debug and default on release.
pub fn create_cors() -> Cors {
    #[cfg(debug_assertions)]
    return Cors::permissive();
    #[cfg(not(debug_assertions))]
    return Cors::default();
}
