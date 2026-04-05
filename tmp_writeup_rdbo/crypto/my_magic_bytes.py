import os

cur_path = os.path.dirname(os.path.realpath(__file__))
jpg_enc_path = cur_path + "/my_magic_bytes.jpg.enc"

if not os.path.isfile(jpg_enc_path):
	print(f"Unable to find encrypted JPG at: '{jpg_enc_path}'")
	exit(0)

jpg_enc = open(jpg_enc_path, "rb")
jpg_enc_data = jpg_enc.read()
jpg_enc.close()

jpg_magic = b"\xFF\xD8\xFF\xE0\x00\x10\x4A\x46\x49\x46\x00\x01"
jpg_enc_magic = jpg_enc_data[:len(jpg_magic)]

print(f"JPG Magic:     {''.join(['{:02x} '.format(b).upper() for b in jpg_magic])}")
print(f"JPG Enc Magic: {''.join(['{:02x} '.format(b).upper() for b in jpg_enc_magic])}")

xor_key = ""

for i in range(0, len(jpg_magic)):
	xor_byte = chr(jpg_enc_magic[i] ^ jpg_magic[i])
	xor_key += xor_byte

print(f"XOR Key:       {''.join(['{:02x} '.format(ord(b)).upper() for b in xor_key])}")

jpg_dec_path = cur_path + "/my_magic_bytes.jpg"
jpg_dec = open(jpg_dec_path, "wb")

for i in range(0, len(jpg_enc_data)):
	xor_dec = jpg_enc_data[i] ^ ord(xor_key[i % len(xor_key)])
	xor_byte = bytes([xor_dec])
	jpg_dec.write(xor_byte)
jpg_dec.close()

jpg_dec = open(jpg_dec_path, "rb")
jpg_dec_data = jpg_dec.read()
jpg_dec.close()

jpg_dec_magic = jpg_dec_data[:len(jpg_magic)]

print(f"JPG Dec Magic: {''.join(['{:02x} '.format(b).upper() for b in jpg_dec_magic])}")
