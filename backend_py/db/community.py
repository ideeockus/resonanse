from sqlalchemy import Column, Integer, String, Boolean, JSON, DateTime, Float, ForeignKey

from sqlalchemy.orm import relationship
from backend_py.db.base import Base


# Модель SQLAlchemy для таблицы communities
class CommunityDB(Base):
    __tablename__ = "communities"
    id = Column(Integer, primary_key=True, index=True)
    name = Column(String)
    description = Column(String)
    poster_image_link = Column(String)
    private = Column(Boolean)
    admin_list = Column(JSON)
    telegram_channel_link = Column(String, nullable=True)
    photo_album = Column(JSON)
    age_limit = Column(Integer, nullable=True)
    community_chat = Column(Boolean)
    category = Column(String)
    events = Column(JSON)
    platforms = Column(JSON)
    location = Column(String)
    community_limit = Column(Integer, nullable=True)
    low_priority = Column(Boolean)
    rejected = Column(Boolean)
    waiting_for_queue = Column(Boolean)
    awaiting_turn = Column(Boolean)

    # Добавляем внешний ключ для связи с пользовательскими аккаунтами
    user_account_id = Column(Integer, ForeignKey("user_accounts.id"))
    user_account = relationship("UserAccountDB", back_populates="communities")
