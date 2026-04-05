# Description

I have a friend that enjoys coding and he hasn't <br>
stopped talking about a snake recently <br>
He left this file on my computer and dares me to <br>
uncover a secret phrase from it. Can you assist?

# Solution

Here is a better formatted version of this writeup on [picoCTF Solutions website](https://picoctfsolutions.com/picoctf-2024-weirdsnake).

To get the Python bytecode file: `wget https://artifacts.picoctf.net/c_titan/31/snake`

The first section in the byte code contains many `LOAD_CONST` calls which creates the `input_list` as seen in the bytecode. Sections 2 through 6 cover the `key_str` variable which can be a bit tricky.

```
  2          84 LOAD_CONST              31 ('J')
             86 STORE_NAME               1 (key_str)
```
This part uses `LOAD_CONST` for 'J' in key_str so it is now just 'J'.

```
  3          88 LOAD_CONST              32 ('_')
             90 LOAD_NAME                1 (key_str)
             92 BINARY_ADD
             94 STORE_NAME               1 (key_str)
```

Next, it loads `'_'`, then loads the name key_str (not storing) than binary add so `'_' + key_str`, and with `STORE_NAME` it loads it into key_str. So now key_str is "_J".

```
  4          96 LOAD_NAME                1 (key_str)
             98 LOAD_CONST              33 ('o')
            100 BINARY_ADD
            102 STORE_NAME               1 (key_str)
```

Similar to the last one but `LOAD_NAME` and `LOAD_CONST` are flipped which means `key_str + 'o'` which means it's just appending it to key_string. So now key_str is "_Jo". Section 5 is the same format as section 4 but with the character '3' making key_string "_Jo3". Section 6 is the same format as section 3 so the final configuration of key_str would be "t_Jo3".

Section 9 and the corresponding disassembly near the bottom corresponds to this line of code: `key_list = [ord(char) for char in key_str]`. Section 11 makes the comparison with key_list and input_list and section 12 does the extend on key_list. Section 15 does the zip on input_list and key_list and section 18 has the join to the result_text variable. The result is the value of the flag. 

Script:
```
input_list = [4, 54, 41, 0, 112, 32, 25, 49, 33, 3, 0, 0, 57, 32, 108, 23, 48, 4, 9, 70, 7, 110, 36, 8, 108, 7, 49, 10, 4, 86, 43, 108, 122, 14, 2, 71, 62, 115, 88, 78]

key_str = 't_Jo3'

key_list = [ord(char) for char in key_str]

while len(key_list) < len(input_list): 
    key_list.extend(key_list)

result = [a ^ b for a, b in zip(input_list, key_list)]
result_text = ''.join(map(chr, result))

print(result_text)
```

Running this script in Python gives the flag.

Flag: `picoCTF{N0t_sO_coNfus1ng_sn@ke_30a...}`
