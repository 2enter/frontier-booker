use crate::enums::{CargoStatus, CargoType};
use axum::body::Bytes;
use axum_typed_multipart::TryFromMultipart;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{query_as, FromRow, PgPool, Postgres};
use typeshare::typeshare;
use uuid::Uuid;

#[typeshare]
#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Cargo {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub paint_time: i32,
    pub r#type: CargoType,
    pub status: CargoStatus,
    pub name: Option<String>,
    pub description: Option<String>,
}

#[typeshare]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CargoInput {
    pub paint_time: f32,
    pub r#type: CargoType,
}

#[typeshare]
#[derive(TryFromMultipart, Serialize, std::fmt::Debug)]
#[serde(rename_all = "camelCase")]
#[try_from_multipart(rename_all = "camelCase")]
pub struct CargoRequest {
    pub cargo_type: CargoType,
    pub paint_time: f32,
    #[serde(skip)] // comment this line before generating typeshare types
    pub file: Bytes,
}

#[typeshare]
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CargoTextInfoRequest {
    pub id: Uuid,
    pub name: String,
    pub description: String,
}

impl Cargo {
    pub async fn get_20(pool: &PgPool) -> Vec<Self> {
        sqlx::query_as("SELECT * FROM cargo ORDER BY created_at DESC LIMIT 20")
            .fetch_all(pool)
            .await
            .unwrap_or_default()
    }

    pub async fn set_pending_by_id(
        pool: &PgPool,
        id: Uuid,
        value: bool,
    ) -> Result<&str, sqlx::Error> {
        sqlx::query("UPDATE cargo SET pending = $1 WHERE id = $2")
            .bind(value)
            .bind(id)
            .execute(pool)
            .await
            .map(|_| "Ok")
    }

    pub async fn get_un_docs(pool: &PgPool) -> Vec<Self> {
        sqlx::query_as(
            "SELECT * FROM cargo WHERE name is NULL AND description is NULL AND pending = false",
        )
        .fetch_all(pool)
        .await
        .unwrap_or_default()
    }

    pub async fn get_by_id(pool: &PgPool, id: Uuid) -> Option<Self> {
        sqlx::query_as("SELECT * FROM cargo WHERE id = $1")
            .bind(id)
            .fetch_one(pool)
            .await
            .ok()
    }

    pub async fn get_today(pool: &PgPool) -> Vec<Self> {
        let today_start = Utc::now().date_naive();
        println!("{today_start}");
        sqlx::query_as("SELECT * FROM cargo WHERE created_at > $1")
            .bind(today_start)
            .fetch_all(pool)
            .await
            .unwrap_or_default()
    }

    pub async fn deliver(pool: &PgPool) -> Vec<Self> {
        let target_time = Utc::now() - chrono::Duration::seconds(60);
        sqlx::query_as(
            r#"UPDATE cargo SET status = 'delivered' WHERE status = 'shipping' and created_at < $1 RETURNING id;"#
        )
            .bind(target_time).fetch_all(pool).await.unwrap_or_default()
    }

    pub async fn launch(pool: &PgPool) -> usize {
        sqlx::query_as::<Postgres, Self>(
            "UPDATE cargo SET status = 'launched' WHERE status = 'delivered' RETURNING id;",
        )
        .fetch_all(pool)
        .await
        .unwrap_or_default()
        .len()
    }

    pub async fn update_text_info(
        pool: &PgPool,
        info: CargoTextInfoRequest,
    ) -> Result<String, sqlx::Error> {
        let CargoTextInfoRequest {
            name,
            description,
            id,
        } = &info;

        sqlx::query("UPDATE cargo SET name = $1, description = $2 WHERE id = $3")
            .bind(name)
            .bind(description)
            .bind(id)
            .execute(pool)
            .await
            .map(|_| "ok".to_string())
    }

    pub async fn create(pool: &PgPool, input: CargoInput) -> Self {
        query_as("INSERT INTO cargo (type, paint_time) VALUES ($1, $2) RETURNING *;")
            .bind(input.r#type)
            .bind(input.paint_time)
            .fetch_one(pool)
            .await
            .unwrap()
    }
}
