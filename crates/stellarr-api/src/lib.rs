pub mod routes;
pub mod handlers;
pub mod middleware;
pub mod error;
pub mod state;

pub use error::ApiError;
pub use state::AppState;

use actix_web::{web, App, HttpServer};
use actix_cors::Cors;

/// Start the Actix-web HTTP server
pub async fn start_server(state: AppState, bind_address: &str) -> std::io::Result<()> {
    tracing::info!("Starting API server on {}", bind_address);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .app_data(web::Data::new(state.clone()))
            .wrap(cors)
            .wrap(tracing_actix_web::TracingLogger::default())
            .configure(routes::configure)
    })
    .bind(bind_address)?
    .run()
    .await
}
