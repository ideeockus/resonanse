use chrono::{DateTime, TimeZone, Utc};

pub enum EventType {
    OfflineMeetup,
    OneToOne,
    Online,
}

pub struct Location {
    latitude: f64,
    longitude: f64,
}

// pub enum EventSubject {
//     Business,
//     Social,
//     Sport,
//
// }

// todo translation
const EVENT_SUBJECTS: &[&str] = &[
    "Бизнес",
    "Спорт",
    "Благотворительность",
    "Развлечения",
    "Образование",
    "Профессиональное",
    "Знакомства",
    "Культура",
    "Интересы",
    "Другое",
];

pub struct BaseEvent<Tz: TimeZone = Utc> {
    id: u64,
    name: String,
    description: String, // markdown (?)
    date: DateTime<Tz>,
    creator_id: u64,
    event_type: EventType,
    location: Location,
}
