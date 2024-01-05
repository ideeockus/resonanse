import os

from sqlalchemy import create_engine
from sqlalchemy.ext.declarative import declarative_base
from sqlalchemy.orm import sessionmaker

from account import UserAccountDB
from backend_py.db.base import Base
from community import CommunityDB

DATABASE_URL = os.environ['DATABASE_URL']

# Создание экземпляра движка SQLAlchemy
engine = create_engine(DATABASE_URL)

# Создание сессии SQLAlchemy
SessionLocal = sessionmaker(autocommit=False, autoflush=False, bind=engine)

# Создание таблицы
Base.metadata.create_all(bind=engine)


# Зависимость для получения сессии базы данных
def get_db():
    db = SessionLocal()
    try:
        yield db
    finally:
        db.close()
