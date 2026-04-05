# Description

Someone's commits seems to be preventing the <br>
program from working. Who is it?

You can download the challenge files here:
* challenge.zip

# Solution

Here is a better formatted version of this writeup on [picoCTF Solutions website](https://picoctfsolutions.com/picoctf-2024-blame-game).

To get the file: `wget https://artifacts.picoctf.net/c_titan/74/challenge.zip`. Then `unzip challenge.zip` and `cd drop-in/`.

Unlike ["Time Machine"](https://picoctfsolutions.com/picoctf-2024-time-machine) challenge it can't be found just with the `git log` command. Because there could be many changes across the project you can use the command on one specified file. In this case, it is `git log message.py` as that is the file in question. By doing this you can see that the author of the commit to "optimize file size of prod code" is the flag.

Flag: `picoCTF{@sk_th3_1nt3rn_ea3...}`
