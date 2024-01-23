from fastapi import HTTPException
from starlette.requests import Request
from starlette.responses import JSONResponse

from backend_py.app import app


# Обработчик исключений для HTTPException
@app.exception_handler(HTTPException)
async def http_exception_handler(request: Request, exc: HTTPException):
    return JSONResponse(
        status_code=exc.status_code,
        content={"detail": exc.detail, "message": exc.detail},
    )


# Обработчик исключений для общих ошибок
@app.exception_handler(Exception)
async def generic_exception_handler(request: Request, exc: Exception):
    return JSONResponse(
        status_code=500,
        content={"detail": "Internal Server Error", "message": str(exc)},
    )
