# Enhance

This is the write-up for the challenge "Enhance" challenge in PicoCTF

# The challenge

## Description
Download this image file and find the flag.

[Download image file](https://artifacts.picoctf.net/c/100/drawing.flag.svg)

<img width="475" alt="image" src="https://github.com/slashben/hadassah-picoctf-2023-writeups/assets/94118086/023e630d-ef45-4faa-9d9e-5bb0677dafc7">

## Hints
none

## Initial look
The above link download basic svg file with big black circle with small white circle in it.


# How to solve it
Open the file with a text editor (like notepad++)

<img width="562" alt="image" src="https://github.com/slashben/hadassah-picoctf-2023-writeups/assets/94118086/5e55d237-0b8f-42a1-9bc7-7801f0887159">

In the lower part of the file, you'll find some characters enclosed in ID tags: {3nh4n} and {c3d_aab729dd}. They appear to resemble a flag,
so I combined them into a single string: {3nh4nc3d_aab729dd}. I presumed this to be the flag and concluded that I only needed to include the picoCTF wrapper.

As a result, the flag is: picoCTF{3nh4nc3d_aab729dd}.

End!
