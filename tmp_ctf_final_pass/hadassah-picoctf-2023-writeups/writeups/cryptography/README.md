# GET aHEAD 

This is the write-up for the challenge "13" challenge in PicoCTF

# The challenge

## Description
Find the encrypted flag behind the string cvpbPGS{abg_gbb_onq_bs_n_ceboyrz}.
![screen](https://user-images.githubusercontent.com/116361337/235448176-07cc844b-37a9-4c30-bc20-bece6e76d8a7.jpg)



## Hints
1. This can be solved online if you don't want to do it by hand!

## Initial look
the string doesn't tell us a thing- all we kmow is that it's the encryption of the original flag. In addition, we didn't
get any other hint. we are not taken to any other website or something like that. so i tried to think about the encryption method i am familier with. 
 
# How to solve it

I looked at the hint - "This can be solved online if you don't want to do it by hand" what means that there is a formula
that can help me. I figured it has to do something with the number 13(it's the name of the riddle) 
and i remembered that i learned about the rotating ciphyer i learned about in another course. and it's looked reasonabke to me because in order
to use it we should decide on a number to rotate any letter. so i chose 13. and i rotated all the letters 13 times with the website https://rot13.com/
so it looked like that:

![rotate](https://user-images.githubusercontent.com/116361337/235449466-504c0ffc-b40d-4a23-ba8b-2fa029b618e6.jpg)

picoCTF{not_too_bad_of_a_problem}

Voila!!! 😎

The flag is `picoCTF{not_too_bad_of_a_problem}`


Cheers 😄
