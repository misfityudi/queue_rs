use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::get,
    Router,
};
use uuid::Uuid;
use crate::models::Blog;
use crate::db::Db;

// --- CRUD for blogs ---

pub async fn list_blogs(State(db): State<Db>) -> impl IntoResponse {
    let query = sqlx::query_as::<_, Blog>(r#"SELECT * FROM "blogs""#)
        .fetch_all(&db.pool)
        .await;

    match query {
        Ok(records) => Ok(Json(records)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn create_blog(State(db): State<Db>, Json(payload): Json<Blog>) -> impl IntoResponse {
    let query = sqlx::query_as::<_, Blog>(
        r#"INSERT INTO "blogs" ("title", "published") VALUES ($1, $2) RETURNING *"#
    )
    .bind(payload.title)
          .bind(payload.published)
    .fetch_one(&db.pool)
    .await;

    match query {
        Ok(record) => Ok((StatusCode::CREATED, Json(record))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn get_blog(State(db): State<Db>, Path(id): Path<Uuid>) -> impl IntoResponse {
    let query = sqlx::query_as::<_, Blog>(r#"SELECT * FROM "blogs" WHERE id = $1"#)
        .bind(id)
        .fetch_optional(&db.pool)
        .await;

    match query {
        Ok(Some(record)) => Ok(Json(record)),
        Ok(None) => Err((StatusCode::NOT_FOUND, "Record not found".to_string())),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn update_blog(State(db): State<Db>, Path(id): Path<Uuid>, Json(payload): Json<Blog>) -> impl IntoResponse {
    let query = sqlx::query_as::<_, Blog>(
        r#"UPDATE "blogs" SET "title" = $1, "published" = $2 WHERE id = $3 RETURNING *"#
    )
    .bind(payload.title)
          .bind(payload.published)
    .bind(id)
    .fetch_one(&db.pool)
    .await;

    match query {
        Ok(record) => Ok(Json(record)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn delete_blog(State(db): State<Db>, Path(id): Path<Uuid>) -> impl IntoResponse {
    let result = sqlx::query(r#"DELETE FROM "blogs" WHERE id = $1"#)
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
        .route("/", get(list_blogs).post(create_blog))
        .route("/:id", get(get_blog).put(update_blog).delete(delete_blog))
}
