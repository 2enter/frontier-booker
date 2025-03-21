use crate::handlers::cargo::*;
use crate::handlers::news::get_news;
use crate::handlers::redirect;
use crate::handlers::sys_info::get_temperature;
use crate::handlers::ws::ws_handler;
use crate::state::AppState;
use axum::extract::DefaultBodyLimit;
use axum::routing::{any, get, post};
use axum::Router;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;

pub fn get_routes(state: AppState) -> Router {
    Router::new()
        .nest(
            "/api",
            Router::new()
                .nest(
                    "/cargo",
                    Router::new()
                        .route("/", post(send_cargo).get(get_cargoes))
                        .route("/{id}", get(get_cargo_by_id))
                        .route("/today", get(get_today_cargoes))
                        .route("/info", post(update_cargo_text_info)),
                )
                .route("/news", get(get_news))
                .route("/sys-temp", get(get_temperature))
                .route("/cargo-info", post(update_cargo_text_info))
                .nest(
                    "/render",
                    Router::new()
                        .route("/news/{num}", get(redirect::news))
                        .route("/info", get(redirect::info))
                        .route("/cctv", get(redirect::cctv)),
                )
                .nest_service(
                    "/storage",
                    ServeDir::new(format!("{}/backend/db/storage", state.config.root_dir)),
                ),
        )
        .route("/ws", any(ws_handler))
        .layer(TraceLayer::new_for_http())
        .route("/zh-tw", get(redirect::app))
        .fallback_service(ServeDir::new(format!(
            "{}/frontend/build",
            state.config.root_dir
        )))
        .layer(DefaultBodyLimit::max(5000000000000))
        .with_state(state)
}
