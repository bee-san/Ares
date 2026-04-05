# Description

If you can find the flag on this disk image, we can close <br>
the case for good! <br>
Download the disk image here.

# Solution

Here is a better formatted version of this writeup on [picoCTF Solutions website](https://picoctfsolutions.com/picoctf-2024-dear-diary).

`wget https://artifacts.picoctf.net/c_titan/63/disk.flag.img.gz`, then `gunzip -d disk.flag.img.gz`.

Then put the disk image in Autopsy and load it in with analysis. Once loaded in the root directory has three files: `force-wait.sh`, `innocuous-file.txt`, and `its-all-in-the-name`.

Based on this and looking through the disk image and exhausting many options then coming back to the root directory this file, `its-all-in-the-name`, alludes to `innocuous-file.txt` being important somehow. When searching for `innocuous-file` throughout the entire disk image in Autopsy 14 occurrences of `innocuous-file.txt` could be found.

Each one has Ascii information and the flag is fragmented between multiple of these `innocuous-file.txt` files. In the fourth occurrence of `innocuous-file.txt` the letters `pic` show up and in the next occurrence `oCT` which leads to the assumption that others would create the flag. By combining all of these the flag can be found.

Flag: `picoCTF{1_533_n4m35_80d2...}`
