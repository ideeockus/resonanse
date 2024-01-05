from typing import Optional

from fastapi import HTTPException, Depends, status
from pydantic import BaseModel
from sqlalchemy.orm import Session

from backend_py.app import app, OpenApiTags
from backend_py.db import UserAccountDB, get_db


# Модель Pydantic для запроса обновления аккаунта
class UpdateAccountRequest(BaseModel):
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


# Модель Pydantic для ответа с информацией об аккаунте
class AccountInfo(BaseModel):
    id: int
    username: str
    first_name: str
    last_name: str
    city: str
    about: str
    user_type: int

    class Config:
        orm_mode = True
        from_attributes = True


# Обновление информации об аккаунте
@app.put("/api/accounts/{account_id}", response_model=AccountInfo, tags=[OpenApiTags.ACCOUNTS])
async def update_account(account_id: int, updated_info: UpdateAccountRequest, db: Session = Depends(get_db)):
    account = db.query(UserAccountDB).filter(UserAccountDB.id == account_id).first()
    if account is None:
        raise HTTPException(status_code=status.HTTP_404_NOT_FOUND, detail="Account not found")

    # Обновляем только те поля, которые предоставлены в запросе
    for field, value in updated_info.dict().items():
        setattr(account, field, value)

    db.commit()
    db.refresh(account)
    return AccountInfo.from_orm(account)


# Получение информации об аккаунте
@app.get("/api/accounts/{account_id}", response_model=AccountInfo, tags=[OpenApiTags.ACCOUNTS])
async def get_account(account_id: int, db: Session = Depends(get_db)):
    account = db.query(UserAccountDB).filter(UserAccountDB.id == account_id).first()
    if account is None:
        raise HTTPException(status_code=status.HTTP_404_NOT_FOUND, detail="Account not found")
    return AccountInfo.model_validate(account)


# Удаление аккаунта
@app.delete("/api/accounts/{account_id}", tags=[OpenApiTags.ACCOUNTS])
async def delete_account(account_id: int, db: Session = Depends(get_db)):
    account = db.query(UserAccountDB).filter(UserAccountDB.id == account_id).first()
    if account is None:
        raise HTTPException(status_code=status.HTTP_404_NOT_FOUND, detail="Account not found")

    db.delete(account)
    db.commit()

    return {"message": "Account deleted successfully"}
