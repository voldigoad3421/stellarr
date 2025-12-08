use actix_web::web;

pub mod movies;
pub mod tv_shows;
pub mod health;
pub mod search;
pub mod indexers;
pub mod downloads;
pub mod system;
pub mod static_pages;

/// Configure all API routes
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .configure(health::configure)
            .configure(movies::configure)
            .configure(tv_shows::configure)
            .configure(search::configure)
            .configure(indexers::configure)
            .configure(downloads::configure)
            .configure(system::configure),
    )
    .configure(static_pages::configure);
}
