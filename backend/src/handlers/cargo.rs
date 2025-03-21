use crate::config::Config;
use crate::handlers::ws::ws_broadcast;
use crate::state::AppState;
use axum::extract::{Json, Path, State};
use axum_typed_multipart::BaseMultipart;
use model::cargo::*;
use model::util::{ApiError, ApiResponse};
use model::ws_msg::WSMsg;
use reqwest::StatusCode;
use sqlx::types::Uuid;
use utils::texture::generate_texture;

pub async fn get_cargoes(State(app_state): State<AppState>) -> Json<ApiResponse<Vec<Cargo>>> {
    ApiResponse::new_success(Cargo::get_20(&app_state.pool).await).into()
}

pub async fn get_cargo_by_id(
    State(app_state): State<AppState>,
    Path(id_str): Path<String>,
) -> Json<ApiResponse<Cargo>> {
    let id = Uuid::parse_str(&id_str);
    if let Err(error) = id {
        return ApiResponse::new_error_with_details(
            StatusCode::BAD_REQUEST,
            "Invalid UUID".to_owned(),
            Some(error.to_string()),
        )
        .into();
    } else {
        if let Some(cargo) = Cargo::get_by_id(&app_state.pool, id.unwrap()).await {
            ApiResponse::new_success(cargo).into()
        } else {
            ApiResponse::new_error(StatusCode::NOT_FOUND).into()
        }
    }
}

pub async fn get_today_cargoes(State(app_state): State<AppState>) -> Json<ApiResponse<Vec<Cargo>>> {
    ApiResponse::new_success(Cargo::get_today(&app_state.pool).await).into()
}

pub async fn update_cargo_text_info(
    State(app_state): State<AppState>,
    Json(info): Json<CargoTextInfoRequest>,
) -> Json<ApiResponse<String>> {
    if let Err(error) = Cargo::update_text_info(&app_state.pool, info).await {
        ApiResponse::new_error_with_details(StatusCode::NOT_FOUND, error.to_string(), None).into()
    } else {
        ApiResponse::new_success("ok".to_string()).into()
    }
}

pub async fn send_cargo(
    State(app_state): State<AppState>,
    data: BaseMultipart<CargoRequest, ApiError>,
) -> Json<ApiResponse<Cargo>> {
    let CargoRequest {
        paint_time,
        cargo_type,
        file,
    } = data.data;

    let cargo = Cargo::create(
        &app_state.pool,
        CargoInput {
            paint_time,
            r#type: cargo_type.clone(),
        },
    )
    .await;

    let id = &cargo.id.to_string();

    let path = format!("{}/backend/db/storage", app_state.config.root_dir);

    generate_texture(id, &file, &path);

    let Config { host, port, .. } = app_state.config;

    ws_broadcast(
        WSMsg::cargo(
            cargo_type,
            id,
            &format!("http://{host}:{port}/api/storage/texture/{id}.jpg"),
        ),
        &app_state.ws_sender.clone(),
    );

    ApiResponse::new_success(cargo).into()
}
