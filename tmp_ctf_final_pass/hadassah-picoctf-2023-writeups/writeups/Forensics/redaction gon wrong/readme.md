# GET aHEAD 

This is the write-up for the challenge "redaction gone wrong" challenge in PicoCTF

# The challenge

## Description
Now you DON’T see me.
This report has some critical data in it, some of which have been redacted correctly, while some were not.
Can you find an important key that was not redacted properly?
![first - Copy](https://github.com/slashben/hadassah-picoctf-2023-writeups/assets/116361337/a04d7ecb-86c8-48f6-884f-1d8d0de62f68)



## Hints
1. How can you be sure of the redaction?

## Initial look
i downloaded the givven pdf file and when i opened it i saw the part 
of it is hidden from me and i understood that i need to find to hidden part. 

# How to solve it

i went to to clue that
said: "How can you be sure of the redaction?" and i understood the the redaction maybe wan't that good.
so in order to find the hiiden part i used Convert PDF To Text website to convert my pdf to text because i
wanted to see my file in it's initial form. after i did it i downloaded the text file that i got from the 
converter and i saw the full text include the hidden parts. in addution i saw the hidden flag:
picoCTF{C4n_Y0u_S33_m3_fully} and than i submutted the flug and finished the exersize.

![second - Copy](https://github.com/slashben/hadassah-picoctf-2023-writeups/assets/116361337/82dba5d7-164e-4657-8e20-42817a75ed09)


picoCTF{C4n_Y0u_S33_m3_fully}

Voila!!! 😎

The flag is `picoCTF{C4n_Y0u_S33_m3_fully}`


Cheers 😄
