from typing import List, Optional

from fastapi import FastAPI, HTTPException, Depends, status
from sqlalchemy import create_engine, Column, Integer, String, Boolean, JSON, ForeignKey
from sqlalchemy.ext.declarative import declarative_base
from sqlalchemy.orm import sessionmaker, Session, relationship
from databases import Database
from pydantic import BaseModel
from datetime import datetime

from backend_py.app import app
from backend_py.db import CommunityDB, get_db


# Модель Pydantic для запроса создания сообщества
class CreateCommunityRequest(BaseModel):
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
    platforms: List[str]
    location: str
    community_limit: Optional[int]
    low_priority: bool
    rejected: bool
    waiting_for_queue: bool
    awaiting_turn: bool
    user_account_id: int


# Модель Pydantic для ответа с информацией о сообществе
class CommunityInfo(BaseModel):
    id: int
    name: str
    description: str
    category: str
    location: str
    poster_image_link: str
    private: bool
    admin_list: List[str]
    age_limit: Optional[int]
    community_chat: bool
    platforms: List[str]
    community_limit: Optional[int]
    low_priority: bool
    rejected: bool
    waiting_for_queue: bool
    awaiting_turn: bool


# Создание сообщества
@app.post("/api/communities", response_model=CommunityInfo)
async def create_community(community_info: CreateCommunityRequest, db: Session = Depends(get_db)):
    community = CommunityDB(**community_info.dict())
    db.add(community)
    db.commit()
    db.refresh(community)
    return CommunityInfo.from_orm(community)


# Получение списка всех сообществ
@app.get("/api/communities", response_model=List[CommunityInfo])
async def get_all_communities(db: Session = Depends(get_db)):
    communities = db.query(CommunityDB).all()
    return [CommunityInfo.from_orm(community) for community in communities]


# Получение информации о конкретном сообществе
@app.get("/api/communities/{community_id}", response_model=CommunityInfo)
async def get_community(community_id: int, db: Session = Depends(get_db)):
    community = db.query(CommunityDB).filter(CommunityDB.id == community_id).first()
    if community is None:
        raise HTTPException(status_code=status.HTTP_404_NOT_FOUND, detail="Community not found")
    return CommunityInfo.from_orm(community)


# Обновление информации о сообществе
@app.put("/api/communities/{community_id}", response_model=CommunityInfo)
async def update_community(community_id: int, updated_info: CreateCommunityRequest, db: Session = Depends(get_db)):
    community = db.query(CommunityDB).filter(CommunityDB.id == community_id).first()
    if community is None:
        raise HTTPException(status_code=status.HTTP_404_NOT_FOUND, detail="Community not found")

    # Обновляем только те поля, которые предоставлены в запросе
    for field, value in updated_info.dict().items():
        setattr(community, field, value)

    db.commit()
    db.refresh(community)
    return CommunityInfo.from_orm(community)


# Удаление сообщества
@app.delete("/api/communities/{community_id}")
async def delete_community(community_id: int, db: Session = Depends(get_db)):
    community = db.query(CommunityDB).filter(CommunityDB.id == community_id).first()
    if community is None:
        raise HTTPException(status_code=status.HTTP_404_NOT_FOUND, detail="Community not")
