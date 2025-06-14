use serde::{Deserialize, Serialize};
use sqlx::{FromRow};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Feedback {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub feedback: String,
    pub rating: i32,
    pub status: Option<String>,
    #[serde(rename = "createdAt ")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}