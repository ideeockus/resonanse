from fastapi.testclient import TestClient

# Импортируем экземпляр FastAPI (app) из основного файла (main.py)
from main import app

# Создаем тестового клиента для приложения
client = TestClient(app)


def test_read_main():
    # Тестируем, что корневой маршрут возвращает ожидаемый статус код 200
    response = client.get("/")
    assert response.status_code == 200
    assert response.json() == {"message": "Hello, World!"}


# Примеры тестов для пользовательских методов
def test_register_user():
    response = client.post("/api/register",
                           json={
                               "username": "testuser", "password": "testpassword", "first_name": "John",
                               "last_name": "Doe", "city": "City", "about": "About", "user_type": 1
                           })
    assert response.status_code == 200
    assert "id" in response.json()


def test_login_user():
    response = client.post("/api/login", data={"username": "testuser", "password": "testpassword"})
    assert response.status_code == 200
    assert "id" in response.json()


# Примеры тестов для событий
def test_create_event():
    response = client.post("/api/events",
                           json={
                               "title": "Test Event", "description": "Test Event Description", "category": "Test",
                               "location": "Test Location", "start_date": "2022-01-01T00:00:00",
                               "end_date": "2022-01-02T00:00:00", "online": True, "event_marked_on_map": True,
                               "registration_list_visible": True, "participant_chat": True,
                               "chat_link": "https://example.com/chat",
                               "organizer_profile_link": "https://example.com/organizer",
                               "community_profile_link": "https://example.com/community",
                               "poster_image_link": "https://example.com/poster", "age_limit": 18, "paid": False
                           })
    assert response.status_code == 200
    assert "id" in response.json()


# Примеры тестов для сообществ
def test_create_community():
    response = client.post("/api/communities",
                           json={
                               "name": "Test Community", "description": "Test Community Description",
                               "poster_image_link": "https://example.com/poster", "private": False,
                               "admin_list": ["admin1", "admin2"], "category": "Test", "location": "Test Location",
                               "community_chat": True
                           })
    assert response.status_code == 200
    assert "id" in response.json()
