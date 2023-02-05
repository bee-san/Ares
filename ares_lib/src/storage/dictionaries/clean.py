# Run this to clean up any dictinaries

# We remove puncuation from the text in the english checker
# therefore we should not have any in our dictionary
PUNC = set("!\"#$%&'()*+,-./:;<=>?@[\]^_`{|}~")
# Anything below length of 2 is rather spammy so we remove it
MIN_LENGTH = 2
# Since the English checker normaliser turns words into lowercase, we only want lowercase words in our dict
LOWERCASE = True

file_name = input("Enter the name of the file you want to modify: ")
f = open(file_name)
f2 = open("modified.txt", "w")
for line in f:
    if len(line) <= MIN_LENGTH:
        continue
    if len(set(line).intersection(PUNC)) != 0:
        continue
    if LOWERCASE:
        line = line.lower()
    f2.write(line)

f.close()
f2.close()
