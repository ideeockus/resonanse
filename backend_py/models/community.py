from typing import List, Optional
from pydantic import BaseModel


class CommunityLimits(BaseModel):
    community_limit: Optional[int]
    age_limit: Optional[int]


class CommunityStatus(BaseModel):
    members: List[str]
    past_events: List[str]


class Community(BaseModel):
    name: str
    description: str
    poster_image_link: str
    private: bool
    admin_list: List[str]
    telegram_channel_link: Optional[str]
    photo_album: List[str]
    age_limit: Optional[int]
    community_chat: bool
    category: str
    events: List[str]
    platforms: List[str]
    location: str
    community_limit: Optional[int]
    low_priority: bool
    rejected: bool
    waiting_for_queue: bool
    awaiting_turn: bool
    limits: CommunityLimits
    community_status: CommunityStatus
