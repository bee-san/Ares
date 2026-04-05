# Description

How well can you perfom basic binary operations? <br>
Start searching for the flag here nc titan.picoctf.net <br>
62817

# Solution

Here is a better formatted version of this writeup on [picoCTF Solutions website](https://picoctfsolutions.com/picoctf-2024-binhexa).

By connecting with `nc titan.picoctf.net 62817` it gives a series of 6 questions to perform operations on two binary numbers. The questions change each time, but in this case here are the numbers:

**Binary Number 1:** 11010100 <br>
**Binary Number 2:** 00100001

[picoCTF Solutions](https://picoctfsolutions.com/tools/binary-calculator) has a good binary calculator that is used for most of the operators. Note: Be sure that both numbers are set to binary.

[**Operation 1: '&'**](https://picoctfsolutions.com/tools/binary-calculator?mode=BIT_AND) <br>
Perform the operation on Binary Number 1&2. <br>
Result: 00000000

[**Operation 2: '|'**](https://picoctfsolutions.com/tools/binary-calculator?mode=BIT_OR) <br>
Perform the operation on Binary Number 1&2. <br>
Enter the binary result: 11110101

[**Operation 3: '*'**](https://picoctfsolutions.com/tools/binary-calculator?mode=MUL) <br> 
Perform the operation on Binary Number 1&2. <br>
Enter the binary result: 1101101010100

[**Operation 4: '<<'**](https://picoctfsolutions.com/tools/bit-shift-calculator?base=2&direction=LEFT&bits=1) <br>
Perform a left shift of Binary Number 1 by 1 bits. <br>
Enter the binary result: 110101000

For bit shifting a [bit shift calculator](https://picoctfsolutions.com/tools/bit-shift-calculator?base=2&bits=1) is easier to use. Note: Be sure to change the "Number Base" to Binary, the correct number referenced in the question, the correct directional shift operation, and the amount of bits to be shifted to 1 bit.

[**Operation 5: '+'**](https://picoctfsolutions.com/tools/binary-calculator?mode=ADD) <br>
Perform the operation on Binary Number 1&2. <br>
Enter the binary result: 11110101

[**Operation 6: '>>'**](https://picoctfsolutions.com/tools/bit-shift-calculator?base=2&direction=RIGHT&bits=1) <br>
Perform a right shift of Binary Number 2 by 1 bits. <br>
Enter the binary result: 10000

**Enter the results of the last operation in hexadecimal:**

For this, a [Binary to Hex converter](https://picoctfsolutions.com/tools/binary-to-hex) could be used. Specifically for the number "10000" in hex, it is 10. By entering that hex number the program gives the flag.

Flag: `picoCTF{b1tw^3se_0p3eR@tI0n_su33essFuL_aea...}`
