mod claude;
mod config;
mod cron;
mod handlers;
mod routes;
mod state;
mod weather;
mod webdriver;

use std::net::SocketAddr;
use std::path::PathBuf;

use crate::handlers::ws::ws_broadcast;
use crate::routes::get_routes;
use axum_server::tls_rustls::RustlsConfig;
use config::Config;
use sqlx::postgres::PgPoolOptions;
use state::AppState;
// use tokio::net::TcpListener;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    rustls::crypto::ring::default_provider()
        .install_default()
        .expect("no crypto provider");

    tracing_subscriber::fmt()
        // .with_max_level(tracing::Level::DEBUG)
        .with_env_filter(
            EnvFilter::from_default_env()
                .add_directive("backend=debug".parse().unwrap())
                .add_directive("sqlx=debug".parse().unwrap())
                .add_directive("tokio_cron_scheduler=debug".parse().unwrap())
                .add_directive("tower_http=debug".parse().unwrap()),
        )
        .init();

    let config = Config::init();

    let pool = PgPoolOptions::new().connect(&config.database_url).await?;
    // let addr = format!("{}:{}", "0.0.0.0", &config.port);

    let app_state = AppState::new(pool, config.clone());
    let app = get_routes(app_state.clone());

    // let listener = TcpListener::bind(&addr).await?;

    let tls_config = RustlsConfig::from_pem_file(
        PathBuf::from(&config.root_dir)
            .join("backend")
            .join("ssl-pem")
            .join("cert.pem"),
        PathBuf::from(&config.root_dir)
            .join("backend")
            .join("ssl-pem")
            .join("key.pem"),
    )
    .await
    .unwrap();

    let socket_addr = SocketAddr::from(([0, 0, 0, 0], config.port.into()));

    cron::init(app_state).await?;

    // axum::serve(listener, app.into_make_service()).await?;
    axum_server::bind_rustls(socket_addr, tls_config)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
