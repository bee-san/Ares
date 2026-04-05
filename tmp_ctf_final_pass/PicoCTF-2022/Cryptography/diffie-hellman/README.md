# diffie-hellman

## Challenge

Alice and Bob wanted to exchange information secretly. The two of them agreed to use the Diffie-Hellman key exchange algorithm, using p = 13 and g = 5. They both chose numbers secretly where Alice chose 7 and Bob chose 3. Then, Alice sent Bob some encoded text (with both letters and digits) using the generated key as the shift amount for a Caesar cipher over the alphabet and the decimal digits. Can you figure out the contents of the message? Download the message [here](https://artifacts.picoctf.net/c/449/message.txt). Wrap your decrypted message in the picoCTF flag format like: `picoCTF{decrypted_message}`

## Solution

We don't need to know anything about the Diffie-Hellman key exchange algorithm for this challenge. Just use [cryptii.com](https://cryptii.com/) (or any Caesar cipher application). Paste in the message `H98A9W_H6UM8W_6A_9_D6C_5ZCI9C8I_D9FF6IFD`, then set the alphabet to `abcdefghijklmnopqrstuvwxyz0123456789`, then increment the shift until the message is clear. The offset I found was 5 (`a --> f`)

### Flag

`picoCTF{C4354R_C1PH3R_15_4_817_0U7D473D_84AA1DA8}`
