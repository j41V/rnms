from sys import argv

def get_target():
    if len(argv) != 3:
        exit(2)
    
    ip = argv[1]
    port = argv[2]
    return (ip,port)