import asyncio
from datetime import datetime

from capybara_service.kudago_api import KudaGoApi

if __name__ == '__main__':
    kudago_api = KudaGoApi()
    resp = asyncio.run(kudago_api.fetch_events(
        text_format=KudaGoApi.TextFormat.Text,
        actual_since=datetime.now(),
        location=KudaGoApi.Location.SPB,
    ))
    print(resp)
    print(resp.text)
    print(resp.json())
