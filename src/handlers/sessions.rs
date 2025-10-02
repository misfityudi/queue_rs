use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::get,
    Router,
};
use uuid::Uuid;
use crate::models::Session;
use crate::db::Db;

// --- CRUD for sessions ---

pub async fn list_sessions(State(db): State<Db>) -> impl IntoResponse {
    let query = sqlx::query_as::<_, Session>(r#"SELECT * FROM "sessions""#)
        .fetch_all(&db.pool)
        .await;

    match query {
        Ok(records) => Ok(Json(records)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn create_session(State(db): State<Db>, Json(payload): Json<Session>) -> impl IntoResponse {
    let query = sqlx::query_as::<_, Session>(
        r#"INSERT INTO "sessions" ("user_id", "expires_at") VALUES ($1, $2) RETURNING *"#
    )
    .bind(payload.user_id)
          .bind(payload.expires_at)
    .fetch_one(&db.pool)
    .await;

    match query {
        Ok(record) => Ok((StatusCode::CREATED, Json(record))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn get_session(State(db): State<Db>, Path(id): Path<Uuid>) -> impl IntoResponse {
    let query = sqlx::query_as::<_, Session>(r#"SELECT * FROM "sessions" WHERE id = $1"#)
        .bind(id)
        .fetch_optional(&db.pool)
        .await;

    match query {
        Ok(Some(record)) => Ok(Json(record)),
        Ok(None) => Err((StatusCode::NOT_FOUND, "Record not found".to_string())),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn update_session(State(db): State<Db>, Path(id): Path<Uuid>, Json(payload): Json<Session>) -> impl IntoResponse {
    let query = sqlx::query_as::<_, Session>(
        r#"UPDATE "sessions" SET "user_id" = $1, "expires_at" = $2 WHERE id = $3 RETURNING *"#
    )
    .bind(payload.user_id)
          .bind(payload.expires_at)
    .bind(id)
    .fetch_one(&db.pool)
    .await;

    match query {
        Ok(record) => Ok(Json(record)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn delete_session(State(db): State<Db>, Path(id): Path<Uuid>) -> impl IntoResponse {
    let result = sqlx::query(r#"DELETE FROM "sessions" WHERE id = $1"#)
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
        .route("/", get(list_sessions).post(create_session))
        .route("/:id", get(get_session).put(update_session).delete(delete_session))
}
