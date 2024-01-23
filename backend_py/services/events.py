from typing import Optional, List

from fastapi import FastAPI, HTTPException, Depends, status
from sqlalchemy import create_engine, Column, Integer, String, Boolean, JSON, DateTime
from sqlalchemy.ext.declarative import declarative_base
from sqlalchemy.orm import sessionmaker, Session, relationship
from pydantic import BaseModel
from datetime import datetime

from backend_py.app import app, OpenApiTags
from backend_py.db import Base, get_db
from backend_py.db.event import EventDB


# Модель Pydantic для запроса создания события
class CreateEventRequest(BaseModel):
    title: str
    description: str
    short_description: str | None
    category: str
    location: str
    start_date: datetime
    end_date: datetime
    online: bool
    attendance_confirmation_days_before: Optional[int]
    chat_link: str
    organizer_id: int
    community_id: int
    poster_image_link: str | None
    paid: bool


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
    paid: bool

    class Config:
        orm_mode = True
        from_attributes = True


# Создание события
@app.post("/api/events", response_model=EventInfo,  tags=[OpenApiTags.EVENTS])
async def create_event(event_info: CreateEventRequest, db: Session = Depends(get_db)):
    event = EventDB(**event_info.model_dump())
    db.add(event)
    db.commit()
    db.refresh(event)
    return EventInfo.model_validate(event)


# Получение списка всех событий
@app.get("/api/events", response_model=List[EventInfo], tags=[OpenApiTags.EVENTS])
async def get_all_events(db: Session = Depends(get_db)):
    events = db.query(EventDB).all()
    return [EventInfo.from_orm(event) for event in events]


# Получение информации о конкретном событии
@app.get("/api/events/{event_id}", response_model=EventInfo, tags=[OpenApiTags.EVENTS])
async def get_event(event_id: int, db: Session = Depends(get_db)):
    event = db.query(EventDB).filter(EventDB.id == event_id).first()
    if event is None:
        raise HTTPException(status_code=status.HTTP_404_NOT_FOUND, detail="Event not found")
    return EventInfo.from_orm(event)


# Обновление информации о событии
@app.put("/api/events/{event_id}", response_model=EventInfo, tags=[OpenApiTags.EVENTS])
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
@app.delete("/api/events/{event_id}", tags=[OpenApiTags.EVENTS])
async def delete_event(event_id: int, db: Session = Depends(get_db)):
    event = db.query(EventDB).filter(EventDB.id == event_id).first()
    if event is None:
        raise HTTPException(status_code=status.HTTP_404_NOT_FOUND, detail="Event not found")

    db.delete(event)
    db.commit()

    return {"message": "Event deleted successfully"}
