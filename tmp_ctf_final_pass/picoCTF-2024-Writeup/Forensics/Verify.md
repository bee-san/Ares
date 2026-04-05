# Description

People keep trying to trick my players with imitation <br>
flags. I want to make sure they get the real thing! I'm <br>
going to provide the SHA-256 hash and a decrypt script <br>
to help you know that my flags are legitimate.

You can download the challenge files here:
* challenge.zip

The same files are accessible via SSH here: <br>
`ssh -p 60327 ctf-player@rhea.picoctf.net` <br>
Using the password `6dd28e9b`. Accept the fingerprint <br>
with yes, and ls once connected to begin. Remember, <br>
in a shell, passwords are hidden! <br>
* Checksum: <br>03b52eabed517324828b9e09cbbf8a7b0911f348f<br>76cf989ba6d51acede6d5d8
* To decrypt the file once you've verified the hash, <br>run ./decrypt.sh files/<file>.

# Solution

Here is a better formatted version of this writeup on [picoCTF Solutions website](https://picoctfsolutions.com/picoctf-2024-verify).

To get the file: `wget https://artifacts.picoctf.net/c_rhea/12/challenge.zip`, then `unzip challenge.zip`. Note: The files are also accessible with the provided ssh in the description. Use `cd home/ctf-player/drop-in` to get to the files.

When you use `ls` three things can be seen: checksum.txt, decrypt.sh, and a files directory. With `cat checksum.txt` the SHA256 hash of the real thing could be seen. In the files directory, there are 301 files with 8 8-character randomized names all with different checksums and contents. Lastly, decrypt.sh is the script provided that could be used on the correct file to get the flag.

If one of the checksums of the files in the files directory matches the checksum provided by checksum.txt then that file used with the decrypt script will give the flag.

Contents of checksum.txt: `03b52eabed517324828b9e09cbbf8a7b0911f348f76cf989ba6d51acede6d5d8`

The `sha256sum` command can be used on a file to get the checksum. To get the checksum of all the files in the directory the `sha256sum files/*` command could be used. When paired with grep command with the known SHA256 checksum that needs to be found the correct file is displayed.

Command: `sha256sum files/* | grep 03b52eabed517324828b9e09cbbf8a7b0911f348f76cf989ba6d51acede6d5d8`

Output: `03b52eabed517324828b9e09cbbf8a7b0911f348f76cf989ba6d51acede6d5d8  files/00011a60`

Lastly, the decrypt script could be run, `./decrypt.sh files/00011a60`, which gives the flag.

Note that if you aren't ssh'd in or in the correct folder the script would have to be modified. Alternatively, the command from the script could be run:

`openssl enc -d -aes-256-cbc -pbkdf2 -iter 100000 -salt -in files/00011a60 -k picoCTF`

Another method of doing it is trying to decrypt every file to get the flag:

`for file in files/*; do openssl enc -d -aes-256-cbc -pbkdf2 -iter 100000 -salt -in "$file" -k picoCTF; done > flag.txt`

This iterates through every file in files/ directory and decrypts. If it was done successfully then it would be put into flag.txt file. With `cat flag.txt` the flag could be seen.

Flag: `picoCTF{trust_but_verify_0...}`
