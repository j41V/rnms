import socket
import api

class Example():
    def __init__(self, ip, port):
        self.ip = ip
        self.port = port
        
    def run(self):
        
        results = "whatever"
        request_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        request_socket.connect((self.ip, int(self.port)))     
        return results.strip()
        
        
ip, port = api.get_target()
example = Example(ip, port)
print(example.run(), end="")