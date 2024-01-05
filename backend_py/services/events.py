from typing import Optional, List

from fastapi import FastAPI, HTTPException, Depends, status
from sqlalchemy import create_engine, Column, Integer, String, Boolean, JSON, DateTime
from sqlalchemy.ext.declarative import declarative_base
from sqlalchemy.orm import sessionmaker, Session, relationship
from pydantic import BaseModel
from datetime import datetime

from backend_py.app import app
from backend_py.db import Base, get_db
from backend_py.db.event import EventDB


# Модель Pydantic для запроса создания события
class CreateEventRequest(BaseModel):
    title: str
    description: str
    short_description: str
    category: str
    location: str
    start_date: datetime
    end_date: datetime
    online: bool
    event_marked_on_map: bool
    distance_from_user: Optional[float]
    registration_list_visible: bool
    participant_chat: bool
    reward: Optional[str]
    attendance_confirmation_days_before: Optional[int]
    similar_events: Optional[List[str]]
    attendance_confirmation: bool
    chat_link: str
    organizer_profile_link: str
    community_profile_link: str
    poster_image_link: str
    age_limit: Optional[int]
    paid: bool
    event_limit: Optional[int]
    low_priority: bool
    rejected: bool
    waiting_for_queue: bool
    awaiting_turn: bool
    community_registration_list_visible: bool
    queue_position: Optional[int]
    user_account_id: int


# Модель Pydantic для ответа с информацией о событии
class EventInfo(BaseModel):
    id: int
    title: str
    description: str
    category: str
    start_date: datetime
    end_date: datetime
    online: bool
    location: str
    poster_image_link: str
    age_limit: Optional[int]
    paid: bool
    event_limit: Optional[int]


# Создание события
@app.post("/api/events", response_model=EventInfo)
async def create_event(event_info: CreateEventRequest, db: Session = Depends(get_db)):
    event = EventDB(**event_info.dict())
    db.add(event)
    db.commit()
    db.refresh(event)
    return EventInfo.from_orm(event)


# Получение списка всех событий
@app.get("/api/events", response_model=List[EventInfo])
async def get_all_events(db: Session = Depends(get_db)):
    events = db.query(EventDB).all()
    return [EventInfo.from_orm(event) for event in events]


# Получение информации о конкретном событии
@app.get("/api/events/{event_id}", response_model=EventInfo)
async def get_event(event_id: int, db: Session = Depends(get_db)):
    event = db.query(EventDB).filter(EventDB.id == event_id).first()
    if event is None:
        raise HTTPException(status_code=status.HTTP_404_NOT_FOUND, detail="Event not found")
    return EventInfo.from_orm(event)


# Обновление информации о событии
@app.put("/api/events/{event_id}", response_model=EventInfo)
async def update_event(event_id: int, updated_info: CreateEventRequest, db: Session = Depends(get_db)):
    event = db.query(EventDB).filter(EventDB.id == event_id).first()
    if event is None:
        raise HTTPException(status_code=status.HTTP_404_NOT_FOUND, detail="Event not found")

    # Обновляем только те поля, которые предоставлены в запросе
    for field, value in updated_info.dict().items():
        setattr(event, field, value)

    db.commit()
    db.refresh(event)
    return EventInfo.from_orm(event)


# Удаление события
@app.delete("/api/events/{event_id}")
async def delete_event(event_id: int, db: Session = Depends(get_db)):
    event = db.query(EventDB).filter(EventDB.id == event_id).first()
    if event is None:
        raise HTTPException(status_code=status.HTTP_404_NOT_FOUND, detail="Event not found")

    db.delete(event)
    db.commit()

    return {"message": "Event deleted successfully"}
