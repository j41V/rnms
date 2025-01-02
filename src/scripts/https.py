import socket
import api
import requests

class Https():
    def __init__(self, ip, port):
        self.ip = ip
        self.port = port
        
    def run(self):
        request = requests.get(f"https://{self.ip}:{self.port}")
        result = request.headers["Server"]
        return result.strip()
     
if __name__ == "__main__":   
    ip, port = api.get_target()
    https = Https(ip, port)
    print(https.run(), end="")