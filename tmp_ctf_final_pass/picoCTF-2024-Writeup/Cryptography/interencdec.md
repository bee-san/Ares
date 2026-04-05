# Description

Can you get the real meaning from this file. <br>
Download the file here.

# Solution

Here is a better formatted version of this writeup on [picoCTF Solutions website](https://picoctfsolutions.com/picoctf-2024-interencdec).

To get the file: `wget https://artifacts.picoctf.net/c_titan/3/enc_flag`

By using `cat enc_flag` command you get this encoded text: `YidkM0JxZGtwQlRYdHFhR3g2YUhsZmF6TnFlVGwzWVROclgya3lNRFJvYTJvMmZRPT0nCg==`

The first method would be by using [CyberChef](https://gchq.github.io/CyberChef/#recipe=From_Base64('A-Za-z0-9%2B/%3D',true,false)Drop_bytes(0,2,false)Drop_bytes(48,1,false)From_Base64('A-Za-z0-9%2B/%3D',true,false)ROT13_Brute_Force(true,true,false,100,0,true,'')) to decode the text. Originally putting it in you can recognize that it is Base64 because of the padding or magic filter will do it for you. It will then give this text: `b'd3BqdkpBTXtqaGx6aHlfazNqeTl3YTNrX2kyMDRoa2o2fQ=='`. 

In CyberChef you can use the Drop Bytes function to remove `b'` and the `'` surrounding the Base64 or you can just copy-paste it up to go through Base64 decoding again. It will then give this text: `wpjvJAM{jhlzhy_k3jy9wa3k_i204hkj6}`. This looks a lot like the format of flags (picoCTF{}) so it is likely a rotation cipher. By applying the ROT13 Brute Force function and then by using control-f to find "picoCTF" the flag is shown.

In the command line it could be done with this: `cat enc_flag | base64 -d | cut -d "'" -f2 | base64 -d | caesar`

The Caesar command is from bsdgames and could be installed like this in debian-based Linux distributions: `sudo apt install bsdgames`.

Flag: `picoCTF{caesar_d3cr9pt3d_b20...}`
