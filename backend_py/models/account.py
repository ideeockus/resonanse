from typing import Optional
from pydantic import BaseModel


class UserAccount(BaseModel):
    id: Optional[int]
    username: str

    first_name: str
    last_name: str
    city: str
    about: str

    headline: Optional[str]
    goals: Optional[str]
    interests: Optional[str]
    language: Optional[str]
    age: Optional[int]
    education: Optional[str]

    hobby: Optional[str]
    music: Optional[str]
    sport: Optional[str]
    books: Optional[str]
    food: Optional[str]
    worldview: Optional[str]
    alcohol: Optional[str]

    email: Optional[str]
    phone: Optional[str]
    tg_username: Optional[str]
    tg_user_id: Optional[int]
    instagram: Optional[str]

    password_hash: str

    user_type: int
