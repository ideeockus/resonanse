use chrono::{Datelike, NaiveDateTime, Timelike};
use time::{OffsetDateTime, PrimitiveDateTime, UtcOffset};

#[allow(unused)]
fn chrono_naive_to_time_offset(naive: NaiveDateTime) -> OffsetDateTime {
    let primitive = PrimitiveDateTime::new(
        time::Date::from_calendar_date(
            naive.year(),
            time::Month::try_from(naive.month() as u8).unwrap(),
            naive.day() as u8,
        )
        .unwrap(),
        time::Time::from_hms_nano(
            naive.hour() as u8,
            naive.minute() as u8,
            naive.second() as u8,
            naive.nanosecond(),
        )
        .unwrap(),
    );

    // Добавление временной зоны (здесь используется UTC, смещение 0)
    let utc_offset = UtcOffset::from_whole_seconds(0).unwrap();
    primitive.assume_offset(utc_offset)
}
