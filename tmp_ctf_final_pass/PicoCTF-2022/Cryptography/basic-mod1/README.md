# basic-mod1

## Challenge

We found this weird message being passed around on the servers, we think we have a working decrpytion scheme. Download the message [here](https://artifacts.picoctf.net/c/393/message.txt). Take each number mod 37 and map it to the following character set: 0-25 is the alphabet (uppercase), 26-35 are the decimal digits, and 36 is an underscore. Wrap your decrypted message in the picoCTF flag format (i.e. `picoCTF{decrypted_message}`)

## Solution

Run the solution [script.py](script.py), which directly applies the operations as per the challenge description.

### Flag

`picoCTF{R0UND_N_R0UND_79C18FB3}`
