# Description

My team has been working very hard on new features <br>
for our flag printing program! I wonder how they'll <br>
work together?

You can download the challenge files here:
* challenge.zip

# Solution

Here is a better formatted version of this writeup on [picoCTF Solutions website](https://picoctfsolutions.com/picoctf-2024-collaborative-development).

To get the file: `wget https://artifacts.picoctf.net/c_titan/71/challenge.zip`. Then `unzip challenge.zip` and `cd drop-in/`.

With `git branch -a` all the current branches could be seen. There are three feature branches and each one has a part of the flag. You could go to each one and retrieve the flags or you could merge them all to main and deal with the merge conflicts. This is a command that prints all feature branches at once:

`git checkout feature/part-1 && cat flag.py && git checkout feature/part-2 && cat flag.py && git checkout feature/part-3 && cat flag.py`

Flag: `picoCTF{t3@mw0rk_m@k3s_th3_dr3@m_w0rk_4c2...}`
