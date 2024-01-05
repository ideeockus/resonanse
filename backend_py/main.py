# Создаем экземпляр FastAPI
# Таким образом, вы можете использовать app в этом файле для настройки дополнительных параметров запуска,
# если это необходимо
# Например, app = FastAPI(title="My API", version="1.0")
from backend_py.app import app

if __name__ == "__main__":
    import uvicorn

    # Запуск приложения с помощью uvicorn
    uvicorn.run(app, host="127.0.0.1", port=8000)
