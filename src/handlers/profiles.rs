use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::get,
    Router,
};
use uuid::Uuid;
use crate::models::Profile;
use crate::db::Db;

// --- CRUD for profiles ---

pub async fn list_profiles(State(db): State<Db>) -> impl IntoResponse {
    let query = sqlx::query_as::<_, Profile>(r#"SELECT * FROM "profiles""#)
        .fetch_all(&db.pool)
        .await;

    match query {
        Ok(records) => Ok(Json(records)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn create_profile(State(db): State<Db>, Json(payload): Json<Profile>) -> impl IntoResponse {
    let query = sqlx::query_as::<_, Profile>(
        r#"INSERT INTO "profiles" ("user_id", "bio", "avatar_url") VALUES ($1, $2, $3) RETURNING *"#
    )
    .bind(payload.user_id)
          .bind(payload.bio)
          .bind(payload.avatar_url)
    .fetch_one(&db.pool)
    .await;

    match query {
        Ok(record) => Ok((StatusCode::CREATED, Json(record))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn get_profile(State(db): State<Db>, Path(id): Path<Uuid>) -> impl IntoResponse {
    let query = sqlx::query_as::<_, Profile>(r#"SELECT * FROM "profiles" WHERE id = $1"#)
        .bind(id)
        .fetch_optional(&db.pool)
        .await;

    match query {
        Ok(Some(record)) => Ok(Json(record)),
        Ok(None) => Err((StatusCode::NOT_FOUND, "Record not found".to_string())),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn update_profile(State(db): State<Db>, Path(id): Path<Uuid>, Json(payload): Json<Profile>) -> impl IntoResponse {
    let query = sqlx::query_as::<_, Profile>(
        r#"UPDATE "profiles" SET "user_id" = $1, "bio" = $2, "avatar_url" = $3 WHERE id = $4 RETURNING *"#
    )
    .bind(payload.user_id)
          .bind(payload.bio)
          .bind(payload.avatar_url)
    .bind(id)
    .fetch_one(&db.pool)
    .await;

    match query {
        Ok(record) => Ok(Json(record)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn delete_profile(State(db): State<Db>, Path(id): Path<Uuid>) -> impl IntoResponse {
    let result = sqlx::query(r#"DELETE FROM "profiles" WHERE id = $1"#)
        .bind(id)
        .execute(&db.pool)
        .await;

    match result {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub fn routes() -> Router<Db> {
    Router::new()
        .route("/", get(list_profiles).post(create_profile))
        .route("/:id", get(get_profile).put(update_profile).delete(delete_profile))
}
