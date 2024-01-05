from sqlalchemy import Column, Integer, String, Boolean, JSON, DateTime, Float, ForeignKey
from sqlalchemy.orm import relationship

from backend_py.db.base import Base


# Модель SQLAlchemy для таблицы events
class EventDB(Base):
    __tablename__ = "events"
    id = Column(Integer, primary_key=True, index=True)
    title = Column(String)
    description = Column(String)
    short_description = Column(String)
    category = Column(String)
    location = Column(String)
    start_date = Column(DateTime)
    end_date = Column(DateTime)
    online = Column(Boolean)
    event_marked_on_map = Column(Boolean)
    distance_from_user = Column(Float, nullable=True)
    registration_list_visible = Column(Boolean)
    participant_chat = Column(Boolean)
    reward = Column(String, nullable=True)
    attendance_confirmation_days_before = Column(Integer, nullable=True)
    similar_events = Column(JSON, nullable=True)
    attendance_confirmation = Column(Boolean)
    chat_link = Column(String)
    organizer_profile_link = Column(String)
    community_profile_link = Column(String)
    poster_image_link = Column(String)
    age_limit = Column(Integer, nullable=True)
    paid = Column(Boolean)
    event_limit = Column(Integer, nullable=True)
    visited_users = Column(JSON, nullable=True)
    not_attended_users = Column(JSON, nullable=True)
    low_priority = Column(Boolean)
    rejected = Column(Boolean)
    waiting_for_queue = Column(Boolean)
    awaiting_turn = Column(Boolean)
    community_registration_list_visible = Column(Boolean)
    queue_position = Column(Integer, nullable=True)

    # Добавляем внешний ключ для связи с пользовательскими аккаунтами
    user_account_id = Column(Integer, ForeignKey("user_accounts.id"))
    user_account = relationship("UserAccountDB", back_populates="events")
