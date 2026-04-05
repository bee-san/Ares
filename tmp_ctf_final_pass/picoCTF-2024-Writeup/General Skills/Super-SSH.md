# Description

Using a Secure Shell (SSH) is going to be pretty <br>
important.<br>
Can you ssh as `ctf-player` to `titan.picoctf.net` at port<br>
`65080` to get the flag?<br>
You'll also need the password `6dd28e9b`. If asked, accept <br>
the fingerprint with yes.<br>
If your device doesn't have a shell, you can use: <br>
https://webshell.picoctf.org<br>
If you're not sure what a shell is, check out our Primer: <br>
https://primer.picoctf.com/#_the_shell

# Solution

Here is a better formatted version of this writeup on [picoCTF Solutions website](https://picoctfsolutions.com/picoctf-2024-super-ssh).

To get the flag ssh into the shell with this command:

`ssh ctf-player@titan.picoctf.net -p 65080`

Note this format: `ssh user@host -p port`

Once you ssh in put "yes" for the fingerprint and input "6dd28e9b" as the password.

Flag: `picoCTF{s3cur3_c0nn3ct10n_5d...}`
