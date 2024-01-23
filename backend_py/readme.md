disclaimer: This backend_py project was generated using chatgpt 


## REST API
### Пользовательские методы:
- POST /api/register - регистрация нового пользователя
- POST /api/login - вход пользователя в систему
- POST /api/logout - выход пользователя из системы
- GET /api/user/{id} - получение информации о пользователе (после авторизации)
- PUT /api/user/{id} - обновление информации о пользователе (после авторизации)

### Методы для событий:
- POST /api/events - создание нового события
- GET /api/events - получение списка всех событий
- GET /api/events/{id} - получение информации о конкретном событии
- PUT /api/events/{id} - обновление информации о событии
- DELETE /api/events/{id} - удаление события
- POST /api/events/{id}/participants - добавление участника к событию
- DELETE /api/events/{id}/participants/{participantId} - удаление участника из события

### Методы для сообществ:
- POST /api/communities - создание нового сообщества
- GET /api/communities - получение списка всех сообществ
- GET /api/communities/{id} - получение информации о конкретном сообществе
- PUT /api/communities/{id} - обновление информации о сообществе
- DELETE /api/communities/{id} - удаление сообщества
- POST /api/communities/{id}/members - добавление участника к сообществу
- DELETE /api/communities/{id}/members/{memberId} - удаление участника из сообщества

### Методы для аккаунта:
- GET /api/accounts/{id} - получение информации о конкретном аккаунте
- PUT /api/accounts/{id} - обновление информации о аккаунте
- DELETE /api/accounts/{id} - удаление аккаунта