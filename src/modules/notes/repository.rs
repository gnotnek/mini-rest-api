use sqlx::SqlitePool;
use crate::{error::AppErr, modules::notes::model::{Note, CreateNote}};

pub async fn get_all(pool: &SqlitePool) -> Result<Vec<Note>, AppErr> {
    let notes = sqlx::query_as::<_, Note>("SELECT * FROM notes")
        .fetch_all(pool)
        .await?;
    Ok(notes)
}

pub async fn get_by_id(pool: &SqlitePool, id: &str) -> Result<Option<Note>, AppErr> {
    let note = sqlx::query_as::<_, Note>("SELECT * FROM notes WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await?;
    Ok(note)
}

pub async fn insert(pool: &SqlitePool, data: CreateNote) -> Result<Note, AppErr> {
    let note = Note::new(data.title, data.content);
    sqlx::query("INSERT INTO notes (id, title, content) VALUES (?, ?, ?)")
        .bind(&note.id)
        .bind(&note.title)
        .bind(&note.content)
        .execute(pool)
        .await?;
    Ok(note)
}

pub async fn update(pool: &SqlitePool, id: &str, data: CreateNote) -> Result<Note, AppErr> {
    let result = sqlx::query("UPDATE notes SET title = ?, content = ? WHERE id = ?")
        .bind(&data.title)
        .bind(&data.content)
        .bind(id)
        .execute(pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppErr::NotFound);
    }

    Ok(Note {
        id: id.to_string(),
        title: data.title,
        content: data.content,
    })
}

pub async fn delete(pool: &SqlitePool, id: &str) -> Result<(), AppErr> {
    let result = sqlx::query("DELETE FROM notes WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppErr::NotFound);
    }

    Ok(())
}
