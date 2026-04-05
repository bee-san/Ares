from string import ascii_uppercase
x = [54, 396, 131, 198, 225, 258, 87, 258, 128, 211, 57,
     235, 114, 258, 144, 220, 39, 175, 330, 338, 297, 288]


a = ascii_uppercase + "0123456789_"

for i in x:
    print(a[i % 37], end="")
