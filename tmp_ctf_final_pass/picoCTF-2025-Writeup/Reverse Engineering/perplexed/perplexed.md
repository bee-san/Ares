# perplexed #
 
## Overview ##

400 points

Category: [Reverse Engineering](../)

Tags: `#reverseengineering #bruteforce #hash`

## Description ##

Download the binary here.

## Approach ##

Analysis began by disassembling the challenge binary `perplexed` in [Ghidra](https://ghidra-sre.org/).

`main()` reads a string from `stdin` and invokes a `check()` function to process this input.

The `check()` function disassembly:

    1.    undefined8 check(char *param_1)
    2.    {
    3.      size_t sVar1;
    4.      undefined8 uVar2;
    5.      size_t sVar3;
    6.      char local_58 [36];
    7.      uint local_34;
    8.      uint local_30;
    9.      undefined4 local_2c;
    10.     int local_28;
    11.     uint local_24;
    12.     int local_20;
    13.     int local_1c;
    14.   
    15.     sVar1 = strlen(param_1);
    16.     if (sVar1 == 0x1b) {
    17.       local_58[0] = -0x1f;
    18.       local_58[1] = -0x59;
    19.       local_58[2] = '\x1e';
    20.       local_58[3] = -8;
    21.       local_58[4] = 'u';
    22.       local_58[5] = '#';
    23.       local_58[6] = '{';
    24.       local_58[7] = 'a';
    25.       local_58[8] = -0x47;
    26.       local_58[9] = -99;
    27.       local_58[10] = -4;
    28.       local_58[0xb] = 'Z';
    29.       local_58[0xc] = '[';
    30.       local_58[0xd] = -0x21;
    31.       local_58[0xe] = 'i';
    32.       local_58[0xf] = 0xd2;
    33.       local_58[0x10] = -2;
    34.       local_58[0x11] = '\x1b';
    35.       local_58[0x12] = -0x13;
    36.       local_58[0x13] = -0xc;
    37.       local_58[0x14] = -0x13;
    38.       local_58[0x15] = 'g';
    39.       local_58[0x16] = -0xc;
    40.       local_1c = 0;
    41.       local_20 = 0;
    42.       local_2c = 0;
    43.
    44.       // for 0 to 22
    45.       for (local_24 = 0; local_24 < 0x17; local_24 = local_24 + 1) {
    46.         // for 0 to 7
    47.         for (local_28 = 0; local_28 < 8; local_28 = local_28 + 1) {
    48.           // set local_20 to 1 if its zero at the start of the iteration
    49.           if (local_20 == 0) {
    50.             local_20 = 1;
    51.           }
    52.           //  inner loop       = 1 << 7 - (0 to 7)
    53.           local_30 = 1 << (7U - (char)local_28 & 0x1f);
    54.           // 
    56.           local_34 = 1 << (7U - (char)local_20 & 0x1f);
    57.           if (0 < (int)((int)param_1[local_1c] & local_34) !=
    58.               0 < (int)((int)local_58[(int)local_24] & local_30)) {
    59.             // Incorrect
    60.             return 1;
    61.           }
    62.           local_20 = local_20 + 1;
    63.           if (local_20 == 8) {
    64.             local_20 = 0;
    65.             local_1c = local_1c + 1;
    66.           }
    67.           sVar3 = (size_t)local_1c;
    68.           sVar1 = strlen(param_1);
    69.           if (sVar3 == sVar1) {
    70.             // Correct
    71.             return 0;
    72.           }
    73.         }
    74.       }
    75.       uVar2 = 0;
    76.     }
    77.     else {
    78.       // Incorrect
    79.       uVar2 = 1;
    80.     }
    81.     return uVar2;
    82.   }

From this we see straight away that the length of the string must be `0x1b` (or `27`) otherwise no testing of the input is commenced.

Also the `local_58` buffer is initialised with constant content, which from looks of lines `#57 - #60` is the hash that the computed hash from our input is compared to, to determine a correct input string. At the first incorrect byte encountered the `check()` function returns with a `-1` incorrect value.

The series of lines above undertake various bitwise manipulations (and'ing and shift-rights) to convert the input string to a hash to compare against `local_58`.

I didn't spend too long trying to see if this process was reversible, on a hunch I tried inputting a test string that started with the `picoCTF` prefix (the remaining characters padded with 'A's) to see if we continued through this loop, successfully matching the first 6 bytes of the target hash, which it did.

It was at this point I decided to brute force the solution by creating a small program to loop through a subset of the ASCII character set, keeping track of `local_1c` to determine if our current character under test was a match (i.e. the `check()` function would progress to the next byte in the hash to compare). Continuing until we obtained all the characters in the input flag.

## Solution ##

The simple brute force application uses the `check()` function from the Ghidra disassembly with minor modification to store the current successful position of the hash comparison `local_1c` in a global variable `num_solved` for use by the main loop to keep track and store the solved characters.

    #include <stdio.h>
    #include <string.h>

    typedef unsigned char undefined8;
    typedef unsigned int uint;

    char pass[28];
    char solved[28];
    int num_solved = 0;

    undefined8 check(char *param_1)
    {
      size_t sVar1;
      undefined8 uVar2;
      size_t sVar3;
      char local_58 [36];
      uint local_34;
      uint local_30;
      // undefined4 local_2c;
      undefined8 local_2c;
      int local_28;
      uint local_24;
      int local_20;
      int local_1c;
      
      sVar1 = strlen(param_1);
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
        local_2c = 0;

        // for 0 to 22
        for (local_24 = 0; local_24 < 0x17; local_24 = local_24 + 1) {
          // for 0 to 7
          for (local_28 = 0; local_28 < 8; local_28 = local_28 + 1) {
            // set local_20 to 1 if its zero at the start of the iteration
            if (local_20 == 0) {
              local_20 = 1;
            }
            //  inner loop       = 1 << 7 - (0 to 7)
            local_30 = 1 << (7U - (char)local_28 & 0x1f);
            // 
            local_34 = 1 << (7U - (char)local_20 & 0x1f);
            if (0 < (int)((int)param_1[local_1c] & local_34) !=
                0 < (int)((int)local_58[(int)local_24] & local_30)) {
              // Incorrect
              num_solved = local_1c;
              return 1;
            }
            local_20 = local_20 + 1;
            if (local_20 == 8) {
              local_20 = 0;
              local_1c = local_1c + 1;
            }
            sVar3 = (size_t)local_1c;
            sVar1 = strlen(param_1);
            if (sVar3 == sVar1) {
              // Correct
              return 0;
            }
          }
        }
        uVar2 = 0;
      }
      else {
        // Incorrect
        printf("Incorrect (2)\n");    
        uVar2 = 1;
      }
      return uVar2;
    }

    int
    main(void)
    {
      int prev_num_solved = 0;
      undefined8 hash_test = 1;
      while (hash_test != 0)
      {
        for (char cut = 0x20; cut < 0x7f; ++cut)
        {
          if (num_solved > 0)
          {
            strncpy(pass, solved, num_solved);
          }

          memset(&pass[num_solved], 'A', sizeof(pass) - num_solved);
          pass[num_solved] = cut;
          pass[sizeof(pass)-1] = '\0';

          hash_test = check(pass);

          if (num_solved > prev_num_solved)
          {
            printf("Solved %d characters....\n", num_solved);
            memcpy(&solved, &pass, num_solved);
            prev_num_solved = num_solved;
          }

          if ((num_solved > 0) && (solved[num_solved-1] == '}'))
          {
            solved[num_solved] = '\0';
            printf("Flag: %s\n", solved);
            return 0;
          }
        }
      }

      return 0;
    }

Compiling and running the source yields:

    $ gcc -o pwn-perplexed pwn-perplexed.c 
    $ ./pwn-perplexed 
    Solved 1 characters....
    Solved 2 characters....
    Solved 3 characters....
    Solved 4 characters....
    Solved 5 characters....
    Solved 6 characters....
    Solved 7 characters....
    Solved 8 characters....
    Solved 9 characters....
    Solved 10 characters....
    Solved 11 characters....
    Solved 12 characters....
    Solved 13 characters....
    Solved 14 characters....
    Solved 15 characters....
    Solved 16 characters....
    Solved 17 characters....
    Solved 18 characters....
    Solved 19 characters....
    Solved 20 characters....
    Solved 21 characters....
    Solved 22 characters....
    Solved 23 characters....
    Solved 24 characters....
    Solved 25 characters....
    Solved 26 characters....
    Flag: picoCTF{...........redacted.............}

Where the actual flag value has been redacted for the purposes of this write up.
