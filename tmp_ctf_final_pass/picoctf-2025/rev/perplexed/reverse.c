// reversed by binaryninja
#include <stddef.h>
#include <stdio.h>
#define uint unsigned int

int main()
{
  char param_1[0x1c] = {0}; // remove input and set it here manually to all zeroes
  size_t sVar1;
  uint uVar2;
  size_t sVar3;
  char local_58 [36];
  uint local_34;
  uint local_30;
  int local_28;
  uint local_24;
  int local_20;
  int local_1c;
  
  sVar1 = 0x1b; // set the length to a constant
  if (sVar1 == 0x1b) {
    local_58[0] = -0x1f;
    local_58[1] = -0x59;
    local_58[2] = '\x1e';
    local_58[3] = -8;
    local_58[4] = 'u';
    local_58[5] = '#';
    local_58[6] = '{';
    local_58[7] = 'a';
    local_58[8] = -0x47;
    local_58[9] = -99;
    local_58[10] = -4;
    local_58[0xb] = 'Z';
    local_58[0xc] = '[';
    local_58[0xd] = -0x21;
    local_58[0xe] = 'i';
    local_58[0xf] = 0xd2;
    local_58[0x10] = -2;
    local_58[0x11] = '\x1b';
    local_58[0x12] = -0x13;
    local_58[0x13] = -0xc;
    local_58[0x14] = -0x13;
    local_58[0x15] = 'g';
    local_58[0x16] = -0xc;
    local_1c = 0;
    local_20 = 0;
    for (local_24 = 0; local_24 < 0x17; local_24 = local_24 + 1) {
      for (local_28 = 0; local_28 < 8; local_28 = local_28 + 1) {
        if (local_20 == 0) {
          local_20 = 1;
        }
        local_30 = 1 << (7U - (char)local_28 & 0x1f);
        local_34 = 1 << (7U - (char)local_20 & 0x1f);
        uint value_of_flag = 0 < (int)((int)local_58[(int)local_24] & local_30); // get the value of the flag at the bit
        param_1[local_1c] |= value_of_flag << (7 - (char)local_20 & 0x1f);  // instead of comparing set the value
 
        local_20 = local_20 + 1;
        if (local_20 == 8) {
          local_20 = 0;
          local_1c = local_1c + 1;
        }
        sVar3 = (size_t)local_1c;
        sVar1 = 0x1b;
        if (sVar3 == sVar1) {
          printf("%s", param_1);
          return 0;
        }
      }
    }
    uVar2 = 0;
  } else {
    uVar2 = 1;
  }
  printf("%s", param_1); // print the flag at the end
  return uVar2;
}