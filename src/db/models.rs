use diesel::Queryable;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub full_name: String,
    pub dob: chrono::NaiveDate,
    pub profile_picture: Option<String>,
    pub active: bool,
    pub password_hash: String,
    pub google_id: Option<String>,
    pub role: String,
}
