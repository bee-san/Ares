# Description

Know of little and big endian? <br>
nc titan.picoctf.net 61087. Source

# Solution

Here is a better formatted version of this writeup on [picoCTF Solutions website](https://picoctfsolutions.com/picoctf-2024-endianness).

First connect to the server: `nc titan.picoctf.net 61087`

Then you are given a word, in this case, "ffoxf". By using [CyberChef](https://gchq.github.io/CyberChef/#recipe=To_Hex('None',0)), convert the word to hex. This should give you the Big Endian representation of the word.

Big Endian: `66666f7866`

To get little-endian representation you can use online sites like [this](https://www.save-editor.com/tools/wse_hex.html) or just do it by hand.

Big Endian (space delimeter): `66 66 6f 78 66`

Little Endian (space delimeter): `66 78 6f 66 66`

It can be seen that by splitting to each hex character and then flipping the orientation the little-endian representation is achieved. Submit the hex values of the corresponding endian with no space delimiter to get the flag.

Little Endian: `66786F6666`

Flag: `picoCTF{3ndi4n_sw4p_su33ess_d58...}`
