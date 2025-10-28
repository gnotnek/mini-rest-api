use axum::{
    extract::{Path, State},
    routing,
    Json, Router,
};
use sqlx::SqlitePool;

use crate::error::AppErr;
use super::service;
use super::model::CreateNote;

pub fn routes() -> Router<SqlitePool> {
    // ✅ Explicitly use routing functions so compiler sees them as used
    let get_route = routing::get(get_notes);
    let post_route = routing::post(create_note);
    let put_route = routing::put(update_note);
    let delete_route = routing::delete(delete_note);
    let get_one_route = routing::get(get_note);

    Router::new()
        .route("/", get_route.post(post_route))
        .route("/:id", get_one_route.put(put_route).delete(delete_route))
}

pub async fn get_notes(State(pool): State<SqlitePool>) -> Result<Json<Vec<super::model::Note>>, AppErr> {
    let notes = service::get_all_notes(&pool).await?;
    Ok(Json(notes))
}

pub async fn get_note(Path(id): Path<String>, State(pool): State<SqlitePool>) -> Result<Json<super::model::Note>, AppErr> {
    let note = service::get_note_by_id(&pool, &id).await?;
    Ok(Json(note))
}

pub async fn create_note(State(pool): State<SqlitePool>, Json(payload): Json<CreateNote>) -> Result<Json<super::model::Note>, AppErr> {
    let note = service::create_note(&pool, payload).await?;
    Ok(Json(note))
}

pub async fn update_note(Path(id): Path<String>, State(pool): State<SqlitePool>, Json(payload): Json<CreateNote>) -> Result<Json<super::model::Note>, AppErr> {
    let updated = service::update_note(&pool, &id, payload).await?;
    Ok(Json(updated))
}

pub async fn delete_note(Path(id): Path<String>, State(pool): State<SqlitePool>) -> Result<Json<String>, AppErr> {
    service::delete_note(&pool, &id).await?;
    Ok(Json(format!("Deleted note with id {}", id)))
}
