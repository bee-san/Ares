# Sleuthkit Intro
This is the write-up for the challenge "Sleuthkit Intro" challenge in PicoCTF

#The challenge
Download the disk image and use mmls on it to find the size of the Linux partition. Connect to the remote checker service to check your answer and get the flag.

success it image:
![](imgs/win-screen.png)

##Hints
(None)


## Initial look
The file disk.img.gz is provided to us. I obtained the file by downloading it and then proceeded to extract its contents. After extraction, I utilized the command "$ mmls disk.img" to display the partitions and their respective sizes.

![](imgs/1.png)

I obtained the Linux partition size, which is 0000202752, as required by the challenge. Subsequently, I submitted this partition size to the remote access checker program, resulting in the successful acquisition of the flag.

![](imgs/2.png)

The flag is: picoCTF{mm15_f7w!}
