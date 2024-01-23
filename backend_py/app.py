from fastapi import FastAPI
from enum import Enum


class OpenApiTags(Enum):
    AUTH = "auth"
    ACCOUNTS = "accounts"
    EVENTS = "events"
    COMMUNITIES = "communities"
    RESOURCES = 'resources'


tags_metadata = [
    {
        "name": OpenApiTags.AUTH.value,
        "description": "Авторизация и т.п.",
    },
    {
        "name": OpenApiTags.ACCOUNTS.value,
        "description": "Для работы с аккаунтами",
        "externalDocs": {
            "description": "...",
            "url": "https://fastapi.tiangolo.com/",
        },
    },
    {
        "name": OpenApiTags.EVENTS.value,
        "description": "События",
    },
    {
        "name": OpenApiTags.COMMUNITIES.value,
        "description": "Сообщества",
    },
]

app = FastAPI(
    title="ResonanseBackend",
    description="Resonanse rest api",
    # summary="?",
    version="0.1.0",
    # terms_of_service="skip",
    # contact={
    #     "name": "name",
    #     "url": "abc",
    #     "email": "email",
    # },
    # license_info={
    #     "name": "Apache 2.0",
    #     "url": "https://www.apache.org/licenses/LICENSE-2.0.html",
    # },
    openapi_tags=tags_metadata,
)

app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"], # Список разрешенных источников (доменов)
    allow_credentials=True,
    allow_methods=["*"], # Разрешение всех методов
    allow_headers=["*"], # Разрешение всех заголовков
)