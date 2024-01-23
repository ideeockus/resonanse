use crate::management::BaseManagementState;
use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::prelude::Dialogue;

#[allow(unused)]
pub type ManagementDialogue = Dialogue<BaseManagementState, InMemStorage<BaseManagementState>>;
pub type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;
