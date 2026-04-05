# Description

I accidentally wrote the flag down. Good thing I deleted <br>
it!

You download the challenge files here:
* challenge.zip

# Solution

Here is a better formatted version of this writeup on [picoCTF Solutions website](https://picoctfsolutions.com/picoctf-2024-commitment-issues).

To get the file: `wget https://artifacts.picoctf.net/c_titan/77/challenge.zip`. Then `unzip challenge.zip`.

Then `cd drop-in/` and with `ls -a` the ".git" file can now be seen. Originally there was just a file called "message.txt" with file contents "TOP SECRET". You can use the `git log` command to see prior commits made and one of them has a note that says "create flag" with the id of "3d5ec8a26ee7b092a1760fea18f384c35e435139". 

With the checkout functionality you can change to past commits with the IDs like this: `git checkout 3d5ec8a26ee7b092a1760fea18f384c35e435139`. By doing `cat message.txt` now it can be seen that the flag is in the file.

Flag: `picoCTF{s@n1t1z3_30e86...}`
