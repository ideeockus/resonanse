use std::collections::HashMap;

use log::debug;
use sqlx::{PgPool, Result};
use uuid::Uuid;

use crate::models::{BaseEvent, EventSubject};
use crate::EventSubjectFilter;

// #[derive(Clone)]
// pub struct CreateBaseEvent {
//     pub is_private: bool,
//     pub is_commercial: bool,
//     pub event_kind: ResonanseEventKind,
//     pub title: String,
//     pub description: String,
//     pub brief_description: Option<String>,
//     pub subject: EventSubject,
//     pub datetime_from: NaiveDateTime,
//     pub datetime_to: Option<NaiveDateTime>,
//     // pub timezone: chrono_tz::Tz,
//     pub location: Option<Location>,
//     pub location_title: Option<String>,
//     pub creator_id: i64,
//     pub event_type: EventType,
//     pub picture: Option<Uuid>,
//     pub contact_info: Option<String>,
// }

#[derive(Debug)]
pub struct EventsRepository {
    db_pool: PgPool,
}

impl EventsRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { db_pool: pool }
    }

    pub async fn create_event(&self, event: BaseEvent) -> Result<BaseEvent> {
        // fill all event except id nad creation_time
        let created_event: BaseEvent = sqlx::query_as(
            r#"insert into resonanse_events
            (
            id, is_private, is_commercial, event_kind, title, description, brief_description,
            subject, datetime_from, datetime_to, location_latitude, location_longitude,
            location_title, creator_id, event_type, picture, contact_info
            )
            values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17)
            returning *
            "#,
        )
        .bind(Uuid::new_v4())
        .bind(event.is_private)
        .bind(event.is_commercial)
        .bind(event.event_kind)
        .bind(event.title)
        .bind(event.description)
        .bind(event.brief_description)
        .bind(event.subject as i32)
        .bind(event.datetime_from)
        .bind(event.datetime_to)
        .bind(event.location.as_ref().map(|geo| geo.latitude))
        .bind(event.location.as_ref().map(|geo| geo.longitude))
        .bind(event.location_title)
        .bind(event.creator_id)
        .bind(event.event_type)
        .bind(event.picture)
        .bind(event.contact_info)
        .fetch_one(&self.db_pool)
        .await?;

        Ok(created_event)
    }

    // pub async fn edit_event(&self, event: CreateBaseEvent, event_uuid: Uuid) -> Result<BaseEvent> {
    //     let created_event: BaseEvent = sqlx::query_as(
    //         r#"insert into resonanse_events
    //         (
    //         id, is_private, is_commercial, title, description, subject,
    //         datetime, location_latitude, location_longitude,
    //         location_title, creator_id, event_type, picture, contact_info
    //         )
    //         values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
    //         returning *
    //         "#,
    //     )
    //         .bind(Uuid::new_v4())
    //         .bind(event.is_private)
    //         .bind(event.is_commercial)
    //         .bind(event.title)
    //         .bind(event.description)
    //         .bind(event.subject as i32)
    //         .bind(event.datetime)
    //         .bind(event.location.latitude)
    //         .bind(event.location.longitude)
    //         .bind(event.location.title)
    //         .bind(event.creator_id)
    //         .bind(event.event_type)
    //         .bind(event.picture)
    //         .bind(event.contact_info)
    //         .fetch_one(&self.db_pool)
    //         .await?;
    //
    //     Ok(created_event)
    // }

    pub async fn get_all_events(&self) -> Result<Vec<BaseEvent>> {
        let events: Result<Vec<BaseEvent>> = sqlx::query_as(
            r#"select *
            from resonanse_events
            where datetime_from >= current_date
            order by datetime_from
            "#,
        )
        .fetch_all(&self.db_pool)
        .await;

        events
    }

    pub async fn get_events_by_title_substr(&self, title: &str) -> Result<Vec<BaseEvent>> {
        let events: Result<Vec<BaseEvent>> = sqlx::query_as(
            r#"select *
            from resonanse_events
            where title like $1
            "#,
        )
        .bind(format!("%{}%", title))
        .fetch_all(&self.db_pool)
        .await;

        events
    }

    pub async fn get_all_public_events(&self, page: i64, page_size: i64) -> Result<Vec<BaseEvent>> {
        let events: Result<Vec<BaseEvent>> = sqlx::query_as(
            r#"select *
            from resonanse_events
            where is_private=false and datetime_from >= current_date
            order by datetime_from
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

        if filters_vec.is_empty() {
            return Ok(Vec::new());
        }

        let filter_params_len = filters_vec.len();
        let filter_params = (1..=filter_params_len)
            .map(|i| format!("${}", i))
            .collect::<Vec<String>>()
            .join(", ");
        let query_str = format!(
            r#"select *
            from resonanse_events
            WHERE subject IN ( { } ) and is_private=false and datetime_from >= current_date
            order by datetime_from
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

    pub async fn delete_event(&self, event_uuid: Uuid, _deleted_by_id: i64) -> Result<()> {
        let deleting_event = self.get_event_by_uuid(event_uuid).await?;

        let _deleted_event: BaseEvent = sqlx::query_as(
            r#"insert into deleted_events
            (
            id, is_private, is_commercial, event_kind, title, description, brief_description,
            subject, datetime_from, datetime_to, location_latitude, location_longitude,
            location_title, creator_id, event_type, picture, contact_info
            )
            values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17)
            returning *
            "#,
        )
        .bind(deleting_event.id)
        .bind(deleting_event.is_private)
        .bind(deleting_event.is_commercial)
        .bind(deleting_event.event_kind)
        .bind(deleting_event.title)
        .bind(deleting_event.description)
        .bind(deleting_event.brief_description)
        .bind(deleting_event.subject as i32)
        .bind(deleting_event.datetime_from)
        .bind(deleting_event.datetime_to)
        .bind(deleting_event.location.as_ref().map(|geo| geo.latitude))
        .bind(deleting_event.location.as_ref().map(|geo| geo.longitude))
        .bind(deleting_event.location_title)
        .bind(deleting_event.creator_id)
        .bind(deleting_event.event_type)
        .bind(deleting_event.picture)
        .bind(deleting_event.contact_info)
        .fetch_one(&self.db_pool)
        .await?;

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

    pub async fn count_events_by_subject(&self) -> Result<HashMap<EventSubject, i64>> {
        let _result = sqlx::query(
            r#"select subject, count(*)
            from resonanse_events
            group by subject
            "#,
        )
        .fetch_all(&self.db_pool)
        .await;

        Ok(HashMap::new())
    }
}
