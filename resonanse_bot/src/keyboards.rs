use std::env;

use teloxide::types::{
    InlineKeyboardButton, InlineKeyboardButtonKind, InlineKeyboardMarkup, WebAppInfo,
    ReplyMarkup, KeyboardMarkup, KeyboardButton,
};
use uuid::Uuid;

use resonanse_common::models::{EventSubject, ResonanseEventKind};
use resonanse_common::EventSubjectFilter;

use crate::config::WEB_APP_URL;

macro_rules! kb_button_from_enum {
    ($s:expr) => {
        InlineKeyboardButton::new(
            t!(&$s.to_string()),
            InlineKeyboardButtonKind::CallbackData($s.to_string()),
        )
    };
}

macro_rules! kb_button_from_str {
    ($s:expr) => {
        InlineKeyboardButton::new(
            t!($s),
            InlineKeyboardButtonKind::CallbackData($s.to_string()),
        )
    };
}

pub fn get_inline_kb_choose_subject() -> InlineKeyboardMarkup {
    let buttons = [
        vec![
            kb_button_from_enum!(EventSubject::Social),
            kb_button_from_enum!(EventSubject::Entertainments),
            kb_button_from_enum!(EventSubject::Charity),
        ],
        vec![
            kb_button_from_enum!(EventSubject::Culture),
            kb_button_from_enum!(EventSubject::Business),
            kb_button_from_enum!(EventSubject::Education),
        ],
        vec![
            kb_button_from_enum!(&EventSubject::Professional),
            kb_button_from_enum!(EventSubject::Sport),
            kb_button_from_enum!(EventSubject::Other),
        ],
    ];

    InlineKeyboardMarkup::new(buttons)
}

pub fn get_inline_kb_choose_event_kind() -> InlineKeyboardMarkup {
    let buttons = [vec![
        kb_button_from_enum!(ResonanseEventKind::Announcement),
        kb_button_from_enum!(ResonanseEventKind::UserOffer),
    ]];

    InlineKeyboardMarkup::new(buttons)
}

#[allow(unused)]
pub const INLINE_WANT_TO_GO_BTN: &str = "keyboards.want_go_to_event_btn";
pub const INLINE_MAP_BTN: &str = "keyboards.event_map_btn";
pub const INLINE_LIKE_EVENT_BTN: &str = "keyboards.like_event_btn";
pub const INLINE_DISLIKE_EVENT_BTN: &str = "keyboards.dislike_event_btn";

pub fn get_inline_kb_event_message(
    event_id: Uuid,
    map_link: Option<String>,
    // want_go_url: String,
) -> InlineKeyboardMarkup {
    let mut buttons = vec![];
    let mut buttons_first_row = vec![];

    let like_btn = InlineKeyboardButton::new(
        t!(INLINE_LIKE_EVENT_BTN),
        InlineKeyboardButtonKind::CallbackData(format!("{}{}", INLINE_LIKE_EVENT_BTN, event_id)),
    );
    let dislike_btn = InlineKeyboardButton::new(
        t!(INLINE_DISLIKE_EVENT_BTN),
        InlineKeyboardButtonKind::CallbackData(format!("{}{}", INLINE_DISLIKE_EVENT_BTN, event_id)),
    );

    buttons_first_row.push(like_btn);
    if let Some(map_link) = map_link {
        let map_link_btn = InlineKeyboardButton::new(
            t!(INLINE_MAP_BTN),
            InlineKeyboardButtonKind::Url(map_link.parse().unwrap()),
        );

        buttons_first_row.push(map_link_btn);
    }
    buttons_first_row.push(dislike_btn);
    buttons.push(buttons_first_row);

    InlineKeyboardMarkup::new(buttons)
}

pub const EVENTS_PAGE_LEFT: &str = "EVENTS_PAGE_LEFT";
pub const EVENTS_PAGE_RIGHT: &str = "EVENTS_PAGE_RIGHT";

pub fn get_inline_kb_events_page() -> InlineKeyboardMarkup {
    let button_left = InlineKeyboardButton::new(
        t!("event_page.turn_left"),
        InlineKeyboardButtonKind::CallbackData(EVENTS_PAGE_LEFT.to_string()),
    );

    let button_right = InlineKeyboardButton::new(
        t!("event_page.turn_right"),
        InlineKeyboardButtonKind::CallbackData(EVENTS_PAGE_RIGHT.to_string()),
    );

    let buttons = [vec![button_left, button_right]];

    InlineKeyboardMarkup::new(buttons)
}

pub fn get_inline_kb_run_web_app() -> InlineKeyboardMarkup {
    let web_app_url = url::Url::parse(env::var(WEB_APP_URL).unwrap().as_str()).unwrap();

    let web_app_btn = InlineKeyboardButton::new(
        t!("web_app.run_app"),
        InlineKeyboardButtonKind::WebApp(WebAppInfo { url: web_app_url }),
    );

    let buttons = [[web_app_btn]];

    InlineKeyboardMarkup::new(buttons)
}

pub const APPLY_EVENT_FILTER_BTN: &str = "APPLY_EVENT_FILTER_BTN";

pub fn get_inline_kb_set_subject_filter(
    event_filters: &EventSubjectFilter,
) -> InlineKeyboardMarkup {
    const FILTER_ON: &str = "✅";
    const FILTER_OFF: &str = "❌";
    const ROW_LEN: usize = 2;

    let max_len = event_filters
        .get_filters()
        .keys()
        .map(|es| es.to_string().len())
        .max()
        .unwrap_or(0);

    let mut buttons = event_filters
        .get_filters()
        .iter()
        .map(|(es, on)| {
            InlineKeyboardButton::new(
                format!(
                    "{}{}[{}]",
                    t!(&es.to_string()),
                    " ".repeat(max_len.saturating_sub(es.to_string().len())),
                    if *on { FILTER_ON } else { FILTER_OFF },
                ),
                InlineKeyboardButtonKind::CallbackData(es.to_string()),
            )
        })
        .collect::<Vec<_>>()
        .chunks(ROW_LEN)
        .map(|c| c.to_vec())
        .collect::<Vec<_>>();

    let apply_button = InlineKeyboardButton::new(
        "Показать",
        InlineKeyboardButtonKind::CallbackData(APPLY_EVENT_FILTER_BTN.to_string()),
    );
    buttons.push(vec![apply_button]);

    InlineKeyboardMarkup::new(buttons)
}

pub const FILL_EVENT_TITLE_BTN_ID: &str = "keyboards.fill_event.title_btn";
pub const FILL_EVENT_SUBJECT_BTN_ID: &str = "keyboards.fill_event.subject_btn";
pub const FILL_EVENT_DESCRIPTION_BTN_ID: &str = "keyboards.fill_event.description_btn";
// const FILL_EVENT_DATETIME_BTN_ID: &str = "fill_event.datetime";
pub const FILL_EVENT_DATETIME_FROM_BTN_ID: &str = "keyboards.fill_event.datetime_from";
pub const FILL_EVENT_DATETIME_TO_BTN_ID: &str = "keyboards.fill_event.datetime_to";

pub const FILL_EVENT_LOCATION_GEO_BTN_ID: &str = "keyboards.fill_event.location_geo";
pub const FILL_EVENT_LOCATION_TITLE_BTN_ID: &str = "keyboards.fill_event.location_title";

pub const FILL_EVENT_PICTURE_BTN_ID: &str = "keyboards.fill_event.picture";
pub const FILL_EVENT_CONTACT_BTN_ID: &str = "keyboards.fill_event.contact_data";
pub const FILL_EVENT_KIND_BTN_ID: &str = "keyboards.fill_event.kind";

pub const FILL_EVENT_FINALIZE_BTN_ID: &str = "keyboards.fill_event.finalize";

pub fn get_make_event_keyboard() -> InlineKeyboardMarkup {
    let buttons = [
        vec![
            kb_button_from_str!(FILL_EVENT_TITLE_BTN_ID),
            // kb_button_from_str!(FILL_EVENT_SUBJECT_BTN_ID),
            kb_button_from_str!(FILL_EVENT_DESCRIPTION_BTN_ID),
        ],
        vec![
            kb_button_from_str!(FILL_EVENT_DATETIME_FROM_BTN_ID),
            kb_button_from_str!(FILL_EVENT_DATETIME_TO_BTN_ID),
        ],
        vec![
            kb_button_from_str!(FILL_EVENT_LOCATION_TITLE_BTN_ID),
            kb_button_from_str!(FILL_EVENT_LOCATION_GEO_BTN_ID),
        ],
        vec![
            kb_button_from_str!(FILL_EVENT_PICTURE_BTN_ID),
            kb_button_from_str!(FILL_EVENT_CONTACT_BTN_ID),
            kb_button_from_str!(FILL_EVENT_KIND_BTN_ID),
        ],
        vec![kb_button_from_str!(FILL_EVENT_FINALIZE_BTN_ID)],
    ];

    InlineKeyboardMarkup::new(buttons)
}

pub fn make_cities_keyboard(mut cities: Vec<String>) -> ReplyMarkup {
    cities.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
    let buttons: Vec<Vec<KeyboardButton>> = cities
        .into_iter()
        .map(|name| vec![KeyboardButton::new(name)])
        .collect();

    let keyboard = KeyboardMarkup::new(buttons)
        .one_time_keyboard(true)
        .resize_keyboard(true);
    ReplyMarkup::Keyboard(keyboard)
}
