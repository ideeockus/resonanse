from sqlalchemy import Column, Integer, String, Boolean, JSON, DateTime, Float, ForeignKey, func
from sqlalchemy.orm import relationship

from backend_py.db.base import Base


class EventDB(Base):
    __tablename__ = 'events'

    id = Column(Integer, primary_key=True, index=True)
    title = Column(String, index=True)
    description = Column(String)
    short_description = Column(String)
    category = Column(String)
    location = Column(String)
    start_date = Column(DateTime(timezone=True), server_default=func.now())  # should it be now() ?
    end_date = Column(DateTime(timezone=True), server_default=func.now())
    online = Column(Boolean)
    attendance_confirmation_days_before = Column(Integer, nullable=True)
    chat_link = Column(String)
    organizer_id = Column(Integer, ForeignKey('user_accounts.id'))
    community_id = Column(Integer, ForeignKey('communities.id'))
    poster_image_link = Column(String, nullable=True)
    paid = Column(Boolean)

    # organizer = relationship('UserAccountDB', back_populates='organized_events', foreign_keys=[organizer_id])
    # community = relationship('CommunityDB', back_populates='events')
