PUNC = set("!\"#$%&'()*+,-./:;<=>?@[\]^_`{|}~")
MIN_LENGTH = 2

file_name = input("Enter the name of the file you want to modify: ")
f = open(file_name)
f2 = open("modfiied_file.txt", "w")
for line in f:
    if len(line) <= MIN_LENGTH:
        continue
    if len(set(line).intersection(PUNC)) != 0:
        continue
    f2.write(line)

f.close()
f2.close()
