from datetime import datetime
from typing import List, Optional
from uuid import UUID

from pydantic import BaseModel


class Limits(BaseModel):
    event_limit: Optional[int]
    distance_from_user: Optional[float]
    attendance_confirmation_days_before: Optional[int]
    age_limit: Optional[int]


class UserStatus(BaseModel):
    visited_users: List[str]
    not_attended_users: List[str]


class LocationCoords(BaseModel):
    latitude: float
    longitude: float


# class Event(BaseModel):
#     id: UUID
#     title: str
#     description: str
#     brief_description: str | None
#     subject: str
#     location: str
#     location_coords: LocationCoords
#     datetime_from: datetime
#     datetime_to: datetime
#     online: bool
#     attendance_confirmation_days_before: Optional[int]
#     similar_events: Optional[List[str]]
#     chat_link: str
#     organizer_id: int
#     community_id: int
#     poster_image_uuid: str | None
#     paid: bool
