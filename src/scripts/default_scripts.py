import socket
import api
import http
import ssh

class DefaultScripts():
    def __init__(self, ip, port):
        self.ip = ip
        self.port = port
        
    def run(self):
        
        results = "whatever"
        
        #http
        http_script = http.Http(self.ip, self.port)
        results = http_script.run()   
        if results != "" and results != "whatever":
            print(results, end="")
            exit(0)
        
        #ssh
        ssh_script = ssh.Ssh(self.ip, self.port)
        results = ssh_script.run()
        if results != "" and results != "whatever":
            print(results, end="")
            exit(0)
        
        print("")
        
        
ip, port = api.get_target()
default_scripts = DefaultScripts(ip, port)
default_scripts.run()