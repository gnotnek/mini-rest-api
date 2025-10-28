use sqlx::SqlitePool;
use crate::error::AppErr;
use super::repository;
use super::model::{Note, CreateNote};

pub async fn get_all_notes(pool: &SqlitePool) -> Result<Vec<Note>, AppErr> {
    repository::get_all(pool).await
}

pub async fn get_note_by_id(pool: &SqlitePool, id: &str) -> Result<Note, AppErr> {
    repository::get_by_id(pool, id)
        .await?
        .ok_or(AppErr::NotFound)
}

pub async fn create_note(pool: &SqlitePool, data: CreateNote) -> Result<Note, AppErr> {
    repository::insert(pool, data).await
}

pub async fn update_note(pool: &SqlitePool, id: &str, data: CreateNote) -> Result<Note, AppErr> {
    repository::update(pool, id, data).await
}

pub async fn delete_note(pool: &SqlitePool, id: &str) -> Result<(), AppErr> {
    repository::delete(pool, id).await
}
