import socket
import api
import http
import https
import banner
import ssh

class DefaultScripts():
    def __init__(self, ip, port):
        self.ip = ip
        self.port = port
        
    def run(self):
        
        results = ""
        
        #http
        http_script = http.Http(self.ip, self.port)
        results = http_script.run()   
        if results != "" and results != "whatever":
            print(results, end="")
            exit(0)
            
        #https
        https_script = https.Https(self.ip, self.port)
        results = https_script.run()   
        if results != "" and results != "whatever":
            print(results, end="")
            exit(0)
                   
        #ssh
        ssh_script = ssh.Ssh(self.ip, self.port)
        results = ssh_script.run()
        if results != "" and results != "whatever":
            print(results, end="")
            exit(0)
        
        #banner
        banner_script = banner.Banner(self.ip, self.port)
        results = banner_script.run()
        if results != "":
            print(results, end="")
        
        
        
        print(results)
        
        
ip, port = api.get_target()
default_scripts = DefaultScripts(ip, port)
default_scripts.run()