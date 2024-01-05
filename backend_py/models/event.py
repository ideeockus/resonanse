from typing import List, Optional
from pydantic import BaseModel


class Limits(BaseModel):
    event_limit: Optional[int]
    distance_from_user: Optional[float]
    attendance_confirmation_days_before: Optional[int]
    age_limit: Optional[int]


class UserStatus(BaseModel):
    visited_users: List[str]
    not_attended_users: List[str]


class Event(BaseModel):
    title: str
    description: str
    short_description: str
    category: str
    location: str
    start_date: str
    end_date: str
    online: bool
    event_marked_on_map: bool
    registration_list_visible: bool
    participant_chat: bool
    reward: Optional[str]
    similar_events: List[str]
    attendance_confirmation: bool
    chat_link: str
    organizer_profile_link: str
    community_profile_link: str
    poster_image_link: str
    paid: bool
    low_priority: bool
    rejected: bool
    waiting_for_queue: bool
    awaiting_turn: bool
    community_registration_list_visible: bool
    queue_position: Optional[int]
    limits: Limits
    user_status: UserStatus
