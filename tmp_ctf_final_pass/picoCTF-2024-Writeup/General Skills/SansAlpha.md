# Description

The Multiverse is within your grasp! Unfortunately, the <br>
server that contains the secrets of the multiverse is in a <br>
universe where keyboards only have numbers and <br>
(most) symbols. <br>
`ssh -p 54983 ctf-player@mimas.picoctf.net` <br>
Use password: 6dd28e9b

# Solution

Here is a better formatted version of this writeup on [picoCTF Solutions website](https://picoctfsolutions.com/picoctf-2024-sansalpha).

This is a challenge of trial and error and could be solved in many different ways. The hint says "Where can you get some letters?" so initially looked for letters that recreate the path `/usr/bin/cat` which could be used to run the file.

Initialized a variable to give command not found: ``` _1=`$ 2>&1` ```. Now when running it with this ``` `"$_1"` ``` this output could be seen:

`bash: bash: $: command not found: command not found`

This gives many letters to work with, and could spell out the letters `cat`:

`${_1:9:1}  - c` <br>
`${_1:1:1}  - a` <br>
`${_1:19:1} - t` <br>
`${_1:2:1}  - s`

With these characters `/usr/bin/cat` was able to be reached with this command: `/?${_1:2:1}?/???/??${_1:19:1}`

However, it showed that it is an invalid operation.

Next, tried to use the echo method of printing out a file like this: `echo "$(<filename)"`

By running, `./*/*`, `./blargh/flag.txt` can be seen which is where the flag is located. So the filename would be `./*/????.???`.

To get echo it is located in `/bin/echo`. Testing could be done on a local machine without restrictions to make sure the command works. All the letters that are needed to get there is `c` which is shown beforehand and `o` which is `${_1:10:1}`.

Running, `/???/?${_1:9:1}?${_1:10:1}`, now will act as the echo command. It could be tested before using however it can now be used to print out the flag with this command:

`/???/?${_1:9:1}?${_1:10:1} "$(<./*/????.???)"`

This gives "return 0" and then the flag. This could be done in many different ways.

Flag: `picoCTF{7h15_mu171v3r53_15_m4dn355_145...}`
