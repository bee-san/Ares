# Description

How about some hide and seek? <br>
Download this file here.

# Solution

Here is a better formatted version of this writeup on [picoCTF Solutions website](https://picoctfsolutions.com/picoctf-2024-canyousee).

To get the file: `wget https://artifacts.picoctf.net/c_titan/6/unknown.zip`, then `unzip unknown.zip`. 

By running `exiftool ukn_reality.jpg` the Attribution URL section looks like it has Base64 encoded text (cGljb0NURntNRTc0RDQ3QV9ISUREM05fYTZkZjhkYjh9Cg==). By putting that text into [CyberChef](https://gchq.github.io/CyberChef/#recipe=From_Base64('A-Za-z0-9%2B/%3D',true,false)) with Base64 decoding the flag could be found.

You could also get the flag in one command: `exiftool ukn_reality.jpg | grep At | cut -d ":" -f2 | tr -d " " | base64 -d`

The first part is getting the line of the Attribution URL section. Then use the cut command with the delimiter of a colon (:) and get the second field. Using the tr command to trim the leading spaces. Lastly, use `base64 -d` to decode the output and get the flag.

Flag: `picoCTF{ME74D47A_HIDD3N_a6d...}`
