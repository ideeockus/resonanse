from datetime import datetime
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
    short_description: str | None
    category: str
    location: str
    start_date: datetime
    end_date: datetime
    online: bool
    attendance_confirmation_days_before: Optional[int]
    similar_events: Optional[List[str]]
    chat_link: str
    organizer_id: int
    community_id: int
    poster_image_link: str | None
    paid: bool
