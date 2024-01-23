from sqlalchemy import Column, Integer, String, Boolean, JSON, DateTime, Float, ForeignKey
from sqlalchemy.orm import relationship

from backend_py.db.base import Base


# Модель SQLAlchemy для таблицы user_accounts
class UserAccountDB(Base):
    __tablename__ = "user_accounts"
    id = Column(Integer, primary_key=True, index=True)
    username = Column(String, unique=True)
    first_name = Column(String)
    last_name = Column(String)
    city = Column(String)
    about = Column(String)
    headline = Column(String, nullable=True)
    goals = Column(String, nullable=True)
    interests = Column(String, nullable=True)
    language = Column(String, nullable=True)
    age = Column(Integer, nullable=True)
    education = Column(String, nullable=True)
    hobby = Column(String, nullable=True)
    music = Column(String, nullable=True)
    sport = Column(String, nullable=True)
    books = Column(String, nullable=True)
    food = Column(String, nullable=True)
    worldview = Column(String, nullable=True)
    alcohol = Column(String, nullable=True)
    email = Column(String, nullable=True)
    phone = Column(String, nullable=True)
    tg_username = Column(String, nullable=True)
    tg_user_id = Column(Integer, nullable=True)
    instagram = Column(String, nullable=True)
    password_hash = Column(String)
    user_type = Column(Integer)
    #
    # # Добавляем внешний ключ для связи с событиями и сообществами
    # events = relationship("EventDB", order_by="EventDB.id", back_populates="user_accounts")
    # communities = relationship("CommunityDB", order_by="CommunityDB.id", back_populates="user_accounts")
