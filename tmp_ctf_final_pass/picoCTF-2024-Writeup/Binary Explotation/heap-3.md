# Description

This program mishandles memory. Can you exploit it to <br>
get the flag? <br>
Download the binary here. <br>
Download the source here. <br>
Connect with the challenge instance here: <br>
nc tethys.picoctf.net 52981

# Solution

Here is a better formatted version of this writeup on [picoCTF Solutions website](https://picoctfsolutions.com/picoctf-2024-heap-3).

For testing the binary and source file could be retrieved with this command: `wget https://artifacts.picoctf.net/c_tethys/6/chall https://artifacts.picoctf.net/c_tethys/6/chall.c`

To connect use this command: `nc tethys.picoctf.net 52981`

There are 4 relevant options: `2` is to allocate the object, `3` is to print x, `4` is to check for win, and `5` is to free x.

Since this is "Use after free" as mentioned in the hint and can be seen in the options before doing anything `5` needs to be run to free x.

Then from previous challenges it is known that 32 'A' characters need to be used to get to the x, however in this case it was found that it is 30 characters by testing with the `3` command to see why it doesn't work. Now select the `2` option with a length of 31 for the 30 'A' characters and the "pico" value instead of "bico". Note that it could be seen in the source code that the value needs to be "pico" to get the flag. After inputting the correct length put this payload:

`AAAAAAAAAAAAAAAAAAAAAAAAAAAAAApico`

It can then be checked with the `3` option that it is indeed now "pico". Once that can be seen the `4` option could be run to get the flag.

Flag: `picoCTF{now_thats_free_real_estate_a11...}`
