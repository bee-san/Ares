# Description

Can you control your overflow?
Download the binary here.
Download the source here.
Connect with the challenge instance here:
nc tethys.picoctf.net 60741

# Solution

Here is a better formatted version of this writeup on [picoCTF Solutions website](https://picoctfsolutions.com/picoctf-2024-heap-1).

To get into the challenge use: `nc tethys.picoctf.net 60741`

From the previous challenge (heap0) it is known that to get to the safe_var you need 32 characters. When looking at the source code it can be seen that the safe_var needs to be set to `pico` to get to the flag. So by writing to the buffer 32 characters and then `pico` safe_var would be written to `pico`. Then by running the fourth option, the flag would be printed.

Flag: `picoCTF{starting_to_get_the_hang_c58...}`
