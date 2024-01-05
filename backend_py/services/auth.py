from fastapi import HTTPException, Depends, status
from pydantic import BaseModel
from sqlalchemy.orm import Session

from backend_py.app import app
from backend_py.db import SessionLocal, UserAccountDB, get_db


# Модель Pydantic для запроса регистрации
class RegisterRequest(BaseModel):
    username: str
    password: str
    first_name: str
    last_name: str
    city: str
    about: str
    user_type: int


# Модель Pydantic для ответа с информацией о пользователе
class UserInfo(BaseModel):
    id: int
    username: str
    first_name: str
    last_name: str
    city: str
    about: str
    user_type: int


# Регистрация нового пользователя
@app.post("/api/register", response_model=UserInfo)
async def register_user(user_info: RegisterRequest, db: Session = Depends(get_db)):
    user = UserAccountDB(**user_info.model_dump(), password_hash="hashed_password")
    db.add(user)
    db.commit()
    db.refresh(user)
    return UserInfo.from_orm(user)


# Вход пользователя
@app.post("/api/login", response_model=UserInfo)
async def login_user(username: str, password: str, db: Session = Depends(get_db)):
    user = db.query(UserAccountDB).filter(UserAccountDB.username == username,
                                          UserAccountDB.password_hash == "hashed_password").first()
    if user is None:
        raise HTTPException(status_code=status.HTTP_401_UNAUTHORIZED, detail="Invalid credentials")
    return UserInfo.from_orm(user)


# Выход пользователя
@app.post("/api/logout")
async def logout_user():
    # Ваши действия при выходе
    return {"message": "Logged out successfully"}
