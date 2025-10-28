use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug, Clone)]
pub struct Note {
    pub id: String,
    pub title: String,
    pub content: String,
}

#[derive(Deserialize)]
pub struct CreateNote {
    pub title: String,
    pub content: String,
}

impl Note {
    pub fn new(title: String, content: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            title,
            content,
        }
    }
}
