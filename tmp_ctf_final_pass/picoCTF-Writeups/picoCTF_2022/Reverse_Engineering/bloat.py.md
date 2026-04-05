# bloat.py

- [Challenge information](#challenge-information)
- [Solution](#solution)
- [References](#references)

## Challenge information

```text
Level: Medium
Tags: picoCTF 2022, Reverse Engineering, obfuscation
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: LT 'SYREAL' JONES

Description:
Can you get the flag?

Run this Python program in the same directory as this encrypted flag.
 
Hints:
(None)
```

Challenge link: [https://play.picoctf.org/practice/challenge/256](https://play.picoctf.org/practice/challenge/256)

## Solution

Let's start by looking at the Python source code given

```python
import sys
a = "!\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ"+ \
            "[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~ "
def arg133(arg432):
  if arg432 == a[71]+a[64]+a[79]+a[79]+a[88]+a[66]+a[71]+a[64]+a[77]+a[66]+a[68]:
    return True
  else:
    print(a[51]+a[71]+a[64]+a[83]+a[94]+a[79]+a[64]+a[82]+a[82]+a[86]+a[78]+\
a[81]+a[67]+a[94]+a[72]+a[82]+a[94]+a[72]+a[77]+a[66]+a[78]+a[81]+\
a[81]+a[68]+a[66]+a[83])
    sys.exit(0)
    return False
def arg111(arg444):
  return arg122(arg444.decode(), a[81]+a[64]+a[79]+a[82]+a[66]+a[64]+a[75]+\
a[75]+a[72]+a[78]+a[77])
def arg232():
  return input(a[47]+a[75]+a[68]+a[64]+a[82]+a[68]+a[94]+a[68]+a[77]+a[83]+\
a[68]+a[81]+a[94]+a[66]+a[78]+a[81]+a[81]+a[68]+a[66]+a[83]+\
a[94]+a[79]+a[64]+a[82]+a[82]+a[86]+a[78]+a[81]+a[67]+a[94]+\
a[69]+a[78]+a[81]+a[94]+a[69]+a[75]+a[64]+a[70]+a[25]+a[94])
def arg132():
  return open('flag.txt.enc', 'rb').read()
def arg112():
  print(a[54]+a[68]+a[75]+a[66]+a[78]+a[76]+a[68]+a[94]+a[65]+a[64]+a[66]+\
a[74]+a[13]+a[13]+a[13]+a[94]+a[88]+a[78]+a[84]+a[81]+a[94]+a[69]+\
a[75]+a[64]+a[70]+a[11]+a[94]+a[84]+a[82]+a[68]+a[81]+a[25])
def arg122(arg432, arg423):
    arg433 = arg423
    i = 0
    while len(arg433) < len(arg432):
        arg433 = arg433 + arg423[i]
        i = (i + 1) % len(arg423)        
    return "".join([chr(ord(arg422) ^ ord(arg442)) for (arg422,arg442) in zip(arg432,arg433)])
arg444 = arg132()
arg432 = arg232()
arg133(arg432)
arg112()
arg423 = arg111(arg444)
print(arg423)
sys.exit(0)
```

Oh, well. Obfuscation was the word.

### First iteration of deobfuscation

Let's try starting to make some sense of this by looking up all string concatenations.  
This can be done in an interactive Python session like this

```python
>>> a = "!\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ"+ \
            "[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~ "
>>> a[71]+a[64]+a[79]+a[79]+a[88]+a[66]+a[71]+a[64]+a[77]+a[66]+a[68]
'happychance'
```

If we also insert some empty lines before and after each function declaration we get this

```python
import sys

a = "!\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ"+ \
            "[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~ "
            
def arg133(arg432):
  if arg432 == 'happychance':
    return True
  else:
    print('That password is incorrect')
    sys.exit(0)
    return False
    
def arg111(arg444):
  return arg122(arg444.decode(), 'rapscallion')

def arg232():
  return input('Please enter correct password for flag: ')
  
def arg132():
  return open('flag.txt.enc', 'rb').read()
  
def arg112():
  print('Welcome back... your flag, user:')
  
def arg122(arg432, arg423):
    arg433 = arg423
    i = 0
    while len(arg433) < len(arg432):
        arg433 = arg433 + arg423[i]
        i = (i + 1) % len(arg423)        
    return "".join([chr(ord(arg422) ^ ord(arg442)) for (arg422,arg442) in zip(arg432,arg433)])
    
arg444 = arg132()
arg432 = arg232()
arg133(arg432)
arg112()
arg423 = arg111(arg444)
print(arg423)
sys.exit(0)
```

This is actually all we need. We now have the correct password and can get the flag

```bash
┌──(kali㉿kali)-[/picoCTF/picoCTF_2022/Reverse_Engineering/Bloat.py]
└─$ python bloat.flag.py
Please enter correct password for flag: happychance
Welcome back... your flag, user:
picoCTF{<REDACTED>}
```

### Second iteration of deobfuscation

But let's keep going for practice and give the functions and global variables better names.

- `arg133` can be `verify_pw`
- `arg232` can be `read_pw`
- `arg132` can be `read_enc_flag`
- `arg112` can be `greeting`
- `arg444` can be `flag`
- `arg432` can be `password`

The string `a` is now unused and can be removed.

The code now looks like this

```python
import sys
            
def verify_pw(password):
  if password == 'happychance':
    return True
  else:
    print('That password is incorrect')
    sys.exit(0)
    return False
    
def arg111(flag):
  return arg122(flag.decode(), 'rapscallion')

def read_pw():
  return input('Please enter correct password for flag: ')
  
def read_enc_flag():
  return open('flag.txt.enc', 'rb').read()
  
def greeting():
  print('Welcome back... your flag, user:')
  
def arg122(arg432, arg423):
    arg433 = arg423
    i = 0
    while len(arg433) < len(arg432):
        arg433 = arg433 + arg423[i]
        i = (i + 1) % len(arg423)        
    return "".join([chr(ord(arg422) ^ ord(arg442)) for (arg422,arg442) in zip(arg432,arg433)])
    
flag = read_enc_flag()
password = read_pw()
verify_pw(password)
greeting()
arg423 = arg111(flag)
print(arg423)
sys.exit(0)
```

### Third iteration of deobfuscation and simplification

If we look closely we can see that the decryption of the flag is independent from the `happychance` password.  
So the functions for promping and verifying that password can be removed. As well as the greeting.  
Let's also remove the sys module and the unnecessary last call to `sys.exit`.

Then we have this

```python
def arg111(flag):
  return arg122(flag.decode(), 'rapscallion')
  
def read_enc_flag():
  return open('flag.txt.enc', 'rb').read()
  
def arg122(arg432, arg423):
    arg433 = arg423
    i = 0
    while len(arg433) < len(arg432):
        arg433 = arg433 + arg423[i]
        i = (i + 1) % len(arg423)        
    return "".join([chr(ord(arg422) ^ ord(arg442)) for (arg422,arg442) in zip(arg432,arg433)])
    
flag = read_enc_flag()
arg423 = arg111(flag)
print(arg423)
```

### Fourth iteration of deobfuscation and simplification

Ah, I've made a minor mistake. The variable `flag` should rather be called `enc_flag` since the plaintext flag is `arg423`.

Let's give the function `arg122` a new name, say `decode_flag`. Let's also change the name of the functions arguments.
We then get

```python
enc_flag = open('flag.txt.enc', 'rb').read()
  
def decode_flag(enc_flag_str, password):
    arg433 = password
    i = 0
    while len(arg433) < len(enc_flag_str):
        arg433 = arg433 + password[i]
        i = (i + 1) % len(password)        
    return "".join([chr(ord(arg422) ^ ord(arg442)) for (arg422,arg442) in zip(enc_flag_str, arg433)])
    
flag = decode_flag(enc_flag.decode(), 'rapscallion')
print(flag)
```

The result isn't perfect but let's end the deobfuscation exercise there.

For additional information, please see the references below.

## References

- [Obfuscation (software) - Wikipedia](https://en.wikipedia.org/wiki/Obfuscation_(software))
- [python - Linux manual page](https://linux.die.net/man/1/python)
- [Python (programming language) - Wikipedia](https://en.wikipedia.org/wiki/Python_(programming_language))
