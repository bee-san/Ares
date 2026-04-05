from string import ascii_uppercase
x = [268, 413, 110, 190, 426, 419, 108, 229, 310, 379, 323,
     373, 385, 236, 92, 96, 169, 321, 284, 185, 154, 137, 186]


a = "0"+ascii_uppercase + "0123456789_"

for i in x:
    print(a[pow(i, -1, 41)], end="")
