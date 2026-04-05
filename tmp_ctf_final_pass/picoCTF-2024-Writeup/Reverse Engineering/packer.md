# Description

Reverse this linux executable? <br>
binary

# Solution

Here is a better formatted version of this writeup on [picoCTF Solutions website](https://picoctfsolutions.com/picoctf-2024-packer).

To get the file: `wget https://artifacts.picoctf.net/c_titan/22/out`

By looking at the strings with `strings out` it can be seen that it was compressed with upx. It could be decompressed with `upx -d out`. By looking at the hint you could use strip to reduce the size of a binary `strip out`.

By putting it in [Ghidra](https://ghidra-sre.org/) and analyzing the program the entry function would be a good place to start. There is a parameter to the entry function, `FUN_00401d65`, and when double-clicking it in Ghidra it moves to that function. When scrolling down to see the contents of the function many prompts can be seen with one of them being this:

"Password correct, please see flag: 7069636f4354467b5539585f556e5034636b314e365f42316e34526933535f35646565343434317d"

By putting the Base64 text in [CyberChef](https://gchq.github.io/CyberChef/#recipe=From_Hex('None')) the flag is given.

Flag: `picoCTF{U9X_UnP4ck1N6_B1n4Ri3S_5de...}`
