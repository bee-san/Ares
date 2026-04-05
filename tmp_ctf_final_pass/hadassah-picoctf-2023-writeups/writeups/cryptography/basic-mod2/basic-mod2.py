message = [268, 413, 438, 313, 426, 337, 272, 188, 392, 338, 77, 332, 139, 113, 92, 239, 247, 120, 419, 72, 295, 190, 131]
alphabet = "abcdefghijklmnopqrstuvwxyz"
digits = "0123456789"
charset = alphabet + digits + "_"
decrypted = ""

for num in message:
    num_mod = num % 41
    num_inv = pow(num_mod, -1, 41) # calculates modular inverse using Python's built-in pow() function
    decrypted += charset[num_inv-1] # maps the resulting number to the character set

flag = "picoCTF{" + decrypted + "}"
print(flag)

