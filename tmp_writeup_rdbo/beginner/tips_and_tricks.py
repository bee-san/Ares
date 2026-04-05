import socket

ctf_host = "69008c0636f17fa4.247ctf.com"
ctf_port = 50207

sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
sock.connect((ctf_host, ctf_port))

try:
	for i in range(0, 500):
		data = sock.recv(1024)
		data_str = data.decode("utf-8")
		print(f"Q: {data_str}")
		result = eval(data_str.split("What is the answer to ")[-1][:-3]) # The last 3 characters are ?\r\n, ignore them on the evaluation #
		print(f"A: {result}")
		sock.send(f"{result}\r\n".encode("utf-8")) # Format and encode output data #
except KeyboardInterrupt:
	print(f"")
except BaseException as e:
	print(f"[*] Unhandled exception: {e}")

flag = sock.recv(1024).decode("utf-8")
print(f"FLAG: {flag}")
sock.close()