import requests
from flask_unsign import decode

ctf_url = "https://09b830de895d397e.247ctf.com"

resp = requests.get(f"{ctf_url}/flag")

# NOTE: The flag is being set in our session cookie, so
#       we just need to decode it
session_cookie = resp.cookies["session"]
print(f"Session cookie: {session_cookie}")

decoded_session = decode(session_cookie)
print(f"Decoded session: {decoded_session}")

flag = decoded_session["flag"].decode("utf-8")
print(f"Flag: {flag}")
