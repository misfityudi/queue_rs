use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::get,
    Router,
};
use uuid::Uuid;
use crate::models::User;
use crate::db::Db;

// --- CRUD for users ---

pub async fn list_users(State(db): State<Db>) -> impl IntoResponse {
    let query = sqlx::query_as::<_, User>(r#"SELECT * FROM "users""#)
        .fetch_all(&db.pool)
        .await;

    match query {
        Ok(records) => Ok(Json(records)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn create_user(State(db): State<Db>, Json(payload): Json<User>) -> impl IntoResponse {
    let query = sqlx::query_as::<_, User>(
        r#"INSERT INTO "users" ("email", "password_hash", "name") VALUES ($1, $2, $3) RETURNING *"#
    )
    .bind(payload.email)
          .bind(payload.password_hash)
          .bind(payload.name)
    .fetch_one(&db.pool)
    .await;

    match query {
        Ok(record) => Ok((StatusCode::CREATED, Json(record))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn get_user(State(db): State<Db>, Path(id): Path<Uuid>) -> impl IntoResponse {
    let query = sqlx::query_as::<_, User>(r#"SELECT * FROM "users" WHERE id = $1"#)
        .bind(id)
        .fetch_optional(&db.pool)
        .await;

    match query {
        Ok(Some(record)) => Ok(Json(record)),
        Ok(None) => Err((StatusCode::NOT_FOUND, "Record not found".to_string())),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn update_user(State(db): State<Db>, Path(id): Path<Uuid>, Json(payload): Json<User>) -> impl IntoResponse {
    let query = sqlx::query_as::<_, User>(
        r#"UPDATE "users" SET "email" = $1, "password_hash" = $2, "name" = $3 WHERE id = $4 RETURNING *"#
    )
    .bind(payload.email)
          .bind(payload.password_hash)
          .bind(payload.name)
    .bind(id)
    .fetch_one(&db.pool)
    .await;

    match query {
        Ok(record) => Ok(Json(record)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn delete_user(State(db): State<Db>, Path(id): Path<Uuid>) -> impl IntoResponse {
    let result = sqlx::query(r#"DELETE FROM "users" WHERE id = $1"#)
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
        .route("/", get(list_users).post(create_user))
        .route("/:id", get(get_user).put(update_user).delete(delete_user))
}
