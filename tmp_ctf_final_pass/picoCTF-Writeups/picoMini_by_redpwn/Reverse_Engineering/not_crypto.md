# not crypto

- [Challenge information](#challenge-information)
- [Solution](#solution)
- [References](#references)

## Challenge information

```text
Level: Hard
Tags: picoMini by redpwn, Reverse Engineering
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: ASPHYXIA

Description:
there's crypto in here but the challenge is not crypto... 🤔

Hints:
(None)
```

Challenge link: [https://play.picoctf.org/practice/challenge/222](https://play.picoctf.org/practice/challenge/222)

## Solution

### Basic file analysis

Let's start by checking the given file with `file`

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoMini_by_redpwn/Reverse_Engineering/not_crypto]
└─$ file not-crypto 
not-crypto: ELF 64-bit LSB pie executable, x86-64, version 1 (SYSV), dynamically linked, interpreter /lib64/ld-linux-x86-64.so.2, BuildID[sha1]=1f838db474ea41305b3181bc0acdc8231273189d, for GNU/Linux 4.4.0, stripped
```

So it is a 64-bit ELF binary, it is a PIE (position-independent executable) and it's stripped.

Now let's run the file and see what happens

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoMini_by_redpwn/Reverse_Engineering/not_crypto]
└─$ ./not-crypto
I heard you wanted to bargain for a flag... whatcha got?
aaaaaaaaaaaaaaaaaaaaaaa
bbbbbbbbbbbbbbbbbbbbbbb
ccccc
ddd
^C
```

Not much action there at all. I tried different inputs and sometimes I got the message `Nope, come back later`

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoMini_by_redpwn/Reverse_Engineering/not_crypto]
└─$ ./not-crypto       
I heard you wanted to bargain for a flag... whatcha got?
test
mooooooore
even mooooooooooooooooooooore data
bbbbbbbbbbbbbbbbbbbbb
Nope, come back later
```

And why not check for other strings?

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoMini_by_redpwn/Reverse_Engineering/not_crypto]
└─$ strings -n 8 ./not-crypto
/lib64/ld-linux-x86-64.so.2
__cxa_finalize
__libc_start_main
__stack_chk_fail
libc.so.6
GLIBC_2.4
GLIBC_2.2.5
_ITM_deregisterTMCloneTable
__gmon_start__
_ITM_registerTMCloneTable
AVAUATUSH
\$ H9\$0u
[]A\A]A^A_
2\$S2D$P@2t$ZD
2\$VD2d$^D
\$[D2L$RD2T$TD
D2D$]@2t$\
[]A\A]A^A_
I heard you wanted to bargain for a flag... whatcha got?
Nope, come back later
Yep, that's it!
GCC: (GNU) 10.2.0
.shstrtab
.note.gnu.property
.note.gnu.build-id
.note.ABI-tag
.gnu.hash
.gnu.version
.gnu.version_r
.rela.dyn
.rela.plt
.eh_frame_hdr
.eh_frame
.init_array
.fini_array
.dynamic
.got.plt
.comment
```

The string `Yep, that's it!` looks very promising. Let's keep an eye out for that!

I also tried to run `strace`

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoMini_by_redpwn/Reverse_Engineering/not_crypto]
└─$ strace ./not-crypto
execve("./not-crypto", ["./not-crypto"], 0x7fff8df28630 /* 54 vars */) = 0
brk(NULL)                               = 0x5568ba815000
mmap(NULL, 8192, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0x7f32ec597000
access("/etc/ld.so.preload", R_OK)      = -1 ENOENT (No such file or directory)
openat(AT_FDCWD, "/etc/ld.so.cache", O_RDONLY|O_CLOEXEC) = 3
newfstatat(3, "", {st_mode=S_IFREG|0644, st_size=87330, ...}, AT_EMPTY_PATH) = 0
mmap(NULL, 87330, PROT_READ, MAP_PRIVATE, 3, 0) = 0x7f32ec581000
close(3)                                = 0
openat(AT_FDCWD, "/lib/x86_64-linux-gnu/libc.so.6", O_RDONLY|O_CLOEXEC) = 3
read(3, "\177ELF\2\1\1\3\0\0\0\0\0\0\0\0\3\0>\0\1\0\0\0Ps\2\0\0\0\0\0"..., 832) = 832
pread64(3, "\6\0\0\0\4\0\0\0@\0\0\0\0\0\0\0@\0\0\0\0\0\0\0@\0\0\0\0\0\0\0"..., 784, 64) = 784
newfstatat(3, "", {st_mode=S_IFREG|0755, st_size=1922136, ...}, AT_EMPTY_PATH) = 0
pread64(3, "\6\0\0\0\4\0\0\0@\0\0\0\0\0\0\0@\0\0\0\0\0\0\0@\0\0\0\0\0\0\0"..., 784, 64) = 784
mmap(NULL, 1970000, PROT_READ, MAP_PRIVATE|MAP_DENYWRITE, 3, 0) = 0x7f32ec3a0000
mmap(0x7f32ec3c6000, 1396736, PROT_READ|PROT_EXEC, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3, 0x26000) = 0x7f32ec3c6000
mmap(0x7f32ec51b000, 339968, PROT_READ, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3, 0x17b000) = 0x7f32ec51b000
mmap(0x7f32ec56e000, 24576, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3, 0x1ce000) = 0x7f32ec56e000
mmap(0x7f32ec574000, 53072, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_FIXED|MAP_ANONYMOUS, -1, 0) = 0x7f32ec574000
close(3)                                = 0
mmap(NULL, 12288, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0x7f32ec39d000
arch_prctl(ARCH_SET_FS, 0x7f32ec39d740) = 0
set_tid_address(0x7f32ec39da10)         = 63993
set_robust_list(0x7f32ec39da20, 24)     = 0
rseq(0x7f32ec39e060, 0x20, 0, 0x53053053) = 0
mprotect(0x7f32ec56e000, 16384, PROT_READ) = 0
mprotect(0x5568b991e000, 4096, PROT_READ) = 0
mprotect(0x7f32ec5c9000, 8192, PROT_READ) = 0
prlimit64(0, RLIMIT_STACK, NULL, {rlim_cur=8192*1024, rlim_max=RLIM64_INFINITY}) = 0
munmap(0x7f32ec581000, 87330)           = 0
newfstatat(1, "", {st_mode=S_IFCHR|0600, st_rdev=makedev(0x88, 0), ...}, AT_EMPTY_PATH) = 0
getrandom("\x8e\x30\x60\xce\x5f\xf0\xc5\x01", 8, GRND_NONBLOCK) = 8
brk(NULL)                               = 0x5568ba815000
brk(0x5568ba836000)                     = 0x5568ba836000
write(1, "I heard you wanted to bargain fo"..., 57I heard you wanted to bargain for a flag... whatcha got?
) = 57
newfstatat(0, "", {st_mode=S_IFCHR|0600, st_rdev=makedev(0x88, 0), ...}, AT_EMPTY_PATH) = 0
read(0, test
"test\n", 1024)                 = 5
read(0, dfgdg
"dfgdg\n", 1024)                = 6
read(0, dfgf
"dfgf\n", 1024)                 = 5
read(0, 213123
"213123\n", 1024)               = 7
read(0, 3532523
"3532523\n", 1024)              = 8
read(0, ^C0x5568ba8156b0, 1024)           = ? ERESTARTSYS (To be restarted if SA_RESTART is set)
strace: Process 63993 detached
```

No new information from that unfortunately.

### Decompile the file in Ghidra

Then let's decompile the file in [Ghidra](https://ghidra-sre.org/) and study the code. Import the file in Ghidra and analyze it with the default settings. Double-click on each function to show the decompiled version of it.

The most promising function is this one `FUN_00101070`

```C
int FUN_00101070(void)

{
  byte *pbVar1;
  undefined auVar2 [16];
  undefined auVar3 [16];
  undefined auVar4 [16];
  undefined auVar5 [16];
  undefined auVar6 [16];
  undefined auVar7 [16];
  undefined auVar8 [16];
  byte bVar9;
  byte bVar10;
  byte bVar11;
  byte bVar12;
  byte bVar13;
  byte bVar14;
  byte bVar15;
  byte bVar16;
  byte bVar17;
  byte bVar18;
  uint uVar19;
  uint6 uVar20;
  unkuint10 Var21;
  undefined auVar22 [12];
  undefined auVar23 [14];
  int iVar24;
  undefined4 uVar25;
  byte *pbVar26;
  byte bVar27;
  byte bVar28;
  byte bVar29;
  long lVar30;
  byte bVar31;
  byte bVar32;
  byte bVar33;
  ulong uVar34;
  byte bVar35;
  uint uVar36;
  ulong uVar37;
  byte bVar38;
  byte bVar39;
  byte bVar40;
  byte bVar41;
  byte bVar42;
  byte bVar43;
  byte bVar44;
  byte bVar45;
  byte *pbVar46;
  long in_FS_OFFSET;
  byte local_1fe;
  byte local_1fd;
  uint local_1fc;
  uint local_1f8;
  byte local_1f4;
  byte local_1f3;
  byte local_1f2;
  byte local_1f1;
  byte local_1f0;
  byte local_1ef;
  byte local_1ee;
  byte local_1ed;
  byte local_1ec;
  byte *local_1e8;
  undefined local_198 [64];
  undefined local_158 [16];
  byte local_148 [144];
  byte local_b8;
  byte local_b7;
  byte local_b6;
  byte local_b5;
  byte local_b4;
  byte local_b3;
  byte local_b2;
  byte local_b1;
  byte local_b0;
  byte local_af;
  byte local_ae;
  byte local_ad;
  byte local_ac;
  byte local_ab;
  byte local_aa;
  byte local_a9;
  undefined local_a8 [3];
  undefined auStack165 [2];
  undefined auStack163 [2];
  undefined uStack161;
  undefined8 uStack160;
  undefined local_98 [16];
  undefined local_88 [16];
  undefined local_78 [16];
  undefined local_68 [16];
  undefined local_58 [16];
  byte local_48 [8];
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  puts("I heard you wanted to bargain for a flag... whatcha got?");
  bVar40 = 0x98;
  bVar32 = 0x32;
  bVar27 = 0x6c;
  bVar28 = 0x1c;
  local_158 = _DAT_001021a0;
  uVar37 = 4;
  pbVar26 = local_158;
  do {
    if ((uVar37 & 3) == 0) {
      uVar34 = (ulong)bVar32;
      bVar32 = (&DAT_001020a0)[bVar27];
      bVar27 = (&DAT_001020a0)[bVar28];
      bVar28 = (&DAT_001020a0)[bVar40];
      bVar40 = (&DAT_001020a0)[uVar34] ^ (&DAT_00102080)[uVar37 >> 2];
    }
    bVar40 = bVar40 ^ *pbVar26;
    uVar36 = (int)uVar37 + 1;
    uVar37 = (ulong)uVar36;
    bVar32 = bVar32 ^ pbVar26[1];
    bVar27 = bVar27 ^ pbVar26[2];
    bVar28 = bVar28 ^ pbVar26[3];
    pbVar26[0x10] = bVar40;
    pbVar26[0x11] = bVar32;
    pbVar26[0x12] = bVar27;
    pbVar26[0x13] = bVar28;
    pbVar26 = pbVar26 + 4;
  } while (uVar36 != 0x2c);
  _local_a8 = _DAT_001021b0;
  fread(local_198,1,0x40,stdin);
  local_88 = _DAT_001021c0;
  local_78 = _DAT_001021d0;
  local_68 = _DAT_001021e0;
  local_58 = _DAT_001021f0;
  iVar24 = 0x10;
  local_1e8 = local_88;
  do {
    if (iVar24 == 0x10) {
      local_1f8 = local_1f8 & 0xffffff00 | (uint)(byte)(&DAT_001020a0)[local_158[0] ^ local_a8[0]];
      uVar25 = vpextrb_avx(_local_a8,4);
      local_1fc = local_1fc & 0xffffff00 |
                  (uint)(byte)(&DAT_001020a0)[(byte)((byte)uVar25 ^ local_158[4])];
      local_1ee = (&DAT_001020a0)[local_158[8] ^ (byte)uStack160];
      uVar25 = vpextrb_avx(_local_a8,0xc);
      local_1ef = (&DAT_001020a0)[(byte)((byte)uVar25 ^ local_158[12])];
      uVar25 = vpextrb_avx(_local_a8,1);
      local_1f4 = (&DAT_001020a0)[(byte)((byte)uVar25 ^ local_158[1])];
      uVar25 = vpextrb_avx(_local_a8,5);
      local_1fd = (&DAT_001020a0)[(byte)((byte)uVar25 ^ local_158[5])];
      uVar25 = vpextrb_avx(_local_a8,9);
      local_1fe = (&DAT_001020a0)[(byte)((byte)uVar25 ^ local_158[9])];
      uVar25 = vpextrb_avx(_local_a8,0xd);
      local_1f0 = (&DAT_001020a0)[(byte)((byte)uVar25 ^ local_158[13])];
      uVar25 = vpextrb_avx(_local_a8,2);
      bVar32 = (&DAT_001020a0)[(byte)((byte)uVar25 ^ local_158[2])];
      uVar25 = vpextrb_avx(_local_a8,6);
      local_1ec = (&DAT_001020a0)[(byte)((byte)uVar25 ^ local_158[6])];
      uVar25 = vpextrb_avx(_local_a8,10);
      local_1f1 = (&DAT_001020a0)[(byte)((byte)uVar25 ^ local_158[10])];
      uVar25 = vpextrb_avx(_local_a8,0xe);
      local_1f2 = (&DAT_001020a0)[(byte)((byte)uVar25 ^ local_158[14])];
      uVar25 = vpextrb_avx(_local_a8,3);
      local_1ed = (&DAT_001020a0)[(byte)((byte)uVar25 ^ local_158[3])];
      uVar25 = vpextrb_avx(_local_a8,7);
      bVar27 = (&DAT_001020a0)[(byte)((byte)uVar25 ^ local_158[7])];
      uVar25 = vpextrb_avx(_local_a8,0xb);
      bVar28 = (&DAT_001020a0)[(byte)((byte)uVar25 ^ local_158[11])];
      uVar25 = vpextrb_avx(_local_a8,0xf);
      local_1f3 = (&DAT_001020a0)[(byte)((byte)uVar25 ^ local_158[15])];
      pbVar26 = local_148;
      do {
        bVar41 = local_1fd ^ (byte)local_1f8;
        bVar31 = local_1f3 ^ local_1f1;
        bVar43 = bVar41 ^ bVar31;
        bVar35 = local_1fe ^ (byte)local_1fc;
        bVar38 = local_1ed ^ local_1f2;
        bVar39 = bVar35 ^ bVar38;
        bVar44 = local_1ed ^ (byte)local_1fc;
        bVar12 = bVar27 ^ bVar32;
        bVar10 = local_1f0 ^ local_1ee;
        bVar13 = local_1ee ^ bVar27;
        bVar42 = bVar10 ^ bVar12;
        bVar14 = local_1ec ^ bVar28;
        bVar9 = local_1ec ^ local_1f4;
        bVar11 = local_1f4 ^ local_1ef;
        bVar15 = local_1ef ^ bVar28;
        bVar16 = bVar11 ^ bVar14;
        bVar45 = pbVar26[7] ^ bVar39 ^ local_1ed;
        bVar17 = bVar32 ^ bVar42 ^ pbVar26[10];
        bVar33 = pbVar26[0xd] ^ bVar16 ^ local_1f4;
        bVar29 = pbVar26[0xe] ^ bVar16 ^ local_1ec;
        bVar18 = bVar28 ^ pbVar26[0xf] ^ bVar16;
        bVar40 = *pbVar26;
        uVar36 = local_1f8 & 0xffffff00;
        local_1f8 = uVar36 | (byte)(&DAT_001020a0)
                                   [(byte)((byte)local_1f8 ^ bVar40 ^ bVar43 ^
                                          ((char)bVar41 >> 7) * -0x1b ^ bVar41 * '\x02')];
        pbVar1 = pbVar26 + 4;
        uVar19 = local_1fc & 0xffffff00;
        local_1fc = uVar19 | (byte)(&DAT_001020a0)
                                   [(byte)(*pbVar1 ^ bVar39 ^ (byte)local_1fc ^
                                          ((char)bVar35 >> 7) * -0x1b ^ bVar35 * '\x02')];
        local_1ee = (&DAT_001020a0)
                    [(byte)(pbVar26[8] ^ bVar42 ^ local_1ee ^
                           ((char)bVar10 >> 7) * -0x1b ^ bVar10 * '\x02')];
        local_1ef = (&DAT_001020a0)
                    [(byte)(bVar16 ^ pbVar26[0xc] ^ local_1ef ^
                           bVar11 * '\x02' ^ ((char)bVar11 >> 7) * -0x1b)];
        local_1f4 = (&DAT_001020a0)
                    [(byte)(pbVar26[1] ^ bVar43 ^ local_1fd ^
                           ((char)(local_1f1 ^ local_1fd) >> 7) * -0x1b ^
                           (local_1f1 ^ local_1fd) * '\x02')];
        local_1fd = (&DAT_001020a0)
                    [(byte)(pbVar26[5] ^ bVar39 ^ local_1fe ^
                           (local_1f2 ^ local_1fe) * '\x02' ^
                           ((char)(local_1f2 ^ local_1fe) >> 7) * -0x1b)];
        local_1fe = (&DAT_001020a0)
                    [(byte)(local_1f0 ^ bVar42 ^ pbVar26[9] ^
                           (local_1f0 ^ bVar32) * '\x02' ^ ((char)(local_1f0 ^ bVar32) >> 7) * -0x1b
                           )];
        local_1f0 = (&DAT_001020a0)
                    [((uint)(bVar9 >> 7) * 0x1b ^ (uint)bVar9 + (uint)bVar9 ^ (uint)bVar33) & 0xff];
        pbVar46 = pbVar26 + 0x10;
        bVar32 = (&DAT_001020a0)
                 [(byte)(pbVar26[2] ^ bVar43 ^ local_1f1 ^
                        ((char)bVar31 >> 7) * -0x1b ^ bVar31 * '\x02')];
        local_1ec = (&DAT_001020a0)
                    [(byte)(local_1f2 ^ pbVar26[6] ^ bVar39 ^
                           bVar38 * '\x02' ^ ((char)bVar38 >> 7) * -0x1b)];
        local_1f1 = (&DAT_001020a0)
                    [((uint)bVar12 * 2 ^ (uint)(bVar12 >> 7) * 0x1b ^ (uint)bVar17) & 0xff];
        local_1f2 = (&DAT_001020a0)
                    [((uint)bVar14 * 2 ^ (uint)(bVar14 >> 7) * 0x1b ^ (uint)bVar29) & 0xff];
        bVar28 = (&DAT_001020a0)
                 [((uint)(bVar13 >> 7) * 0x1b ^ (uint)bVar13 * 2 ^
                  (uint)(byte)(bVar27 ^ bVar42 ^ pbVar26[0xb])) & 0xff];
        local_1ed = (&DAT_001020a0)
                    [(byte)(pbVar26[3] ^ bVar43 ^ local_1f3 ^
                           (local_1f3 ^ (byte)local_1f8) * '\x02' ^
                           ((char)(local_1f3 ^ (byte)local_1f8) >> 7) * -0x1b)];
        bVar27 = (&DAT_001020a0)[(byte)(bVar45 ^ ((char)bVar44 >> 7) * -0x1b ^ bVar44 * '\x02')];
        local_1f3 = (&DAT_001020a0)
                    [((uint)(bVar15 >> 7) * 0x1b ^ (uint)bVar15 * 2 ^ (uint)bVar18) & 0xff];
        pbVar26 = pbVar46;
      } while (&local_b8 != pbVar46);
      local_1f8 = uVar36 | (&DAT_001020a0)
                           [(byte)((byte)local_1f8 ^ bVar40 ^ bVar43 ^
                                  ((char)bVar41 >> 7) * -0x1b ^ bVar41 * '\x02')] ^ local_b8;
      auVar2 = vmovd_avx((uint)(bVar32 ^ local_ae));
      local_1fc = uVar19 | local_1f2 ^ local_b2;
      auVar3 = vmovd_avx((uint)(local_1ec ^ local_aa));
      auVar4 = vmovd_avx((uint)(local_1f1 ^ local_b6));
      auVar8 = vpinsrb_avx(auVar2,(uint)(local_ad ^ bVar27),1);
      auVar2 = vmovd_avx((uint)((&DAT_001020a0)
                                [(byte)(*pbVar1 ^ bVar39 ^ (byte)local_1fc ^
                                       ((char)bVar35 >> 7) * -0x1b ^ bVar35 * '\x02')] ^ local_b4));
      lVar30 = 0xf;
      auVar5 = vmovd_avx((uint)(local_1ee ^ local_b0));
      auVar6 = vmovd_avx(local_1f8);
      auVar7 = vmovd_avx(local_1fc);
      auVar4 = vpinsrb_avx(auVar4,(uint)(local_1f3 ^ local_b5),1);
      auVar6 = vpinsrb_avx(auVar6,(uint)(local_1fd ^ local_b7),1);
      auVar5 = vpinsrb_avx(auVar5,(uint)(local_1f0 ^ local_af),1);
      auVar6 = vpunpcklwd_avx(auVar6,auVar4);
      auVar2 = vpinsrb_avx(auVar2,(uint)(local_1fe ^ local_b3),1);
      auVar4 = vpinsrb_avx(auVar7,(uint)(local_1ed ^ local_b1),1);
      auVar5 = vpunpcklwd_avx(auVar5,auVar8);
      auVar4 = vpunpcklwd_avx(auVar2,auVar4);
      auVar2 = vmovd_avx((uint)(local_1ef ^ local_ac));
      auVar4 = vpunpckldq_avx(auVar6,auVar4);
      auVar2 = vpinsrb_avx(auVar2,(uint)(local_1f4 ^ local_ab),1);
      auVar3 = vpinsrb_avx(auVar3,(uint)(bVar28 ^ local_a9),1);
      auVar2 = vpunpcklwd_avx(auVar2,auVar3);
      auVar2 = vpunpckldq_avx(auVar5,auVar2);
      local_98 = vpunpcklqdq_avx(auVar4,auVar2);
      if (uStack160._7_1_ == -1) {
        _local_a8 = _local_a8 & (undefined  [16])0xffffffffffffffff;
        lVar30 = 0xe;
        uStack160._7_1_ = uStack160._6_1_;
        if (uStack160._6_1_ == -1) {
          auVar23 = _local_a8;
          _local_a8 = ZEXT1415(_local_a8);
          lVar30 = 0xd;
          uStack160._7_1_ = uStack160._5_1_;
          if (uStack160._5_1_ == -1) {
            _local_a8 = _local_a8 & (undefined  [14])0xffffffffffffffff;
            _local_a8 = CONCAT214(uStack160._6_2_,auVar23) & (undefined  [16])0xffffffffffffffff;
            lVar30 = 0xc;
            uStack160._7_1_ = uStack160._4_1_;
            if (uStack160._4_1_ == -1) {
              auVar22 = _local_a8;
              _local_a8 = ZEXT1213(_local_a8);
              lVar30 = 0xb;
              uStack160._7_1_ = uStack160._3_1_;
              if (uStack160._3_1_ == -1) {
                _local_a8 = _local_a8 & (undefined  [12])0xffffffffffffffff;
                _local_a8 = CONCAT412(uStack160._4_4_,auVar22) & (undefined  [16])0xffffffffffffffff
                ;
                lVar30 = 10;
                uStack160._7_1_ = uStack160._2_1_;
                if (uStack160._2_1_ == -1) {
                  Var21 = _local_a8;
                  _local_a8 = ZEXT1011(_local_a8);
                  lVar30 = 9;
                  uStack160._7_1_ = uStack160._1_1_;
                  if (uStack160._1_1_ == -1) {
                    _local_a8 = _local_a8 & 0xffffffffffffffff;
                    _local_a8 = CONCAT610(uStack160._2_6_,Var21) &
                                (undefined  [16])0xffffffffffffffff;
                    lVar30 = 8;
                    uStack160._7_1_ = (byte)uStack160;
                    if ((byte)uStack160 == -1) {
                      uVar37 = _local_a8;
                      _local_a8 = (unkuint9)_local_a8;
                      lVar30 = 7;
                      uStack160._7_1_ = uStack161;
                      if (uStack161 == -1) {
                        _local_a8 = _local_a8 & 0xffffffffffffff;
                        _local_a8 = CONCAT88(uStack160,uVar37) & (undefined  [16])0xffffffffffffffff
                        ;
                        lVar30 = 6;
                        uStack160._7_1_ = auStack163[1];
                        if (auStack163[1] == -1) {
                          uVar20 = _local_a8;
                          _local_a8 = (uint7)_local_a8;
                          lVar30 = 5;
                          uStack160._7_1_ = auStack163[0];
                          if (auStack163[0] == -1) {
                            _local_a8 = _local_a8 & 0xffffffffff;
                            _local_a8 = CONCAT106(stack0xffffffffffffff5e,uVar20) &
                                        (undefined  [16])0xffff00ffffffffff;
                            lVar30 = 4;
                            uStack160._7_1_ = auStack165[1];
                            if (auStack165[1] == -1) {
                              uVar36 = _local_a8;
                              _local_a8 = (uint5)_local_a8;
                              lVar30 = 3;
                              uStack160._7_1_ = auStack165[0];
                              if (auStack165[0] == -1) {
                                _local_a8 = _local_a8 & 0xffffff;
                                _local_a8 = CONCAT124(stack0xffffffffffffff5c,uVar36) &
                                            (undefined  [16])0xffffffff00ffffff;
                                lVar30 = 2;
                                uStack160._7_1_ = local_a8[2];
                                if (local_a8[2] == -1) {
                                  lVar30 = 1;
                                  uStack160._7_1_ = local_a8[1];
                                  if (local_a8[1] == -1) {
                                    _local_a8 = CONCAT142(stack0xffffffffffffff5a,local_a8._0_2_) &
                                                (undefined  [16])0xffffffffffff00ff;
                                    lVar30 = 0;
                                    uStack160._7_1_ = local_a8[0];
                                    if (local_a8[0] == -1) {
                                      _local_a8 = ZEXT1416(stack0xffffffffffffff5a) << 0x10 &
                                                  (undefined  [16])0xffffffffffff0000;
                                      iVar24 = 0;
                                      goto LAB_00101385;
                                    }
                                  }
                                }
                              }
                            }
                          }
                        }
                      }
                    }
                  }
                }
              }
            }
          }
        }
      }
      local_a8[lVar30] = uStack160._7_1_ + '\x01';
      iVar24 = 0;
    }
LAB_00101385:
    lVar30 = (long)iVar24;
    iVar24 = iVar24 + 1;
    *local_1e8 = *local_1e8 ^ local_98[lVar30];
    local_1e8 = local_1e8 + 1;
    if (local_48 == local_1e8) {
      iVar24 = memcmp(local_88,local_198,0x40);
      if (iVar24 == 0) {
        puts("Yep, that\'s it!");
      }
      else {
        iVar24 = 1;
        puts("Nope, come back later");
      }
      if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
        return iVar24;
      }
                    /* WARNING: Subroutine does not return */
      __stack_chk_fail();
    }
  } while( true );
}
```

A bit from the beginning there is a call to `fread`

```c
<---snip--->
  _local_a8 = _DAT_001021b0;
  fread(local_198,1,0x40,stdin);
  local_88 = _DAT_001021c0;
<---snip--->
```

It reads 0x40 or decimal 64 bytes of input. That's probably why the program didn't always give me an output message.

And at the end of the function we see the string `Yep, that's it!` after a call to `memcmp`.

```c
<---snip--->
      iVar24 = memcmp(local_88,local_198,0x40);
      if (iVar24 == 0) {
        puts("Yep, that\'s it!");
      }
<---snip--->
```

The call to `memcmp` ought to be a comparison with the flag involved.

### Find out where to set the breakpoint

Let's try to set a breakpoint and read what is compared in memory with `memcmp`.

First I tried to set a breakpoint at the beginning of the `memcmp` function with `break memcmp` but that didn't reveal any flags.

Can we set a breakpoint at the call to `memcmp` instead?  
But there is a slight problem. Since the program is a PIE we only have relative offsets in Ghidra

```text
        001013b4 48 8b 7c        MOV        RDI,qword ptr [RSP + local_1c0]
                 24 48
        001013b9 e8 a2 fc        CALL       <EXTERNAL>::memcmp                               int memcmp(void * __s1, void * _
                 ff ff
        001013be 41 89 c4        MOV        R12D,EAX
```

We need to find out where the program is loaded in memory.

Run `gdb -q not-crypto` then execute `starti` to start the program and break at the first instruction.  
Then run `vmmap` to get the memory map

```text
gef➤  vmmap
[ Legend:  Code | Heap | Stack ]
Start              End                Offset             Perm Path
0x0000555555554000 0x0000555555555000 0x0000000000000000 r-- /mnt/hgfs/CTFs/picoCTF/picoMini_by_redpwn/Reverse_Engineering/not_crypto/not-crypto
0x0000555555555000 0x0000555555556000 0x0000000000001000 r-x /mnt/hgfs/CTFs/picoCTF/picoMini_by_redpwn/Reverse_Engineering/not_crypto/not-crypto
0x0000555555556000 0x0000555555557000 0x0000000000002000 r-- /mnt/hgfs/CTFs/picoCTF/picoMini_by_redpwn/Reverse_Engineering/not_crypto/not-crypto
0x0000555555557000 0x0000555555559000 0x0000000000002000 rw- /mnt/hgfs/CTFs/picoCTF/picoMini_by_redpwn/Reverse_Engineering/not_crypto/not-crypto
0x00007ffff7fc5000 0x00007ffff7fc9000 0x0000000000000000 r-- [vvar]
0x00007ffff7fc9000 0x00007ffff7fcb000 0x0000000000000000 r-x [vdso]
0x00007ffff7fcb000 0x00007ffff7fcc000 0x0000000000000000 r-- /usr/lib/x86_64-linux-gnu/ld-linux-x86-64.so.2
0x00007ffff7fcc000 0x00007ffff7ff1000 0x0000000000001000 r-x /usr/lib/x86_64-linux-gnu/ld-linux-x86-64.so.2
0x00007ffff7ff1000 0x00007ffff7ffb000 0x0000000000026000 r-- /usr/lib/x86_64-linux-gnu/ld-linux-x86-64.so.2
0x00007ffff7ffb000 0x00007ffff7fff000 0x0000000000030000 rw- /usr/lib/x86_64-linux-gnu/ld-linux-x86-64.so.2
0x00007ffffffde000 0x00007ffffffff000 0x0000000000000000 rw- [stack]
```

For this to work you need to have [GEF (GDB Enhanced Features)](https://github.com/hugsy/gef) installed.
We can see from the output that the program's image base is at `0x0000555555554000`.

To rebase the program in Ghidra we select `Memory Map` in the `Window menu` and then press the `Set Image Base` icon that looks like a house. Set the image base to `0x0000555555554000`.

Now we can see the real memory addresses

```text
    5555555553b4 48 8b 7c        MOV        RDI,qword ptr [RSP + local_1c0]
                 24 48
    5555555553b9 e8 a2 fc        CALL       <EXTERNAL>::memcmp                               int memcmp(void * __s1, void * _
                 ff ff
    5555555553be 41 89 c4        MOV        R12D,EAX
```

### Set breakpoint in GDB and run to get the flag

Now we know that we should break at `0x5555555553b9`.

Restart GDB and set this as a breakpoint

```text
gef➤  break *0x5555555553b9
Breakpoint 1 at 0x5555555553b9
```

Then `run` again and input some random data.

When we hit the breakpoint we see a partial flag pointed to by the RDI register

```text
memcmp@plt (
   $rdi = 0x00007fffffffdcb0 → "picoCTF{c0mp1l3r_0pt1m1z4t10n_15_pur3_w1z4rdry_but[...]",
   $rsi = 0x00007fffffffdba0 → "dfgfdgdf\ndfgdfgdfgdfggfgdfgdfgd\ndfgertgwrsdfvdfg[...]",
   $rdx = 0x0000000000000040
)
```

Get the full flag with

```text
gef➤  x/s $rdi
0x7fffffffdcb0: "picoCTF{c0mp1l3r_0pt1m1z4t10n_<REDACTED>}\n"
```

For additional information, please see the references below.

### References

- [file - Linux manual page](https://man7.org/linux/man-pages/man1/file.1.html)
- [GDB (The GNU Project Debugger) - Documentation](https://sourceware.org/gdb/documentation/)
- [GDB (The GNU Project Debugger) - Homepage](https://sourceware.org/gdb/)
- [GEF (GDB Enhanced Features) - Documentation](https://hugsy.github.io/gef/)
- [GEF (GDB Enhanced Features) - GitHub](https://github.com/hugsy/gef)
- [Ghidra - Homepage](https://ghidra-sre.org/)
- [Position-independent code - Wikipedia](https://en.wikipedia.org/wiki/Position-independent_code)
- [strace - Linux manual page](https://man7.org/linux/man-pages/man1/strace.1.html)
- [strings - Linux manual page](https://man7.org/linux/man-pages/man1/strings.1.html)
