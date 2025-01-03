import socket
import api
import requests

class Http():
    def __init__(self, ip, port):
        self.ip = ip
        self.port = port
        
    def run(self):
        request = requests.get(f"http://{self.ip}:{self.port}")
        result = request.headers["Server"]
        return result.strip()
     
if __name__ == "__main__":   
    ip, port = api.get_target()
    http = Http(ip, port)
    print(http.run(), end="")