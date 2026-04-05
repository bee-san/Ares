# Description

The Network Operations Center (NOC) of your local <br>
institution picked up a suspicious file, they're getting<br>
conflicting information on what type of file it is. They've <br>
brought you in as an external expert to examine the <br>
file. Can you extract all the information from this <br>
strange file? <br>
Download the suspicious file here.

# Solution

Here is a better formatted version of this writeup on [picoCTF Solutions website](https://picoctfsolutions.com/picoctf-2024-secret-of-the-polyglot).

To get the file: `wget https://artifacts.picoctf.net/c_titan/9/flag2of2-final.pdf`

First, open it as a pdf to get the 2nd part of the flag. Through the command line, it could be done with `pdftotext` command.

First to install use, `sudo apt install poppler-utils`, then to run the command:
`pdftotext flag2of2-final.pdf`

Then to get the flag use, `cat flag2of2-final.txt`, to get this: `1n_pn9_&_pdf_7f9...}`

When looking at the file with `cat flag2of2-final.pdf`, looking through the hex, or running the file command with `file flag2of2-final.pdf` it could be seen that the magic bytes show the file as a png. By changing the name with this command, `mv flag2of2-final.pdf flag2of2-final.png`, the file could be opened as a png and the first part of the flag could be read.

Doing it through the command line Optical Character Recognition (ocr) tools could be used. To download a well-known one, `sudo apt install gocr`, then `gocr flag2of2-final.png | tr -d "\n"` to remove the new lines and paste the contents. This gives `piconF{f1u3n7_` which is mostly right other than it regonizing an n instead of CT. Overall it should be `picoCTF{f1u3n7_`.

Flag: `picoCTF{f1u3n7_1n_pn9_&_pdf_7f9...}`
