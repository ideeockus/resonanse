from sqlalchemy import Column, Integer, String, Boolean, JSON, ForeignKey

from backend_py.db.base import Base


# Модель SQLAlchemy для таблицы communities
class CommunityDB(Base):
    __tablename__ = "communities"
    id = Column(Integer, primary_key=True, index=True)
    name = Column(String)
    description = Column(String)
    poster_image_link = Column(String)
    private = Column(Boolean)
    telegram_channel_link = Column(String, nullable=True)
    community_chat = Column(Boolean)
    category = Column(String)
    location = Column(String)
    owner_id = Column(Integer, ForeignKey('user_accounts.id'))
