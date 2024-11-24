import socket
import api

class Ssh():
    def __init__(self, ip, port):
        self.ip = ip
        self.port = port
        
    def run(self):
        results = "whatever"
        request_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        request_socket.connect((self.ip, int(self.port)))
        request_socket.settimeout(5)
        response = request_socket.recv(4096)
        results = response.decode()
        return results.strip()
        
if __name__ == "__main__":        
    ip, port = api.get_target()
    ssh = Ssh(ip, port)
    print(ssh.run(), end="")