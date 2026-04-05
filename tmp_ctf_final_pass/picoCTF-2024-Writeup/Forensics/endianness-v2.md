# Description

Here's a file that was recovered from a 32-bits system <br>
that organized the bytes a weird way. We're not even <br>
sure what type of file it is. <br>
Download it here and see what you can get out of it

# Solution

Here is a better formatted version of this writeup on [picoCTF Solutions website](https://picoctfsolutions.com/picoctf-2024-endianness-v2).

By using [CyberChef](https://gchq.github.io/CyberChef/#recipe=To_Hex('Space',0)Swap_endianness('Hex',4,true)From_Hex('Auto')Render_Image('Raw')) the file was put into the input section. Then converted to hex for the "Swap Endianness" function under a word length of 4. After this, the hex looks more like a JPG with the correct `ÿØÿà␀␐JFIF␀␁` magic bytes start. After the endianness was swapped and it's in the correct order it could be converted back from hex to get the data from the image. It could then be rendered in CyberChef to get the image that displays the flag.

Flag: `picoCTF{cert!f1Ed_iNd!4n_s0rrY_3nDian_76e...}`
