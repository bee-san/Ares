# Description

I've gotten bored of handing out flags as text. Wouldn't <br>
it be cool if they were an image instead? 

You can download the challenge files here:
* challenge.zip

The same files are accessible via SSH here: <br>
`ssh -p 55235 ctf-player@atlas.picoctf.net` <br>
Using the password `6dd28e9b`. Accept the fingerprint <br>
with yes, and ls once connected to begin. Remember, <br>
in a shell, passwords are hidden!

# Solution

Here is a better formatted version of this writeup on [picoCTF Solutions website](https://picoctfsolutions.com/picoctf-2024-scan-surprise).

To get the file: `wget https://artifacts.picoctf.net/c_atlas/3/challenge.zip`, then `unzip challenge.zip`. Note: The files are also accessible with the provided ssh in the description. Use `cd home/ctf-player/drop-in` to get to `flag.png`.

Once there you can open the image and use a phone to scan the QR code and get the flag. Although it could also be done in Linux with zbar-tools.

First install the package with `sudo apt install zbar-tools` then to use it run this command:
`zbarimg flag.png`

Flag: `picoCTF{p33k_@_b00_a81...}`
