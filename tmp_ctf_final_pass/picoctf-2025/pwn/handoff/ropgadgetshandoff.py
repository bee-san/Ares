from pwn import *

rop = ROP(ELF("./handoff"))
print(rop.dump())

print(ELF("./handoff").got)
