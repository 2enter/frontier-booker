mod claude;
mod config;
mod cron;
mod handlers;
mod routes;
mod state;
mod weather;
mod webdriver;

use crate::handlers::ws::ws_broadcast;
use crate::routes::get_routes;
use config::Config;
use sqlx::postgres::PgPoolOptions;
use state::AppState;
use tokio::net::TcpListener;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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
    let addr = format!("{}:{}", "0.0.0.0", &config.port);

    let app_state = AppState::new(pool, config);
    let app = get_routes(app_state.clone());

    let listener = TcpListener::bind(&addr).await?;

    cron::init(app_state).await?;
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}
