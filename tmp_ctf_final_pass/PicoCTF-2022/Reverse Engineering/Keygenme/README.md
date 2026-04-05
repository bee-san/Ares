# Keygenme

## Challenge

Can you get the flag? Reverse engineer this [binary](./keygenme).

## Solution

Reverse the program using [Ghidra](https://ghidra-sre.org/). We find the main function:

```c#

undefined8 FUN_0010148b(void)

{
  char cVar1;
  long in_FS_OFFSET;
  char local_38 [40];
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  printf("Enter your license key: ");
  fgets(local_38,0x25,stdin);
  cVar1 = FUN_00101209(local_38);
  if (cVar1 == '\0') {
    puts("That key is invalid.");
  }
  else {
    puts("That key is valid.");
  }
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return 0;
}
```

If `FUN_00101209` returns `0` our key is invalid. So, we want to reverse that function:

```c#
/* WARNING: Could not reconcile some variable overlaps */

undefined8 FUN_00101209(char *param_1)

{
  size_t sVar1;
  undefined8 uVar2;
  long in_FS_OFFSET;
  int local_d0;
  int local_cc;
  int local_c8;
  int local_c4;
  int local_c0;
  undefined2 local_ba;
  byte local_b8 [16];
  byte local_a8 [16];
  undefined8 local_98;
  undefined8 local_90;
  undefined8 local_88;
  undefined4 local_80;
  char local_78 [13];
  undefined local_6b;
  undefined local_6a;
  undefined local_66;
  undefined local_60;
  undefined local_5e;
  undefined local_5b;
  char local_58 [32];
  char acStack56 [40];
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_98 = 0x7b4654436f636970;
  local_90 = 0x30795f676e317262;
  local_88 = 0x6b5f6e77305f7275;
  local_80 = 0x5f7933;
  local_ba = 0x7d;
  sVar1 = strlen((char *)&local_98);
  MD5((uchar *)&local_98,sVar1,local_b8);
  sVar1 = strlen((char *)&local_ba);
  MD5((uchar *)&local_ba,sVar1,local_a8);
  local_d0 = 0;
  for (local_cc = 0; local_cc < 0x10; local_cc = local_cc + 1) {
    sprintf(local_78 + local_d0,"%02x",(ulong)local_b8[local_cc]);
    local_d0 = local_d0 + 2;
  }
  local_d0 = 0;
  for (local_c8 = 0; local_c8 < 0x10; local_c8 = local_c8 + 1) {
    sprintf(local_58 + local_d0,"%02x",(ulong)local_a8[local_c8]);
    local_d0 = local_d0 + 2;
  }
  for (local_c4 = 0; local_c4 < 0x1b; local_c4 = local_c4 + 1) {
    acStack56[local_c4] = *(char *)((long)&local_98 + (long)local_c4);
  }
  acStack56[27] = local_6b;
  acStack56[28] = local_66;
  acStack56[29] = local_5b;
  acStack56[30] = local_78[1];
  acStack56[31] = local_6a;
  acStack56[32] = local_60;
  acStack56[33] = local_5e;
  acStack56[34] = local_5b;
  acStack56[35] = (undefined)local_ba;
  sVar1 = strlen(param_1);
  if (sVar1 == 0x24) {
    for (local_c0 = 0; local_c0 < 0x24; local_c0 = local_c0 + 1) {
      if (param_1[local_c0] != acStack56[local_c0]) {
        uVar2 = 0;
        goto LAB_00101475;
      }
    }
    uVar2 = 1;
  }
  else {
    uVar2 = 0;
  }
LAB_00101475:
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar2;
}
```

So, `param_1` is the user provided key. If the length of that key is not `0x24=36` then the function immediately returns `0`. Thus, the flag will be exactly 36 characters.

The flag/key starts with `picoCTF{br1ng_y0ur_0wn_k3y_` because of the below code snippet:

```c++
local_98 = 0x7b4654436f636970;
local_90 = 0x30795f676e317262;
local_88 = 0x6b5f6e77305f7275;
local_80 = 0x5f7933;
```

Converting [each](https://gchq.github.io/CyberChef/#recipe=From_Hex('Auto')Reverse('Character')&input=MHg3YjQ2NTQ0MzZmNjM2OTcw) [of](https://gchq.github.io/CyberChef/#recipe=From_Hex('Auto')Reverse('Character')&input=MHgzMDc5NWY2NzZlMzE3MjYy) [those](https://gchq.github.io/CyberChef/#recipe=From_Hex('Auto')Reverse('Character')&input=MHg2YjVmNmU3NzMwNWY3Mjc1) [numbers](https://gchq.github.io/CyberChef/#recipe=From_Hex('Auto')Reverse('Character')&input=MHg1Zjc5MzM) to ascii and then reversing them produces `picoCTF{`, `br1ng_y0`, `ur_0wn_k`, and `3y_` respectively. We need to reverse the strings because of little [endianness](https://en.wikipedia.org/wiki/Endianness).

Now, we could try statically reversing the key checking function to get the flag, which I did try, but after many hours it became apparent that a dynamic analysis approach would be much simpler.

(Note that using [GEF](https://github.com/hugsy/gef) to debug this program is easier than GDB. Nevertheless, this writeup uses GDB.)

We can run the binary in gdb and set a breakpoint at `strlen`, since this function is called close to the location that the user input is checked character by character against the `acStack56` variable. So, run `gdb keygenme` and then `break strlen`. Now, run the program with `r` and then enter `c` 17 times to get to the point where we can enter a license key. We enter `picoCTF{br1ng_y0ur_0wn_k3y_AAAAAAAA}`. We use `AAAAAAAA` as the unknown portion since `A=0x41`, which is easy to identify in a hexadecimal memory dump.

Once the dummy key is entered we can keep continuing until the dummy key is in a register. We run `layout reg` and `layout next` to see the registers and assembly at the same time. Now, run `x/32c $rax` to see the first 32 decoded characters starting at the address `$rax` points to. This will show the start of the flag character by character. If we start running `si` to step into the function, we see calls to MD5, so we go to the next breakpoint with `c`. Continuing again once more and running `x/32c $rax` shows that our input is in the rax register.

Now that we have reached the relevant code, we run `s` to step over the string length check. Then, we step in (`si`) repeatedly. When doing this we notice that the loop is taking each character of our input and moving it to the `rdx` register and then moving each character of the valid input to the `rax` register for comparison. Eventually, after spamming `si` long enough we get to a point where `rax=0x31` but `rdx=0x41`. Thus, we have reached the first `A` in our dummy flag. Our key at `rdx` is incorrect so we change the value of `rdx` to the expected value by running `set $rdx=$rax` and note down the correct value. Then, we run `si` and continue setting `$rdx=$rax` and noting down the correct value until we have the entire flag. Running `set $rdx=$rax` is necessary in order for the program to continue checking since if it notices one wrong character the function stops. (Note that it might be possible to set a breakpoint on the `cmp` instruction and skip out on spamming the `si` command.) We can stop getting values once we have 8, since that is the number of unknown characters.

The values I gathered using this method are as follows: `0x31 0x39 0x38 0x33 0x36 0x63 0x64 0x38`. We convert these to ascii using [CyberChef](https://gchq.github.io/CyberChef/#recipe=From_Hex%28%27Auto%27%29&input=MHgzMSAweDM5IDB4MzggMHgzMyAweDM2IDB4NjMgMHg2NCAweDM4) to get `19836cd8`. So, the flag is what we know before plus `19836cd8` plus `}`.

### Flag

`picoCTF{br1ng_y0ur_0wn_k3y_19836cd8}`
