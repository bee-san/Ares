# File types
This is the write-up for the challenge "File types" from picoCTF 2022 (Forensics).
<br>Written by: Idan Cohen

## The Challenge
## Description
This file was found among some files marked confidential but my pdf reader cannot read it, maybe yours can. You can download the file from here.
## Hint
Remember that some file types can contain and nest other files.
![image](https://github.com/slashben/hadassah-picoctf-2023-writeups/assets/48062272/208c9140-ed01-4fe5-8769-9d4f1c598b1a)

## Initial look
The link downloads a file with the name "Flag.pdf", but when I tried to open it, it says "Unable to open document", and it is actually not a pdf file despite the name. 
<br> ![image](https://github.com/slashben/hadassah-picoctf-2023-writeups/assets/48062272/6b24ca61-5f57-4964-936b-93f6ee7a2974)

In order to find the real type of a file you can write the shell command: File [file_name]. This is what I did and I got the file type <b>"shell archive text"</b>:
<br> ![image](https://github.com/slashben/hadassah-picoctf-2023-writeups/assets/48062272/44fd5491-952e-4d52-a38e-abcd8f4be237)

When I read the content of the file, it says that in order to extract the content I have to run the command "sh ./Flag.pdf", but I didn't have this installed on my Linux:
<br> ![image](https://github.com/slashben/hadassah-picoctf-2023-writeups/assets/48062272/2302dd64-08dd-455d-842e-0795f2e22d9e)

So I had to install "sharutils" (shell archive utilities):
<br> ![image](https://github.com/slashben/hadassah-picoctf-2023-writeups/assets/48062272/1f7f89a1-1928-4bad-9ace-0a04c3ddfaab)

Now I could run the command and got another compressed file nested inside the first "pdf" file (that was not a pdf file at all):
<br> ![image](https://github.com/slashben/hadassah-picoctf-2023-writeups/assets/48062272/f909bc77-2755-4e19-ad00-7fd1b6457f5d)
<br> ![image](https://github.com/slashben/hadassah-picoctf-2023-writeups/assets/48062272/41a728b3-4d92-4e1e-9d6a-e21844c9f9e6)

When I check the file type of the nested file, I found it is from type <b>"ar archive"</b>:
<br> ![image](https://github.com/slashben/hadassah-picoctf-2023-writeups/assets/48062272/29178496-f298-49da-9443-086c8149515e)

I checked in the manual how to extract from this file type, and the command is "ar xv [file_name]". When I did it I got another nested file:
<br>![image](https://github.com/slashben/hadassah-picoctf-2023-writeups/assets/48062272/404a04b3-da49-48b9-a8a4-901539723d31)

The new file is from type <b>"cpio archive"</b>:
<br>![image](https://github.com/slashben/hadassah-picoctf-2023-writeups/assets/48062272/2936c34b-6b42-4ce0-b57c-7eb340a8aa5f)

The format is used for copying files to and from archives:
<br>![image](https://github.com/slashben/hadassah-picoctf-2023-writeups/assets/48062272/841bc25f-6f95-44d8-be7a-17551c718d9e)

In order to extract the content, according to the manual I had to run the command "cpio –file [file_name] –extract", but I had to change the original file name from "flag" to another name in order to prevent name collision. When I run this command I got another nested file "flag":
<br>![image](https://github.com/slashben/hadassah-picoctf-2023-writeups/assets/48062272/9ec2abe0-8c6d-40d3-a93c-2cc0ae5cafb2)

The new file was from type: <b>"bzip2"</b>. In order to extract the content I had to use the command "bunzip2". The nested file was from type <b>"gzip"</b>.
<br>![image](https://github.com/slashben/hadassah-picoctf-2023-writeups/assets/48062272/4bda0d00-bd58-4cb5-b40c-8d8024d0603a)

In order to extract the content of this gzip file, I had to run the command "gunzip" (but first to change the suffix from ".out" to ".gzip"), and I got another nested file of type <b>"lzip"</b>.
<br>![image](https://github.com/slashben/hadassah-picoctf-2023-writeups/assets/48062272/3a0d369e-1ec6-4f2f-b4c3-49e3e2252df1)

I had to install lzip on my Linux, and then run the command "lzip -d flag" in order to extract the content. The new nested file was from type <b>"LZ4"</b>.
<br>![image](https://github.com/slashben/hadassah-picoctf-2023-writeups/assets/48062272/671fb7d9-85d9-4cea-82df-c4f0205c293c)

I had to install LZ4 on my Linux, and then extract the file content by this command:
<br>![image](https://github.com/slashben/hadassah-picoctf-2023-writeups/assets/48062272/50f5f809-76ef-42d3-a43b-411acea7e7f1)

Now I got a file of type <b>"lzma"</b>, and in order to extract the content I used the command "unlzma" (but first change the file suffix to .lzma). At this stage, I got a new file of type <b>"lzop"</b>:
<br>![image](https://github.com/slashben/hadassah-picoctf-2023-writeups/assets/48062272/31b51839-6cc5-4a69-b77b-29a34848878e)

I installed "lzop" on my Linux, and then extracted the content using the following command (but first had to change the file name to a new name to prevent name collision). Now I got a new nested file of type <b>"lzip"</b>:
<br>![image](https://github.com/slashben/hadassah-picoctf-2023-writeups/assets/48062272/8ad025ef-631d-4c85-af47-c42dc9a435e6)

In order to extract the content of this lzip file I had to use the command "lzip -d [file_name]" (but first I had to delete a current file called flag.out in this directory, to prevent name collision). The new file revealed inside was of type <b>"XZ"</b>:
<br>![image](https://github.com/slashben/hadassah-picoctf-2023-writeups/assets/48062272/59badab2-286d-451d-9f1a-ad3ffc04878c)

I extracted the XZ file content using "unxz" command, and finally discovered a <b>ASCII text file</b>!
<br>![image](https://github.com/slashben/hadassah-picoctf-2023-writeups/assets/48062272/e97fc11b-c76f-4c77-8b5c-2fd4b1bf4d32)

But when I tried to read this ASCII text file content I discovered that is encoded in hexadecimal form:
<br>![image](https://github.com/slashben/hadassah-picoctf-2023-writeups/assets/48062272/a524928e-4570-402a-8101-cb3d13508208)

So I had to convert it to ASCII characters using the following command, and I discovered the flag 😊 :
<br>![image](https://github.com/slashben/hadassah-picoctf-2023-writeups/assets/48062272/daacb588-e5b6-4c30-98a4-ec81b55afdf1)

The flag is <b>picoCTF{f1len@m3_m@n1pul@t10n_f0r_0b2cur17y_950c4fee}</b>
<br>![image](https://github.com/slashben/hadassah-picoctf-2023-writeups/assets/48062272/4a0ccd76-c088-4118-a497-3dc880b1fc6b)


## Conclusion
<p>When I downloaded the original file from picoCTF site it looks like a regular pdf file, but it wasn't pdf file at all. As the flag says in some obscure way: "filename manipulation for obscurity", the suffix ".pdf" used only for manipulation, obscurity and misleading.</p>
<p>In order to reveal the flag I had to go through 11 different comressed files from different types that was nested in each other, As the hint says: "Remember that some file types can contain and nest other files." The types were: <b>shell archive text, ar archive, cpio archive, bzip2, gzip, lzip, LZ4, lzma, lzop, lzip, XZ.</b></p>  
<p>In order to know how to extract the content of the files I had to look in the manual or in google. Some of the file formats I had to install on my Linux first.</p>
<p>Finally, after a long way I discovered an ASCII text file but it was encoded in hexadecimal format, so I also had to convert it to ASCII characters and reveal the file content – the flag.</p>
