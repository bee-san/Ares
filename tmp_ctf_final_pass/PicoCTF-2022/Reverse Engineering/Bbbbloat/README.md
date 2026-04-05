# Bbbbloat

## Challenge

Can you get the flag? Reverse engineer this [binary](./bbbbloat).

## Solution

Reverse the program using [Ghidra](https://ghidra-sre.org/). We see `FUN_00101307` contains most of the code:

```c#
undefined8 FUN_00101307(void)

{
  char *__s;
  long in_FS_OFFSET;
  int local_48;
  undefined8 local_38;
  undefined8 local_30;
  undefined8 local_28;
  undefined8 local_20;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_38 = 0x4c75257240343a41;
  local_30 = 0x3062396630664634;
  local_28 = 0x65623066635f3d33;
  local_20 = 0x4e326560623535;
  printf("What\'s my favorite number? ");
  __isoc99_scanf();
  if (local_48 == 0x86187) {
    __s = (char *)FUN_00101249(0,&local_38);
    fputs(__s,stdout);
    putchar(10);
    free(__s);
  }
  else {
    puts("Sorry, that\'s not it!");
  }
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return 0;
}
```

Our input `local_48` is compared against `0x86187`, which is `549255` in decimal. Running the program and entering `549255` prints the flag:

```
./bbbbloat
What's my favorite number? 549255
picoCTF{cu7_7h3_bl047_36dd316a}
```

### Flag

`picoCTF{cu7_7h3_bl047_36dd316a}`
