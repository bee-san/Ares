# Description

Can you handle APKs? <br>
Download the android apk here.

# Solution

Here is a better formatted version of this writeup on [picoCTF Solutions website](https://picoctfsolutions.com/picoctf-2024-mob-psycho).

To get the file: `wget https://artifacts.picoctf.net/c_titan/53/mobpsycho.apk`, then `unzip mobpsycho.apk`. Note: Be sure to be the folder you want all the files to be located. 

It gives a lot of files to look through so by using various common search terms like ctf, pico, and flag this command showed something useful `strings * | grep flag`. It shows `res/color/flag.txt` near the end of the output.

To verify, `ls -R | grep flag -B 20`, which recursively lists out everything and got the 20 lines above flag.txt to see that it is indeed at file in ./res/color directory.

Then the outputs of flag.txt could be retrieved with `cat res/color/flag.txt` which gives encoded text. You could take the output and put it into [CyberChef](https://gchq.github.io/CyberChef/#recipe=From_Hex('None')) and decode with Hex or use xxd as shown in this command: `cat res/color/flag.txt | xxd -r -p`. This gives the flag.

Flag: `picoCTF{ax8mC0RU6ve_NX85l4ax8mCl_a3e...}`
