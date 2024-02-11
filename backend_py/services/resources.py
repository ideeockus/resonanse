import uuid
import os
from pathlib import Path

from fastapi import UploadFile, File
from starlette.responses import FileResponse

from backend_py.app import app, OpenApiTags

# ensure resources dir exists
# resources_dir_path = Path('resonanse_storage/backend_resources')

# resources_dir_path = Path(os.getcwd()).parent / Path('resonanse_storage/event_images')
RESONANSE_STORAGE_PATH = os.getenv('RESONANSE_STORAGE_PATH', '/opt/resonanse_storage')
resonanse_storage = Path(RESONANSE_STORAGE_PATH)
resources_dir_path = resonanse_storage / Path('event_images')
if not resources_dir_path.exists():
    resources_dir_path.mkdir(parents=True, exist_ok=True)


# Метод для загрузки изображения
@app.post('/api/resources/upload-image', tags=[OpenApiTags.RESOURCES])
async def upload_image(file: UploadFile = File(...)):
    # Сохраняем загруженное изображение
    image_uuid = str(uuid.uuid4())
    with open(resources_dir_path / image_uuid, 'wb') as image:
        image.write(file.file.read())
    return {'filename': image_uuid}


# Метод для получения изображения по ссылке
@app.get('/api/resources/get-image/{image_filename}', tags=[OpenApiTags.RESOURCES])
async def get_image(image_filename: str):
    # Возвращаем изображение по запросу
    return FileResponse(
        path=resources_dir_path / image_filename,
        filename=image_filename,
        media_type='multipart/form-data'
    )
