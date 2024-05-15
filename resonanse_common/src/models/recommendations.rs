use crate::models::BaseEvent;
use serde::Deserialize;

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
