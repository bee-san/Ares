# Description

Can you abuse the banner? <br>
The server has been leaking some crucial information <br>
on tethys.picoctf.net 57125. Use the leaked <br>
information to get to the server. <br>
To connect to the running application use nc <br>
tethys.picoctf.net 50695. From the above information <br>
abuse the machine and find the flag in the /root <br>
directory.

# Solution

Here is a better formatted version of this writeup on [picoCTF Solutions website](https://picoctfsolutions.com/picoctf-2024-dont-you-love-banners).

The password could be seen by connecting to the server that has been leaking information with this command: `nc tethys.picoctf.net 57125`

Output: `SSH-2.0-OpenSSH_7.6p1 My_Passw@rd_@1234`

Now that the password has been received the server can be entered with this command: `nc tethys.picoctf.net 50695`

There is a welcome banner and then it asks for the password which is `My_Passw@rd_@1234`. It's followed by two questions:

What is the top cyber security conference in the world? [defcon](https://defcon.org/) <br>
the first hacker ever was known for phreaking(making free phone calls), who was it? [john draper](https://search-guard.com/john-draper-captain-crunch/)

After that, it gives access to the shell at `/home/player`. It gives permission to see the banner text, and the text file which says "keep digging". In the challenge description it says the flag is in `/root` so the command `cd /root` can be used to go to that directory. In that directory, there is the flag.txt file which can not be read with current permissions. There is also a script.py which shows the initial banner and questions used to get into the shell.

In the script, it shows that it grabs the banner from `/home/player/banner`, and if it doesn't exist it says `Please supply banner in /home/player/banner`. Because this Python script is located in the root directory it is run with root privileges, but pulls the banner from `/home/player`.

By going back with `cd /home/player` the banner could be modified and re-enter with the netcat command to see the changed banner. By creating a symbolic link to the flag in the root directory and calling it banner it will read the flag.txt file with the root permissions and display it as the banner. Now when re-entering with, `nc tethys.picoctf.net 50695`, the flag will be displayed.

To remove the banner file `rm banner` can be used. Then `ln -s /root/flag.txt banner` to create the symbolic link and call the file banner. Lastly, leave the terminal and connect again with `nc tethys.picoctf.net 50695`.

Flag: `picoCTF{b4nn3r_gr4bb1n9_su((3sfu11y_8126...}`
