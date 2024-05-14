use serde::Serialize;
use clickhouse::Row;
use uuid::Uuid;

#[derive(Row, Serialize)]
pub struct UserInteraction {
    pub user_id: i64,
    pub event_id: Uuid,
    pub interaction_type: String,
    pub interaction_dt: chrono::NaiveDateTime,
}