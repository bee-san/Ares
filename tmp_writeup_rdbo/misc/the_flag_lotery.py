import socket

ctf_url = "cf31f146aa7e5c6c.247ctf.com"
ctf_port = 50419

response = "0"

while True:
    sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    sock.connect((ctf_url, ctf_port))
    print(sock.recv(1024).decode().replace("\n", ""))
    print(f"Current Guess: {response}")
    sock.send(response.encode())
    response = sock.recv(1024).decode()
    if not response.startswith("Nope"):
        print(f"FLAG: {response}")
        break
    print(response)
    response = response.split(", better luck next time!")[0]
    response = response[response.rfind(" "):]