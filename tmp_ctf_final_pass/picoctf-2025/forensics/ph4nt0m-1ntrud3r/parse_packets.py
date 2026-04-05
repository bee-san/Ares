import json
import binascii
from base64 import b64decode

with open("packets.json", "r") as f:
    packets = json.load(f)

# this gets you most of the way there, had to manually edit a bit though.

parsed_data = []
print(float(packets[0]['_source']['layers']['frame']['frame.time_delta']))
packets = sorted(packets, key = lambda x: float(x['_source']['layers']['frame']['frame.time_delta']))
for packet in packets:
    tcp_payload = packet['_source']['layers']['tcp']['tcp.payload']
    hexdata = tcp_payload.replace(":", "")
    bytedata = binascii.unhexlify(hexdata)
    decoded_bytes = b64decode(bytedata)
    parsed_data.append(decoded_bytes.decode())

print(''.join(parsed_data))