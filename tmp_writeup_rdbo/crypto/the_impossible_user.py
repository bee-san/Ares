import requests

ctf_url = "https://1b3f9ee7a26af9d1.247ctf.com"

block_len = 16 # 16 characters, 2 bytes each #
block_size = block_len * 2
flag_user = "impossible_flag_user"
flag_user_hex = ""

# Get hex string of 'flag_user' #
for c in flag_user:
	flag_user_hex += hex(ord(c)).replace("0x", "")

print(f"Flag User Hex: {flag_user_hex}")

user_block0 = flag_user_hex[:-2] + "00" # Avoid detection and get first block #
user_block1 = "00" + flag_user_hex[2:]  # Avoid detection and get second block #
print(f"User (First Block):  {user_block0}")
print(f"User (Second Block): {user_block1}")

# Get first block #
req = requests.get(f"{ctf_url}/encrypt?user={user_block0}")
block0 = req.text[:block_size]
print(f"First Block: {block0}")

# Get second block #
req = requests.get(f"{ctf_url}/encrypt?user={user_block1}")
block1 = req.text[block_size:]
print(f"Second Block: {block1}")

# Merge blocks into one #
user_ecb = block0 + block1
print(f"ECB Block: {user_ecb}")

# Retrieve flag #
req = requests.get(f"{ctf_url}/get_flag?user={user_ecb}")
flag = req.text
print(f"FLAG: {flag}")
