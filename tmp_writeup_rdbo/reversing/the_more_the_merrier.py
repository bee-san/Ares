import os
from pathlib import Path

cur_path = Path(os.path.dirname(os.path.realpath(__file__)))
tmtm_path = cur_path / "the_more_the_merrier"
if not os.path.isfile(tmtm_path):
	print(f"Unable to find file 'the_more_the_merrier' at: '{tmtm_path}'")
	exit(0)

tmtm = open(tmtm_path, "rb")
tmtm_data = tmtm.read()
tmtm.close()

flag_template = "247CTF{}"
flag_signature = b""
flag_signature_end = b""

for c in flag_template:
	cur_byte = f"{c}\x00\x00\x00"
	if c != flag_template[-1]:
		flag_signature += cur_byte.encode("utf-8")
	else:
		flag_signature_end += cur_byte.encode("utf-8")

print(f"Flag Signature (Base): {flag_signature}")
print(f"Flag Signature (End): {flag_signature_end}")

flag_base = tmtm_data.find(flag_signature)
print(f"Flag Base: {flag_base}")

flag_end = tmtm_data.find(flag_signature_end)
print(f"Flag Base: {flag_end}")

flag = tmtm_data[flag_base:flag_end+len(flag_signature_end)].decode("utf-8")
print(f"FLAG: {flag}")
