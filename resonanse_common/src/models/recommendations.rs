use serde::Deserialize;
use crate::models::BaseEvent;

#[derive(Deserialize, Debug)]
pub enum RecSubsystem {
    Basic,
    Dynamic,
    Collaborative,
}

#[derive(Deserialize, Debug)]
pub struct RecItem {
    subsystem: RecSubsystem,
    event: BaseEvent,
    score: f32,
}
