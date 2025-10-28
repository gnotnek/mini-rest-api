use axum::{
    extract::{Path, State},
    routing,
    Json, Router,
};
use sqlx::SqlitePool;
use tracing::{info, error, instrument};

use crate::error::AppErr;
use super::service;
use super::model::{Note, CreateNote};

pub fn routes() -> Router<SqlitePool> {
    // ✅ Explicitly import route methods so compiler marks them used
    let get_route = routing::get(get_notes);
    let post_route = routing::post(create_note);
    let put_route = routing::put(update_note);
    let delete_route = routing::delete(delete_note);
    let get_one_route = routing::get(get_note);

    Router::new()
        .route("/", get_route.post(post_route))
        .route("/:id", get_one_route.put(put_route).delete(delete_route))
}

/// GET /notes
#[instrument(skip(pool))]
pub async fn get_notes(State(pool): State<SqlitePool>) -> Result<Json<Vec<Note>>, AppErr> {
    info!("Fetching all notes");
    match service::get_all_notes(&pool).await {
        Ok(notes) => {
            info!(count = notes.len(), "Successfully fetched notes");
            Ok(Json(notes))
        }
        Err(err) => {
            error!(error = ?err, "Failed to fetch notes");
            Err(err)
        }
    }
}

/// GET /notes/:id
#[instrument(skip(pool), fields(note_id = %id))]
pub async fn get_note(
    Path(id): Path<String>,
    State(pool): State<SqlitePool>,
) -> Result<Json<Note>, AppErr> {
    info!("Fetching note by ID");
    match service::get_note_by_id(&pool, &id).await {
        Ok(note) => {
            info!("Successfully fetched note");
            Ok(Json(note))
        }
        Err(err) => {
            error!(error = ?err, "Failed to fetch note by ID");
            Err(err)
        }
    }
}

/// POST /notes
#[instrument(skip(pool, payload), fields(title = %payload.title))]
pub async fn create_note(
    State(pool): State<SqlitePool>,
    Json(payload): Json<CreateNote>,
) -> Result<Json<Note>, AppErr> {
    info!("Creating a new note");
    match service::create_note(&pool, payload).await {
        Ok(note) => {
            info!(note_id = %note.id, "Note created successfully");
            Ok(Json(note))
        }
        Err(err) => {
            error!(error = ?err, "Failed to create note");
            Err(err)
        }
    }
}

/// PUT /notes/:id
#[instrument(skip(pool, payload), fields(note_id = %id, title = %payload.title))]
pub async fn update_note(
    Path(id): Path<String>,
    State(pool): State<SqlitePool>,
    Json(payload): Json<CreateNote>,
) -> Result<Json<Note>, AppErr> {
    info!("Updating note");
    match service::update_note(&pool, &id, payload).await {
        Ok(updated) => {
            info!(note_id = %updated.id, "Note updated successfully");
            Ok(Json(updated))
        }
        Err(err) => {
            error!(error = ?err, "Failed to update note");
            Err(err)
        }
    }
}

/// DELETE /notes/:id
#[instrument(skip(pool), fields(note_id = %id))]
pub async fn delete_note(
    Path(id): Path<String>,
    State(pool): State<SqlitePool>,
) -> Result<Json<String>, AppErr> {
    info!("Deleting note");
    match service::delete_note(&pool, &id).await {
        Ok(_) => {
            info!("Note deleted successfully");
            Ok(Json(format!("Deleted note with id {}", id)))
        }
        Err(err) => {
            error!(error = ?err, "Failed to delete note");
            Err(err)
        }
    }
}
