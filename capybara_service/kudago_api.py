from datetime import datetime
from enum import Enum

import httpx
from httpx import AsyncClient, Response

from capybara_service.models import KudaGoEvent


class KudaGoApi:
    API_URL_BASE = 'https://kudago.com/public-api/'
    API_VERSION = 'v1.4'

    class TextFormat(Enum):
        Html = 'html'
        Plain = 'plain'
        Text = 'text'

    class Location(Enum):
        SPB = "spb"
        MSK = "msk"
        NSK = "nsk"
        EKB = "ekb"
        NNV = "nnv"
        KZN = "kzn"
        VBG = "vbg"
        SMR = "smr"
        KRD = "krd"
        SOCHI = "sochi"
        UFA = "ufa"
        KRASNOYARSK = "krasnoyarsk"
        KEV = "kev"
        NEW_YORK = "new-york"

    @property
    def api_url(self) -> str:
        return self.API_URL_BASE + self.API_VERSION

    async def fetch_events(
            self,
            page: int | None = None,
            page_size: int | None = None,
            lang=None,
            fields=None,
            expand=None,
            order_by=None,
            text_format: TextFormat | None = None,
            ids=None,
            location: Location | None = None,
            actual_since: datetime | None = None,
            actual_until: datetime | None = None,
            is_free=None,
            categories=None,
            lon=None,
            lat=None,
            radius=None,
    ) -> Response:
        base_url = self.api_url + '/events/'

        text_format = text_format.value if text_format else None
        location = location.value if location else None
        actual_since = actual_since.timestamp() if actual_since else None
        actual_until = actual_until.timestamp() if actual_until else None

        params = {
            'page': page,
            'page_size': page_size,
            'lang': lang,
            'fields': fields,
            'expand': expand,
            'order_by': order_by,
            'text_format': text_format,
            'ids': ids,
            'location': location,
            'actual_since': actual_since,
            'actual_until': actual_until,
            'is_free': is_free,
            'categories': categories,
            'lon': lon,
            'lat': lat,
            'radius': radius,
        }

        # Удаление None значений из словаря параметров
        params = {k: v for k, v in params.items() if v is not None}

        async with AsyncClient() as client:
            response = await client.get(base_url, params=params)
            # print(response.json())
            return response
        # if response.status_code == 200:
        #     return response.json()
        # else:
        #     return response.status_code, response.reason
