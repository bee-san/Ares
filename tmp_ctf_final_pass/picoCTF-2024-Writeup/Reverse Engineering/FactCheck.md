# Description

This binary is putting together some important piece of <br>
information... Can you uncover that information? <br>
Examine this file. Do you understand its inner <br>
workings?

# Solution

Here is a better formatted version of this writeup on [picoCTF Solutions website](https://picoctfsolutions.com/picoctf-2024-factcheck).

`wget https://artifacts.picoctf.net/c_titan/187/bin`

Initially, it could be put into [Ghidra](https://ghidra-sre.org/) and analyzed.

Here is a portion of the main function in Ghidra:

```
  std::__cxx11::basic_string<>::basic_string
            ((char *)flagFirstHalf,(allocator *)"picoCTF{wELF_d0N3_mate_");
  std::allocator<char>::~allocator(&local_249);
  std::allocator<char>::allocator();
                    /* try { // try from 0010130a to 0010130e has its CatchHandler @ 00101996 */
  std::__cxx11::basic_string<>::basic_string((char *)char_0,(allocator *)&DAT_0010201d);
  std::allocator<char>::~allocator(&local_249);
  std::allocator<char>::allocator();
                    /* try { // try from 00101345 to 00101349 has its CatchHandler @ 001019b1 */
  std::__cxx11::basic_string<>::basic_string((char *)char_5,(allocator *)&DAT_0010201f);
  std::allocator<char>::~allocator(&local_249);
  std::allocator<char>::allocator();
                    /* try { // try from 00101380 to 00101384 has its CatchHandler @ 001019cc */
  std::__cxx11::basic_string<>::basic_string((char *)char_d,(allocator *)&DAT_00102021);
  std::allocator<char>::~allocator(&local_249);
  std::allocator<char>::allocator();
                    /* try { // try from 001013bb to 001013bf has its CatchHandler @ 001019e7 */
  std::__cxx11::basic_string<>::basic_string((char *)char_3,(allocator *)&DAT_00102023);
  std::allocator<char>::~allocator(&local_249);
  std::allocator<char>::allocator();
                    /* try { // try from 001013f6 to 001013fa has its CatchHandler @ 00101a02 */
  std::__cxx11::basic_string<>::basic_string((char *)char_2,(allocator *)&DAT_00102025);
  std::allocator<char>::~allocator(&local_249);
  std::allocator<char>::allocator();
                    /* try { // try from 00101431 to 00101435 has its CatchHandler @ 00101a1d */
  std::__cxx11::basic_string<>::basic_string((char *)char_a,(allocator *)&DAT_00102027);
  std::allocator<char>::~allocator(&local_249);
  std::allocator<char>::allocator();
                    /* try { // try from 0010146c to 00101470 has its CatchHandler @ 00101a38 */
  std::__cxx11::basic_string<>::basic_string((char *)char_a_2,(allocator *)&DAT_00102027);
  std::allocator<char>::~allocator(&local_249);
  std::allocator<char>::allocator();
                    /* try { // try from 001014a7 to 001014ab has its CatchHandler @ 00101a53 */
  std::__cxx11::basic_string<>::basic_string((char *)char_e,(allocator *)&DAT_00102029);
  std::allocator<char>::~allocator(&local_249);
  std::allocator<char>::allocator();
                    /* try { // try from 001014e2 to 001014e6 has its CatchHandler @ 00101a6e */
  std::__cxx11::basic_string<>::basic_string((char *)char_e_2,(allocator *)&DAT_00102029);
  std::allocator<char>::~allocator(&local_249);
  std::allocator<char>::allocator();
                    /* try { // try from 0010151d to 00101521 has its CatchHandler @ 00101a89 */
  std::__cxx11::basic_string<>::basic_string((char *)char_d_2,(allocator *)&DAT_00102021);
  std::allocator<char>::~allocator(&local_249);
  std::allocator<char>::allocator();
                    /* try { // try from 00101558 to 0010155c has its CatchHandler @ 00101aa4 */
  std::__cxx11::basic_string<>::basic_string((char *)char_b,(allocator *)&DAT_0010202b);
  std::allocator<char>::~allocator(&local_249);
  std::allocator<char>::allocator();
                    /* try { // try from 00101593 to 00101597 has its CatchHandler @ 00101abf */
  std::__cxx11::basic_string<>::basic_string((char *)char_e_3,(allocator *)&DAT_00102029);
  std::allocator<char>::~allocator(&local_249);
  std::allocator<char>::allocator();
                    /* try { // try from 001015ce to 001015d2 has its CatchHandler @ 00101ada */
  std::__cxx11::basic_string<>::basic_string((char *)char_6,(allocator *)&DAT_0010202d);
  std::allocator<char>::~allocator(&local_249);
  std::allocator<char>::allocator();
                    /* try { // try from 00101606 to 0010160a has its CatchHandler @ 00101af5 */
  std::__cxx11::basic_string<>::basic_string((char *)char_c,(allocator *)&DAT_0010202f);
  std::allocator<char>::~allocator(&local_249);
  std::allocator<char>::allocator();
                    /* try { // try from 0010163e to 00101642 has its CatchHandler @ 00101b0d */
  std::__cxx11::basic_string<>::basic_string((char *)char_9,(allocator *)&DAT_00102031);
  std::allocator<char>::~allocator(&local_249);
  std::allocator<char>::allocator();
                    /* try { // try from 00101676 to 0010167a has its CatchHandler @ 00101b25 */
  std::__cxx11::basic_string<>::basic_string((char *)char_8,(allocator *)&DAT_00102033);
  std::allocator<char>::~allocator(&local_249);
                    /* try { // try from 00101699 to 0010185f has its CatchHandler @ 00101b3d */
```

It can be seen that in the above code, the first half of the flag is given. After that, there are many `basic_string<>` functions to where the `&DAT_*` could be clicked to see the associated character. To make it simple, all of the variable names next to `&DAT` were changed to correspond with the associated character. If it was seen more than once then a `_2` or `_3` was added to the end of the variable name.

Here is another portion just below that with the changed variable names:

```
  charater = (char *)std::__cxx11::basic_string<>::operator[]((ulong)char_5);
  if (*charater < 'B') {
    std::__cxx11::basic_string<>::operator+=(flagFirstHalf,char_e_3);
  }

  charater = (char *)std::__cxx11::basic_string<>::operator[]((ulong)char_6);
  if (*charater != 'A') {
    std::__cxx11::basic_string<>::operator+=(flagFirstHalf,char_9);
  }

  charater = (char *)std::__cxx11::basic_string<>::operator[]((ulong)char_3);
  charater2 = *charater;
  charater = (char *)std::__cxx11::basic_string<>::operator[]((ulong)char_e);
  if ((int)charater2 - (int)*charater == 3) {
    std::__cxx11::basic_string<>::operator+=(flagFirstHalf,char_3);
  }

  std::__cxx11::basic_string<>::operator+=(flagFirstHalf,char_d);
  std::__cxx11::basic_string<>::operator+=(flagFirstHalf,char_a);
  charater = (char *)std::__cxx11::basic_string<>::operator[]((ulong)char_a_2);
  if (*charater == 'G') {
    std::__cxx11::basic_string<>::operator+=(flagFirstHalf,char_a_2);
  }

  std::__cxx11::basic_string<>::operator+=(flagFirstHalf,char_2);
  std::__cxx11::basic_string<>::operator+=(flagFirstHalf,char_c);
  std::__cxx11::basic_string<>::operator+=(flagFirstHalf,char_0);
  std::__cxx11::basic_string<>::operator+=(flagFirstHalf,char_e_2);
  std::__cxx11::basic_string<>::operator+=(flagFirstHalf,'}');
```

There are 5 main sections in the code shown above. 

The first part makes this comparison, `'5' < 'B'`, in the ASCII values. By looking at an [ASCII Table](https://www.cs.cmu.edu/~pattis/15-1XX/common/handouts/ascii.html) it can be seen that 5 has a value of 53 and B has a value of 66. This means the statement is true and the character 'e' is appended to the first half of the flag provided earlier: `picoCTF{wELF_d0N3_mate_`. Now the flag would be `picoCTF{wELF_d0N3_mate_e`.

Next, it checks this statement, `'6' != 'A'`, to where '6' is indeed not equal to 'A' so 9 is appended to the flag. The new value of the flag: `picoCTF{wELF_d0N3_mate_e9`.

Next, it does, `charater2 ('3') - character ('e')`, which is `51 - 101` and does not correspond to 3, therefore, does not append the character three to the flag and it stays the same.

Then two lines add the characters 'd' and 'a' with no checks putting the flag to this: `picoCTF{wELF_d0N3_mate_e9da`. Afterwards, it checks if 'a' is equal to 'G' which it is not so it doesn't add anything to that if statement.

Lastly, there are 5 more lines that add characters without any checks to the flag. The characters are '2', 'c', '0', 'e', and '}'. When appended to the end of the current flag it gives the final flag. Note the flag will differ based on the user.

Flag: `picoCTF{wELF_d0N3_mate_e9da2c0e}`
