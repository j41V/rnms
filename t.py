port_file = open("tcp.txt", "r").readlines()
write_file = open("test", "w")
for line in port_file:
    desc, port = line.strip().strip("/udp").split(",")
    write_string = port + " => " + '"' + desc + '".to_string(),\n'
    write_file.write(write_string)