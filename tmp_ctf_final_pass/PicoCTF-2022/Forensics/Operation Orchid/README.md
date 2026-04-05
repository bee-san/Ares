# Operation Orchid

## Challenge

Download this disk image and find the flag. Note: if you are using the webshell, download and extract the disk image into `/tmp` not your home directory.

* [Download compressed disk image](https://artifacts.picoctf.net/c/236/disk.flag.img.gz) ([Archive](https://web.archive.org/web/20220321025118/https://artifacts.picoctf.net/c/236/disk.flag.img.gz))

## Solution

1. We can decompress the disk image with `gunzip disk.flag.img.gz` and then mount it with `sudo kpartx -av disk.flag.img`.
2. In the mounted volume, there is a file `/root/flag.txt.enc` and `.ash_history`. Looking at `.ash_history` we see the following:

```
touch flag.txt
nano flag.txt 
apk get nano
apk --help
apk add nano
nano flag.txt 
openssl
openssl aes256 -salt -in flag.txt -out flag.txt.enc -k unbreakablepassword1234567
shred -u flag.txt
ls -al
halt
```

3. So, it looks like `flag.txt.enc` was encrypted and salted using aes256 with key `unbreakablepassword1234567`.

4. We can decrypt the `flag.txt.enc` and print the flag with `openssl aes256 -d -salt -in flag.txt.enc -out flag.txt -k unbreakablepassword1234567; cat flag.txt` (notice the additional `-d` option).

### Flag

`picoCTF{h4un71ng_p457_0a710765}`
