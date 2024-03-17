import http.server
import socketserver
import os

PORT = 8001
DIRECTORY = "build"  # Путь к вашей собранной директории

class MyHttpRequestHandler(http.server.SimpleHTTPRequestHandler):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, directory=DIRECTORY, **kwargs)

    def do_GET(self):
        # Если запрошенный файл не существует, возвращаем index.html
        if not os.path.exists(os.path.join(DIRECTORY, self.path[1:])):  # Игнорируем первый символ '/', чтобы получить правильный путь
            self.path = 'index.html'
        return http.server.SimpleHTTPRequestHandler.do_GET(self)

# Создаем объект http сервера
handler_object = MyHttpRequestHandler

with socketserver.TCPServer(("", PORT), handler_object) as httpd:
    print("Server started at localhost:" + str(PORT))
    httpd.serve_forever()

