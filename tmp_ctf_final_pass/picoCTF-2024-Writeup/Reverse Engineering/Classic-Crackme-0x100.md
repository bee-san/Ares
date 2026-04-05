# Description

A classic Crackme. Find the password, get the flag! <br>
Binary can be downloaded here. <br>
Crack the Binary file locally and recover the password. <br>
Use the same password on the server to get the flag! <br>
Access the server using nc titan.picoctf.net 56916

# Solution

Here is a better formatted version of this writeup on [picoCTF Solutions website](https://picoctfsolutions.com/picoctf-2024-classic-crackme-0x100).

To download the binary run this command: `wget https://artifacts.picoctf.net/c_titan/83/crackme100`

When putting into Ghidra this is the main function:

```
undefined8 main(void)

{
  int iVar1;
  size_t sVar2;
  char local_a8 [64];
  undefined8 local_68;
  undefined8 local_60;
  undefined8 local_58;
  undefined8 local_50;
  undefined8 local_48;
  undefined7 local_40;
  undefined4 uStack_39;
  uint local_2c;
  uint local_28;
  char local_21;
  uint local_20;
  uint local_1c;
  uint local_18;
  int local_14;
  int local_10;
  int local_c;
  
  local_68 = 0x676d76727970786c;
  local_60 = 0x7672657270697564;
  local_58 = 0x727166766b716f6d;
  local_50 = 0x6575717670716c62;
  local_48 = 0x796771706d7a7565;
  local_40 = 0x73687478726963;
  uStack_39 = 0x77616a;
  setvbuf(stdout,(char *)0x0,2,0);
  printf("Enter the secret password: ");
  __isoc99_scanf(&DAT_00402024,local_a8);
  local_c = 0;
  sVar2 = strlen((char *)&local_68);
  local_14 = (int)sVar2;
  local_18 = 0x55;
  local_1c = 0x33;
  local_20 = 0xf;
  local_21 = 'a';
  for (; local_c < 3; local_c = local_c + 1) {
    for (local_10 = 0; local_10 < local_14; local_10 = local_10 + 1) {
      local_28 = (local_10 % 0xff >> 1 & local_18) + (local_10 % 0xff & local_18);
      local_2c = ((int)local_28 >> 2 & local_1c) + (local_1c & local_28);
      iVar1 = ((int)local_2c >> 4 & local_20) +
              ((int)local_a8[local_10] - (int)local_21) + (local_20 & local_2c);
      local_a8[local_10] = local_21 + (char)iVar1 + (char)(iVar1 / 0x1a) * -0x1a;
    }
  }
  iVar1 = memcmp(local_a8,&local_68,(long)local_14);
  if (iVar1 == 0) {
    printf("SUCCESS! Here is your flag: %s\n","picoCTF{sample_flag}");
  }
  else {
    puts("FAILED!");
  }
  return 0;
}
```

At first, using the hex values above setvbuf as the cipher text was attempted but the key when inputted was incorrect. When running it through gdb with `gdb crackme100` the correct string could be found. 

To find the right place to break in gdb first look at the objdump with `objdump -D crackme100`. By looking at the main function there are many movabs with large hex values. After every mov is complete but before the setvbuf as could be seen in the Ghidra output a breakpoint could be made. A possible breakpoint could be set at `4011e8` this could be done with `break *0x4011e8` in gdb. Once the breakpoint is set the `run` command could be used then `info local` and the value that is needed is in the output variable and is shown to be this value:

`lxpyrvmgduiprervmoqkvfqrblqpvqueeuzmpqgycirxthsjaw`

In Ghidra changed variable names to make more sense like changing local_c to i and local_10 to j and so on. Once this was done created the Python script shown below to undo the operation:

```
c = "lxpyrvmgduiprervmoqkvfqrblqpvqueeuzmpqgycirxthsjaw"
p = ['' for _ in c]

for i in range(3):
    for j in range(len(c)):
        v7 = (85 & (j % 255)) + (85 & ((j % 255) >> 1))
        v6 = (v7 & 51) + (51 & (v7 >> 2))
        x = (ord(c[j]) - 97) % 26
        y = (x - ((v6 & 15) + (15 & (v6 >> 4)))) % 26
        p[j] = chr(y + 97)

    c = ''.join(p)

print(c)
```

Once the new value is output from the Python script that is used as the password. It could first be tested with crackme100 to see if it gets the test flag and if it does it could be run on the server. To connect use `nc titan.picoctf.net 56916` and submitting the password should get the flag.

Flag: `picoCTF{s0lv3_angry_symb0ls_150f...}`
