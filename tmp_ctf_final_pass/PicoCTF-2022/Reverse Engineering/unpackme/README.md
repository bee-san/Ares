# unpackme

## Challenge

Can you get the flag? Reverse engineer this [binary](./unpackme-upx).

## Solution

The challenge strongly hints (it's in the binary name, challenge name, and is a literal hint) that this is a binary packed with UPX. Additionally, running `strings unpackme-upx | grep upx` displays `Info: This file is packed with the UPX executable packer http://upx.sf.net $`. According to [their website](https://upx.github.io/), "UPX is a free, portable, extendable, high-performance executable packer for several executable formats."

We can download the latest release of UPX from the [GitHub releases page](https://github.com/upx/upx/releases). Extracting the archive gives us a folder with a `upx` binary. We can run `./upx -d unpackme-upx` to decompress the binary and replace it on disk. Now, our `unpackme-upx` file is unpacked.

Next, run `gdb ./unpackme-upx` and `layout asm` (or `disassemble main`) to get:

```
   0x0000000000401e73 <+0>:	endbr64 
   0x0000000000401e77 <+4>:	push   %rbp
   0x0000000000401e78 <+5>:	mov    %rsp,%rbp
   0x0000000000401e7b <+8>:	sub    $0x50,%rsp
   0x0000000000401e7f <+12>:	mov    %edi,-0x44(%rbp)
   0x0000000000401e82 <+15>:	mov    %rsi,-0x50(%rbp)
   0x0000000000401e86 <+19>:	mov    %fs:0x28,%rax
   0x0000000000401e8f <+28>:	mov    %rax,-0x8(%rbp)
   0x0000000000401e93 <+32>:	xor    %eax,%eax
   0x0000000000401e95 <+34>:	movabs $0x4c75257240343a41,%rax
   0x0000000000401e9f <+44>:	movabs $0x30623e306b6d4146,%rdx
   0x0000000000401ea9 <+54>:	mov    %rax,-0x30(%rbp)
   0x0000000000401ead <+58>:	mov    %rdx,-0x28(%rbp)
   0x0000000000401eb1 <+62>:	movabs $0x3532666630486637,%rax
   0x0000000000401ebb <+72>:	mov    %rax,-0x20(%rbp)
   0x0000000000401ebf <+76>:	movl   $0x36665f60,-0x18(%rbp)
   0x0000000000401ec6 <+83>:	movw   $0x4e,-0x14(%rbp)
   0x0000000000401ecc <+89>:	lea    0xb1131(%rip),%rdi        # 0x4b3004
   0x0000000000401ed3 <+96>:	mov    $0x0,%eax
   0x0000000000401ed8 <+101>:	call   0x410df0 <printf>
   0x0000000000401edd <+106>:	lea    -0x3c(%rbp),%rax
   0x0000000000401ee1 <+110>:	mov    %rax,%rsi
   0x0000000000401ee4 <+113>:	lea    0xb1135(%rip),%rdi        # 0x4b3020
   0x0000000000401eeb <+120>:	mov    $0x0,%eax
   0x0000000000401ef0 <+125>:	call   0x410f80 <__isoc99_scanf>
   0x0000000000401ef5 <+130>:	mov    -0x3c(%rbp),%eax
   0x0000000000401ef8 <+133>:	cmp    $0xb83cb,%eax
   0x0000000000401efd <+138>:	jne    0x401f42 <main+207>
   0x0000000000401eff <+140>:	lea    -0x30(%rbp),%rax
   0x0000000000401f03 <+144>:	mov    %rax,%rsi
   0x0000000000401f06 <+147>:	mov    $0x0,%edi
   0x0000000000401f0b <+152>:	call   0x401db5 <rotate_encrypt>
   0x0000000000401f10 <+157>:	mov    %rax,-0x38(%rbp)
   0x0000000000401f14 <+161>:	mov    0xdd7b5(%rip),%rdx        # 0x4df6d0 <stdout>
   0x0000000000401f1b <+168>:	mov    -0x38(%rbp),%rax
   0x0000000000401f1f <+172>:	mov    %rdx,%rsi
   0x0000000000401f22 <+175>:	mov    %rax,%rdi
   0x0000000000401f25 <+178>:	call   0x420bd0 <fputs>
   0x0000000000401f2a <+183>:	mov    $0xa,%edi
   0x0000000000401f2f <+188>:	call   0x421070 <putchar>
   0x0000000000401f34 <+193>:	mov    -0x38(%rbp),%rax
   0x0000000000401f38 <+197>:	mov    %rax,%rdi
   0x0000000000401f3b <+200>:	call   0x42eec0 <free>
   0x0000000000401f40 <+205>:	jmp    0x401f4e <main+219>
   0x0000000000401f42 <+207>:	lea    0xb10da(%rip),%rdi        # 0x4b3023
   0x0000000000401f49 <+214>:	call   0x420e90 <puts>
   0x0000000000401f4e <+219>:	mov    $0x0,%eax
   0x0000000000401f53 <+224>:	mov    -0x8(%rbp),%rcx
   0x0000000000401f57 <+228>:	xor    %fs:0x28,%rcx
   0x0000000000401f60 <+237>:	je     0x401f67 <main+244>
   0x0000000000401f62 <+239>:	call   0x45cdf0 <__stack_chk_fail_local>
   0x0000000000401f67 <+244>:	leave  
   0x0000000000401f68 <+245>:	ret
```

We know that the program asks for a number and then will probably print the flag given the correct number. So, we are looking for a `cmp` instruction. We see `<+133>:	cmp    $0xb83cb,%eax`. Converting `b83cb` from hexadecimal to decimal yields `754635`.

Running the program and entering `754635` prints out the flag.

Note that without decompressing using `upx`, gdb would have been unable to display any assembly.

### Flag

`picoCTF{up><_m3_f7w_77ad107e}`
