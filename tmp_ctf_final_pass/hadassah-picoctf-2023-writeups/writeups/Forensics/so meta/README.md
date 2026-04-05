# so meta
This is the write-up for the challenge "so meta" challenge in PicoCTF

# The challenge
https://play.picoctf.org/practice/challenge/19?category=4&page=2

# Description
Find the flag being held on this server to get ahead of the competition https://jupiter.challenges.picoctf.org/static/916b07b4c87062c165ace1d3d31ef655/pico_img.png

picture of done:
![](img/finish.jpg)

# Hints
1.What does meta mean in the context of files? 
2.Ever heard of metadata?

# Initial look
image named "pico_img.png" contain hidden text

# How to solve it
I downloaded the image to the computer, opened the command prompt in the same folder where the image is located.
I wrote the command 'findstr "pico" pico_img.png', the command actually searches for the word "pico"
(a word found in the flag I'm looking for) inside the image.
After I wrote the command, the flag was printed in the command prompt.

![](img/key.png)


The flag is: picoCTF{s0_m3ta_d8944929}

And I'm done  😄
