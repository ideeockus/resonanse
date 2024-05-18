use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum RecSubsystem {
    Basic,
    Dynamic,
    Collaborative,
}

#[derive(Deserialize, Debug)]
pub struct SimplifiedRecItem {
    pub subsystem: RecSubsystem,
    pub event_id: Uuid,
    pub score: f32,
}

// #[derive(Deserialize, Debug)]
// pub struct RecItem {
//     subsystem: RecSubsystem,
//     event: BaseEvent,
//     score: f32,
// }
