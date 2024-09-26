use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;


#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ShortUrlModel {
    pub id: i32,              
    pub short_url: String,       
    pub original_url: String,            
    pub created_on: DateTime<Utc>,   
    pub user_id: Uuid
}