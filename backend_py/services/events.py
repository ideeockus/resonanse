from datetime import datetime
from typing import Optional, List
from uuid import UUID

from fastapi import HTTPException, Depends, status
from pydantic import BaseModel, field_validator
from sqlalchemy.orm import Session

from backend_py.app import app, OpenApiTags
from backend_py.db import get_db
from backend_py.db.event import EventDB


# Модель Pydantic для запроса создания события
class CreateEventRequest(BaseModel):
    is_online: bool
    is_paid: bool
    title: str
    description: str
    brief_description: str | None
    subject: str
    location_title: str
    datetime_from: datetime
    datetime_to: datetime
    attendance_confirmation_days_before: Optional[int]
    contact_info: str
    creator_id: int
    # community_id: int
    picture: UUID | None


# Модель Pydantic для ответа с информацией о событии
class EventInfo(BaseModel):
    id: UUID
    is_private: bool
    is_commercial: bool
    is_online: bool
    is_paid: bool
    event_kind: int  # Announcement, UserOffer
    title: str
    description: str
    subject: str
    datetime_from: datetime
    datetime_to: datetime | None
    location_latitude: float | None
    location_longitude: float | None
    location_title: str
    creator_id: int
    event_type: int
    picture: UUID | None = None
    contact_info: str

    @field_validator("subject", mode="before")
    @classmethod
    def convert_subject(cls, raw: int) -> str:
        match raw:
            case 1:
                return "Профессия"
            case 2:
                return "Бизнес"
            case 3:
                return "Образование"
            case 4:
                return "Развлечения"
            case 5:
                return "Спорт"
            case 6:
                return "Общение"
            case 7:
                return "Культура"
            case 8:
                return "Добро"

        return "Unknown"

    class Config:
        orm_mode = True
        from_attributes = True


# Создание события
@app.post("/api/events", response_model=EventInfo, tags=[OpenApiTags.EVENTS])
async def create_event(event_info: CreateEventRequest, db: Session = Depends(get_db)):
    event = EventDB(**event_info.model_dump())
    db.add(event)
    db.commit()
    db.refresh(event)
    return EventInfo.model_validate(event)


# Получение списка всех событий
@app.get("/api/events", response_model=List[EventInfo], tags=[OpenApiTags.EVENTS])
async def get_all_events(db: Session = Depends(get_db)):
    today_date = datetime.now().date()
    # Фильтруем события, начинающиеся после полуночи текущего дня
    events = db.query(EventDB).filter(EventDB.datetime_from >= today_date).all()
    return [EventInfo.model_validate(event) for event in events]


# Получение информации о конкретном событии
@app.get("/api/events/{event_id}", response_model=EventInfo, tags=[OpenApiTags.EVENTS])
async def get_event(event_id: UUID, db: Session = Depends(get_db)):
    event = db.query(EventDB).filter(EventDB.id == event_id).first()
    if event is None:
        raise HTTPException(status_code=status.HTTP_404_NOT_FOUND, detail="Event not found")
    return EventInfo.model_validate(event)

# Обновление информации о событии
# @app.put("/api/events/{event_id}", response_model=EventInfo, tags=[OpenApiTags.EVENTS])
# async def update_event(event_id: UUID, updated_info: CreateEventRequest, db: Session = Depends(get_db)):
#     event = db.query(EventDB).filter(EventDB.id == event_id).first()
#     if event is None:
#         raise HTTPException(status_code=status.HTTP_404_NOT_FOUND, detail="Event not found")
#
#     # Обновляем только те поля, которые предоставлены в запросе
#     for field, value in updated_info.dict().items():
#         setattr(event, field, value)
#
#     db.commit()
#     db.refresh(event)
#     return EventInfo.from_orm(event)


# Удаление события
# @app.delete("/api/events/{event_id}", tags=[OpenApiTags.EVENTS])
# async def delete_event(event_id: UUID, db: Session = Depends(get_db)):
#     event = db.query(EventDB).filter(EventDB.id == event_id).first()
#     if event is None:
#         raise HTTPException(status_code=status.HTTP_404_NOT_FOUND, detail="Event not found")
#
#     db.delete(event)
#     db.commit()
#
#     return {"message": "Event deleted successfully"}
