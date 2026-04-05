import requests

URL_BASE = "http://verbal-sleep.picoctf.net:53626"

with open("backup_files_paths.txt", "r") as f:
    links = f.readlines()

final_tested = "App_Data/Install/SQL/View_OM_AccountContact_ContactJoined.sql"

links = [link.strip() for link in links]

for link in links:
    print(link, requests.get(f"{URL_BASE}/{link}"))