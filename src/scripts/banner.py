import socket
import api
import requests

class Banner():
    def __init__(self, ip, port):
        self.ip = ip
        self.port = port
        
    def run(self):
        scan_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        scan_socket.connect((self.ip, port))
        scan_socket.send(b"test\n")
        banner = scan_socket.recv(1024)
        result =  banner.decode()
        return result.strip()
     
if __name__ == "__main__":   
    ip, port = api.get_target()
    banner = Banner(ip, port)
    print(banner.run(), end="")