pub enum EventType {
    OfflineMeetup,
    OneToOne,
    Online,
}

pub struct Location {
    latitude: f64,
    longitude: f64,
}

// pub struct BaseEvent<Tz = Utc> {
//     id: u64,
//     name: String,
//     description: String, // markdown (?)
//     date: DateTime<Tz>,
//     creator_id: u64,
//     event_type: EventType,
//     location: Location,
// }
