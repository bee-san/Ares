1. Extract hashes from bitlocker file using bitlocker2john. See https://github.com/GTekSD/BitLocker-CTF/blob/main/answer/you-looser/u-need-to/learn/a-lot/kiddo/go/see-this/answer.md

```bash
bitlocker2john bitlocker-1.dd
```

2. Use the cracked password "jacqueline" to decrypt the drive and mount it with dislocker. Used following guide from Perplexity with the GPT-4o model.

Prompt: How to decrypt a bitlocker encrypter ".dd" file with "dislocker". I have a password.

To decrypt a BitLocker-encrypted `.dd` file using `dislocker` with a password, follow these steps:

---

### 1. Install Dislocker
Ensure `dislocker` is installed on your system. For Debian-based distributions, you can install it using:
```bash
sudo apt install dislocker
```

---

### 2. Prepare Directories
Create two directories: one for the decrypted file and another for mounting the decrypted NTFS partition:
```bash
sudo mkdir /mnt/dislocker /mnt/decrypt
```

---

### 3. Decrypt the `.dd` File Using Your Password
Run `dislocker` with the `-u` option to provide your password. Replace `/path/to/encrypted.dd` with the path to your `.dd` file and `YourPasswordHere` with your actual BitLocker password:
```bash
sudo dislocker -V /path/to/encrypted.dd -uYourPasswordHere -- /mnt/dislocker
```
This command will create a virtual NTFS file named `dislocker-file` in `/mnt/dislocker`.

---

### 4. Mount the Decrypted File
Mount the `dislocker-file` as a virtual NTFS partition to access its contents:
```bash
sudo mount -o loop /mnt/dislocker/dislocker-file /mnt/decrypt
```

You can now access the decrypted files in `/mnt/decrypt`.

---

### 5. Unmount When Done
After accessing the data, unmount both directories:
```bash
sudo umount /mnt/decrypt
sudo umount /mnt/dislocker
```

---

### **Notes:**
1. If you omit the password in the command (e.g., `-u` without specifying it), you will be prompted to enter it interactively.
2. Ensure that `/mnt/dislocker` and `/mnt/decrypt` are empty before running these commands.
3. If you encounter issues, verify that the `.dd` file is a valid BitLocker-encrypted volume.

By following these steps, you can successfully decrypt and access a BitLocker-encrypted `.dd` file using `dislocker`.