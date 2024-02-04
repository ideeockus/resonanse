use sqlx::{FromRow, Row};
use sqlx::postgres::PgRow;
use uuid::Uuid;


#[derive(Debug, Clone)]
#[repr(i64)]
pub enum EventScoreType {
    Like = 1,
    Neutral = 0,
    Dislike = -1,
}

#[derive(Clone, Debug)]
pub struct EventScore {
    pub user_id: i64,
    pub event_id: Uuid,
    pub event_score: EventScoreType,
}

impl FromRow<'_, PgRow> for EventScore {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::error::Error> {
        Ok(Self {
            user_id: row.try_get::<_, &str>("user_id")?,
            event_id: row.try_get::<_, &str>("event_id")?,
            event_score: match row.try_get("event_score")? {
                1 => EventScoreType::Like,
                0 => EventScoreType::Neutral,
                -1 => EventScoreType::Dislike,
                _ => return Err(sqlx::error::Error::RowNotFound),
            },
        })
    }
}