from typing import List, Optional

from fastapi import HTTPException, Depends, status
from pydantic import BaseModel
from sqlalchemy.orm import Session

from backend_py.app import app, OpenApiTags
from backend_py.db import CommunityDB, get_db


# Модель Pydantic для запроса создания сообщества
class CreateCommunityRequest(BaseModel):
    name: str
    description: str
    poster_image_link: str
    private: bool
    telegram_channel_link: str | None
    community_chat: bool
    category: str
    location: str
    owner_id: int


# Модель Pydantic для ответа с информацией о сообществе
class CommunityInfo(BaseModel):
    id: int
    name: str
    description: str
    poster_image_link: str
    private: bool
    telegram_channel_link: str | None
    community_chat: bool
    category: str
    location: str
    owner_id: int

    class Config:
        orm_mode = True
        from_attributes = True


# Создание сообщества
@app.post("/api/communities", response_model=CommunityInfo, tags=[OpenApiTags.COMMUNITIES])
async def create_community(community_info: CreateCommunityRequest, db: Session = Depends(get_db)):
    community = CommunityDB(**community_info.model_dump())
    db.add(community)
    db.commit()
    db.refresh(community)
    return CommunityInfo.from_orm(community)


# Получение списка всех сообществ
@app.get("/api/communities", response_model=List[CommunityInfo], tags=[OpenApiTags.COMMUNITIES])
async def get_all_communities(db: Session = Depends(get_db)):
    communities = db.query(CommunityDB).all()
    return [CommunityInfo.model_validate(community) for community in communities]


# Получение информации о конкретном сообществе
@app.get("/api/communities/{community_id}", response_model=CommunityInfo, tags=[OpenApiTags.COMMUNITIES])
async def get_community(community_id: int, db: Session = Depends(get_db)):
    community = db.query(CommunityDB).filter(CommunityDB.id == community_id).first()
    if community is None:
        raise HTTPException(status_code=status.HTTP_404_NOT_FOUND, detail="Community not found")
    return CommunityInfo.model_validate(community)


# Обновление информации о сообществе
@app.put("/api/communities/{community_id}", response_model=CommunityInfo, tags=[OpenApiTags.COMMUNITIES])
async def update_community(community_id: int, updated_info: CreateCommunityRequest, db: Session = Depends(get_db)):
    community = db.query(CommunityDB).filter(CommunityDB.id == community_id).first()
    if community is None:
        raise HTTPException(status_code=status.HTTP_404_NOT_FOUND, detail="Community not found")

    # Обновляем только те поля, которые предоставлены в запросе
    for field, value in updated_info.dict().items():
        setattr(community, field, value)

    db.commit()
    db.refresh(community)
    return CommunityInfo.model_validate(community)


# Удаление сообщества
@app.delete("/api/communities/{community_id}", response_model=dict)
async def delete_community(community_id: int, db: Session = Depends(get_db)):
    community = db.query(CommunityDB).filter(CommunityDB.id == community_id).first()
    if community:
        db.delete(community)
        db.commit()
        return {"message": "Community deleted successfully"}
    else:
        raise HTTPException(status_code=404, detail="Community not found")
