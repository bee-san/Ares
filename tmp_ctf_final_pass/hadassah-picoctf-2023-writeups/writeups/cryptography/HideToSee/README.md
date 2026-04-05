# HideToSee
This is the write-up for the challenge "HideToSee" challenge in PicoCTF

# The challenge
## Description
How about some hide and seek heh?
Look at this image here.

![](img/1.png)
![](img/2.png)
## Hint
Download the image and try to extract it.

# How to solve it
I attempted to decode the hidden message within the photo using a Steganography Online tool. However, the resulting message was not clear or easily decipherable.
![](img/3.png)
![](img/4.png)

To further investigate, I then tried a different tool called Steganographic Decoder and uploaded the same image.
![](img/5.png)
![](img/5.png)

After successfully decoding the hidden message using a steganographic decoder, I obtained the flag. However, the flag was encrypted, so I proceeded to decrypt it using the Cipher Chef website with the Atbash Cipher. 

![](img/7.png)

The flag is: picoCTF{atbash_crack_1f84d779}

:))
