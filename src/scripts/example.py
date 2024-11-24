import socket
import api

class Example():
    def __init__(self, ip, port):
        self.ip = ip
        self.port = port
        
    def run(self):
        
        results = "whatever"
        
        print(results)
        
        
ip, port = api.get_target()
example = Example(ip, port)
example.run()