# CyberDefenders WebStrike Lab
Write up of the CyberDefenders Webstrike Lab

Link to Investigation: https://cyberdefenders.org/blueteam-ctf-challenges/webstrike/

## Scenario
A suspicious file was identified on a company web server, raising alarms within the intranet. The Development team flagged the anomaly, suspecting potential malicious activity. To address the issue, the network team captured critical network traffic and prepared a PCAP file for review.
Your task is to analyze the provided PCAP file to uncover how the file appeared and determine the extent of any unauthorized activity.

## Question 1
Identifying the geographical origin of the attack facilitates the implementation of geo-blocking measures and the analysis of threat intelligence. From which city did the attack originate?

* You can view the IP Connections under __Statistics__ > __Endpoints__ > __IPv4__
* You'll have to do a OSINT Search on AbuseIPDB or VT to determine country of origin:
* <img width="1431" height="137" alt="image" src="https://github.com/user-attachments/assets/42f05513-9372-4f8e-b82f-9fbf42c25ac8" />

## Question 2 
Knowing the attacker's User-Agent assists in creating robust filtering rules. What's the attacker's Full User-Agent?

* You can view the User-Agent by looking deeper into a HTTP Request on Wireshark. Here we can see a connection to the affected endpoint from the attacker, and within the protocol details, it reveals the User-Agent:
* <img width="817" height="63" alt="image" src="https://github.com/user-attachments/assets/8ca19368-31bc-4c0d-bed4-4b8a9960b9bf" />

## Question 3 
We need to determine if any vulnerabilities were exploited. What is the name of the malicious web shell that was successfully uploaded?
* You can view HTTP POST requests by filtering for it in WireShark: http.request.method == POST; There can you can follow the HTTP Stream to determine the webshell that was uploaded:
* <img width="831" height="165" alt="image" src="https://github.com/user-attachments/assets/51206207-11a5-4343-ba83-41aa3e5d264d" />

## Question 4
Identifying the directory where uploaded files are stored is crucial for locating the vulnerable page and removing any malicious files. Which directory is used by the website to store the uploaded files?
* You can view the attacker retrieving the malicious file by filtering by HTTP Request URI in Wireshark; There can you see the directory it is uploaded in:
* <img width="1010" height="59" alt="image" src="https://github.com/user-attachments/assets/7f5df330-967d-4c67-ba44-7e1c1aca385d" />

## Question 5
Which port, opened on the attacker's machine, was targeted by the malicious web shell for establishing unauthorized outbound communication?
* Following the HTTP Stream from before, you can view the commands the attacker attempted and it displays a connection to be opened on a specific port:
* <img width="798" height="50" alt="image" src="https://github.com/user-attachments/assets/5557ccca-ddab-4d77-8a01-c0419fc88064" />

## Question 6
Recognizing the significance of compromised data helps prioritize incident response actions. Which file was the attacker attempting to exfiltrate?
* If you view POST connections again, this time from the affected machine to the attacker, you can see a form item was attempting to exfil.
* <img width="481" height="34" alt="image" src="https://github.com/user-attachments/assets/f3d8c125-314d-4def-ac51-4e1d6f144bb8" />


# Conclusion
Overall, this is an easy lab and a good introduction to Wireshark. Hopefully this helps even though as of writing this (3/24/2026) this lab is retired
