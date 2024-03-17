from sqlalchemy import Column, Integer, String, Boolean, JSON, DateTime, UUID, Float, ForeignKey, func
from sqlalchemy.orm import relationship

from backend_py.db.base import Base


class EventDB(Base):
    __tablename__ = 'resonanse_events'

    id = Column(UUID, primary_key=True, index=True)
    is_private = Column(Boolean, nullable=False)
    is_commercial = Column(Boolean, nullable=False)
    is_online = Column(Boolean, nullable=False)
    is_paid = Column(Boolean, nullable=False)
    event_kind = Column(Integer, nullable=False)
    title = Column(String)
    description = Column(String)
    brief_description = Column(String)
    subject = Column(String)
    datetime_from = Column(DateTime(timezone=True))
    datetime_to = Column(DateTime(timezone=True), nullable=True)
    location_latitude = Column(Float)
    location_longitude = Column(Float)
    location_title = Column(String)
    creator_id = Column(Integer, ForeignKey('user_accounts.id'))
    # community_id = Column(Integer, ForeignKey('communities.id'))
    event_type = Column(Integer, nullable=False)
    picture = Column(String, nullable=True)
    contact_info = Column(String)
    # chat_link = Column(String)
    creation_time = Column(DateTime(timezone=True), server_default=func.now())
    attendance_confirmation_days_before = Column(Integer, nullable=True)
    # organizer = relationship('UserAccountDB', back_populates='organized_events', foreign_keys=[organizer_id])
    # community = relationship('CommunityDB', back_populates='events')
