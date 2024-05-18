use uuid::Uuid;

pub enum MyComplexCommand {
    GetEventIntId(i64),
    GetEventUuid(Uuid),
}
