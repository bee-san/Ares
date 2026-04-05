# Description

The judge for these pictures is a real fan of antiques. <br>
Can you age this photo to the specifications? <br>
Set the timestamps on this picture to 1970:01:01 <br>
00:00:00.001+00:00 with as much precision as possible <br>
for each timestamp. In this example, +00:00 is a <br>
timezone adjustment. Any timezone is acceptable as <br>
long as the time is equivalent. As an example, this <br>
timestamp is acceptable as well: 1969:12:31 <br>
19:00:00.001-05:00. For timestamps without a timezone <br>
adjustment, put them in GMT time (+00:00). The <br>
checker program provides the timestamp needed for each. <br>
Use this picture. <br>
Submit your modified picture here: <br>
`nc -w 2 mimas.picoctf.net 57925 < original_modified.jpg` <br>
Check your modified picture here: <br>
`nc -d mimas.picoctf.net 50499`

# Solution

Here is a better formatted version of this writeup on [picoCTF Solutions website](https://picoctfsolutions.com/picoctf-2024-blast-from-the-past).

To get the file: `wget https://artifacts.picoctf.net/c_mimas/91/original.jpg` then `mv original.jpg original_modified.jpg`.

`exiftool -AllDates='1970:01:01 00:00:00.001' original_modified.jpg`

To check if you it was done correctly write these commands:

`nc -w 2 mimas.picoctf.net 57925 < original_modified.jpg` - Submitting the picture <br>
`nc -d mimas.picoctf.net 50499` - Checking the picture

From here it could be seen that it checks for SubSecCreateDate tag then SubSecDateTimeOriginal tag, then SubSecModifyDate tag, and lastly the Samsung:TimeStamp tag.

`exiftool -SubSecCreateDate='1970:01:01 00:00:00.001' -SubSecDateTimeOriginal='1970:01:01 00:00:00.001' -SubSecModifyDate='1970:01:01 00:00:00.001' original_modified.jpg`

On exiftool website (https://exiftool.org/TagNames/Samsung.html) it could be seen that this tag `Samsung:TimeStamp tag` is not writable. Although not writable exiftool can still read the value with this command: `exiftool -Samsung:TimeStamp original_modified.jpg`

When looking at the file with hex editor bless: `sudo apt install bless`, then ran with `bless` to open the GUI.From there you can go to file/open or open to open the file.

At the bottom of the file, this can be seen:

`Image_UTC_Data1700513181420`

The last part with the numbers (1700513181420) is an epoch time stamp which could be read at the [EpochConverter website](https://www.epochconverter.com/). When converted it shows the exact same time as the Samsung:TimeStamp value.

Epoch time starts at 1970 so to get to that it would be 0, however to add one second afterwards and to match the amount of digits needs for an epoch time stamp this value would be used 0000000000001. By changing the epoch time stamp in bless to 0000000000001 to now be `Image_UTC_Data0000000000001` and saving the image will change the Samsung:TimeStamp value.

It could be checked with this command that it was indeed changed: `exiftool -Samsung:TimeStamp original_modified.jpg`

To get the flag write these commands again:

`nc -w 2 mimas.picoctf.net 57925 < original_modified.jpg`, then `nc -d mimas.picoctf.net 50499`

Flag: `picoCTF{71m3_7r4v311ng_p1c7ur3_12e0...}`
