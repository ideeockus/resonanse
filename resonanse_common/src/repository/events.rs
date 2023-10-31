use crate::models::{BaseEvent, EventSubject, EventType, Location};
use crate::EventSubjectFilter;
use chrono::NaiveDateTime;
use log::debug;
use sqlx::{query, PgPool, Result};
use uuid::Uuid;

#[derive(Clone)]
pub struct CreateBaseEvent {
    pub is_private: bool,
    pub is_commercial: bool,
    pub title: String,
    pub description: String,
    pub subject: EventSubject,
    pub datetime: NaiveDateTime,
    // pub timezone: chrono_tz::Tz,
    pub location: Location,
    pub creator_id: i64,
    pub event_type: EventType,
    pub picture: Option<Uuid>,
    pub contact_info: Option<String>,
}

#[derive(Debug)]
pub struct EventsRepository {
    db_pool: PgPool,
}

impl EventsRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { db_pool: pool }
    }

    pub async fn create_event(&self, event: CreateBaseEvent) -> Result<BaseEvent> {
        // let created_event = query!(
        //     r#"insert into resonanse_events
        //     (
        //     id, is_private, is_commercial, title, description, subject,
        //     datetime, location_latitude, location_longitude,
        //     location_title, creator_id, event_type, picture, contact_info
        //     )
        //     values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
        //     returning is_private, is_commercial, title, description,
        //     datetime, location_latitude, location_longitude,
        //     location_title, creator_id, picture, id, creation_time,
        //     event_type as "event_type: EventType", subject as "subject: EventSubject",
        //     contact_info
        //     "#,
        //     Uuid::new_v4(),
        //     event.is_private,
        //     event.is_commercial,
        //     event.title,
        //     event.description,
        //     event.subject as i32,
        //     event.datetime,
        //     // event.timezone,
        //     event.location.latitude,
        //     event.location.longitude,
        //     event.location.title,
        //     event.creator_id,
        //     event.event_type as i32,
        //     event.picture,
        // )
        let created_event: BaseEvent = sqlx::query_as(
            r#"insert into resonanse_events
            (
            id, is_private, is_commercial, title, description, subject,
            datetime, location_latitude, location_longitude,
            location_title, creator_id, event_type, picture, contact_info
            )
            values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
            returning *
            "#,
        )
            .bind(Uuid::new_v4())
            .bind(event.is_private)
            .bind(event.is_commercial)
            .bind(event.title)
            .bind(event.description)
            .bind(event.subject as i32)
            .bind(event.datetime)
            .bind(event.location.latitude)
            .bind(event.location.longitude)
            .bind(event.location.title)
            .bind(event.creator_id)
            .bind(event.event_type)
            .bind(event.picture)
            .bind(event.contact_info)
            .fetch_one(&self.db_pool)
            .await?;

        // Ok(BaseEvent {
        //     id: created_event.id,
        //     is_private: created_event.is_private,
        //     is_commercial: created_event.is_commercial,
        //     title: created_event.title,
        //     description: created_event.description,
        //     subject: created_event.subject,
        //     datetime: created_event.datetime,
        //     location: Location {
        //         latitude: created_event.location_latitude,
        //         longitude: created_event.location_longitude,
        //         title: created_event.location_title,
        //     },
        //     creator_id: created_event.creator_id,
        //     event_type: created_event.event_type,
        //     picture: created_event.picture,
        //     creation_time: created_event.creation_time,
        // })
        Ok(created_event)
    }

    pub async fn get_all_public_events(&self, page: i64, page_size: i64) -> Result<Vec<BaseEvent>> {
        let events: Result<Vec<BaseEvent>> = sqlx::query_as(
            r#"select *
            from resonanse_events
            where is_private=false and datetime >= current_date
            order by datetime
            offset $1 rows
            fetch next $2 rows only
            "#,
        )
            .bind(page * page_size)
            .bind(page_size)
            .fetch_all(&self.db_pool)
            .await;

        events
    }

    pub async fn get_public_events(
        &self,
        page: i64,
        page_size: i64,
        events_subject_filter: &EventSubjectFilter,
    ) -> Result<Vec<BaseEvent>> {
        let filters_vec = events_subject_filter
            .0
            .iter()
            .filter(|(_, f)| **f)
            .map(|(f, _)| (*f as i32))
            .collect::<Vec<_>>();
        // let filter_params = filters_vec.iter().map(|f| f.to_string()).collect::<Vec<_>>().join("");

        if filters_vec.is_empty() {
            return Ok(Vec::new());
        }

        let filter_params_len = filters_vec.len();
        let filter_params = (1..=filter_params_len)
            .map(|i| format!("${}", i))
            .collect::<Vec<String>>()
            .join(", ");
        // let filter_params = format!("$1{}", ", $".repeat(filters_vec.len() - 1));
        let query_str = format!(
            r#"select *
            from resonanse_events
            WHERE subject IN ( { } ) and is_private=false and datetime >= current_date
            order by datetime
            offset ${} rows
            fetch next ${} rows only
            "#,
            filter_params,
            filter_params_len + 1,
            filter_params_len + 2,
        );
        debug!("get_public_events builded query: {}", query_str);

        let mut events_query = sqlx::query_as(&query_str);
        for subj_i32 in filters_vec {
            events_query = events_query.bind(subj_i32);
        }

        let events: Result<Vec<BaseEvent>> = events_query
            .bind(page * page_size)
            .bind(page_size)
            .fetch_all(&self.db_pool)
            .await;
        events
    }

    pub async fn get_event_by_uuid(&self, uuid: Uuid) -> Result<BaseEvent> {
        let event: Result<BaseEvent> = sqlx::query_as(
            r#"select *
            from resonanse_events
            where id=$1
            "#,
        )
            .bind(uuid)
            .fetch_one(&self.db_pool)
            .await;

        event
    }

    pub async fn delete_events(&self, event_uuid: Uuid) -> Result<()> {
        let result = sqlx::query(
            r#"
            delete from resonanse_events
            where id=$1
            "#,
        )
            .bind(event_uuid)
            .execute(&self.db_pool)
            .await?;

        debug!("delete_events result {:?}", result);

        Ok(())
    }

    pub async fn create_event_tg_binding(&self, post_id: i64, event_id: Uuid) -> Result<()> {
        let result = sqlx::query(
            r#"insert into event_tg_table
            (post_id, creator_id)
            values ($1, $2)
            "#,
        )
            .bind(post_id)
            .bind(event_id)
            .execute(&self.db_pool)
            .await?;
        debug!("event_tg_table result {:?}", result);

        Ok(())
    }
}
