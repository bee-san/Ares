#!/bin/sh

# NOTE: The 'jwt.txt' was extracted from the 'access_token_cookie', created when
#       you access <CTF URL>/flag.
# NOTE: The `get_flag()` function checks if your JWT identity is 'admin', if so,
#       return the flag. To change our JWT identity, we can craft a custom payload using 'jwt.io',
#       but first we need to crack the JWT secret, since the token is signed. After
#       obtaining the secret, we can craft the JWT payload and modify our identify from 'anonymous'
#       to 'admin'.
# NOTE: The JWT secret is: wepwn247

john jwt.txt --wordlist=/usr/share/wordlists/rockyou.txt --format=HMAC-SHA256
