# Operation Oni

## Challenge

Download this disk image, find the key and log into the remote machine. Note: if you are using the webshell, download and extract the disk image into /tmp not your home directory.

* [Download disk image](https://artifacts.picoctf.net/c/373/disk.img.gz) ([Archive](https://web.archive.org/web/20220321005900/https://artifacts.picoctf.net/c/373/disk.img.gz))
* Remote machine: `ssh -i key_file -p 65529 ctf-player@saturn.picoctf.net`

## Solution

1. We can decompress the disk image with `gunzip disk.img.gz` and then mount it with `sudo kpartx -av disk.img`.
2. In the mounted volume, the key is in `/root/.ssh/id_ed25519`. We can save this to a file called `key_file`.
3. Finally, run the given `ssh` command. The flag is stored in `flag.txt` in the current directory.
