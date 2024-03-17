import typing as t
from datetime import datetime

from pydantic.dataclasses import dataclass
from pydantic import BaseModel, HttpUrl


class Event(BaseModel):
    id: int
    start_date: datetime
    end_date: datetime


class Date(BaseModel):
    start: int
    end: int


class Place(BaseModel):
    id: int


class Location(BaseModel):
    slug: str


class Source(BaseModel):
    link: HttpUrl
    name: str


class Image(BaseModel):
    image: HttpUrl
    source: Source


class Agent(BaseModel):
    id: int
    title: str
    slug: str
    agent_type: str
    images: list[Image] = []  # Assuming images structure is similar to the one provided
    site_url: HttpUrl


class Role(BaseModel):
    slug: str


class Participant(BaseModel):
    role: Role
    agent: Agent


class KudaGoEvent(BaseModel):
    id: int
    publication_date: int
    dates: list[Date]
    title: str
    slug: str
    place: Place
    description: str
    body_text: str
    location: Location
    categories: list[str]
    tagline: str
    age_restriction: str
    price: str
    is_free: bool
    images: list[Image]
    favorites_count: int
    comments_count: int
    site_url: HttpUrl
    short_title: str
    tags: list[str]
    disable_comments: bool
    participants: list[Participant]
