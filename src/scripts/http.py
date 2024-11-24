import socket
import api

class Http():
    def __init__(self, ip, port):
        self.ip = ip
        self.port = port
        
    def run(self):
        result = ""
        http_req_header = ("GET / HTTP/1.1\r\nHost:"+ip+"\r\n\r\n").encode()
        request_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        request_socket.connect((ip, int(port)))
        request_socket.send(http_req_header)
        response = request_socket.recv(1024).decode()
        
        headers = response.split("\r\n\r\n")[0]
        
        for header_line in headers.split("\r\n"):
            if "Server" in header_line:
                result = header_line.split(": ")[1]
                #print("result: ", result)
        return result.strip()
        
ip, port = api.get_target()
http = Http(ip, port)
http.run()