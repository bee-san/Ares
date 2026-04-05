# Vhuynh-BTLO-Piggy-Writeup
Writeup of the BTLO Piggy Investigation

Link to Investigation; https://blueteamlabs.online/home/investigation/piggy-aij2bd8h2

## Scenario
Investigate some simple network activity in Wireshark! You can launch Wireshark in a terminal with the command 'wireshark'. The questions are mapped to the four PCAPs on the Desktop.\

## Question 1

1. PCAP One) What remote IP address was used to transfer data over SSH? (Format: X.X.X.X)

   * You can sort by Protocol within PCAP one; Sorting by SSH will provide you the offending IP:
   * <img width="1447" height="189" alt="image" src="https://github.com/user-attachments/assets/075f6551-df92-4ae4-a03f-d88a7683fe77" />
## Question 2
2. PCAP One) How much data was transferred in total? (Format: XXXX M)

   * You can find this out by viewing the statistics of the IPv4 Connection:
   * <img width="1263" height="476" alt="image" src="https://github.com/user-attachments/assets/19cd731d-0629-45a9-8c31-0dda8d868d21" />
## Question 3
3. PCAP Two) Review the IPs the infected system has communicated with. Perform OSINT searches to identify the malware family tied to this infrastructure (Format: MalwareName)

   * You can sort by Destination to numerically sort the DST IPs and discover the Two IPs the Internal IP was communicating with. After this you can do a Virustotal Search for the IPs and discover the Malware Family this IP was Associated with:
   * <img width="217" height="463" alt="image" src="https://github.com/user-attachments/assets/c92fa36b-df84-47d5-8a5b-9dc840ee69ae" />
## Question 4
4. PCAP Three) Review the two IPs that are communicating on an unusual port. What are the two ASN numbers these IPs belong to?

   * This one will require to recall your port number knowledge, start by filtering port numbers with familiar traffic (like 443) and high ephemeral ports, which can be used as private ports for services. Once you've done this, you can see there are communications with two port numbers that stand out:
   * <img width="216" height="317" alt="image" src="https://github.com/user-attachments/assets/dd8f9ab7-2630-46d5-8f33-e8ee3d181b81" />
   * After this, you can discover the IP's communicating on this port and a OSINT ASN Lookup tool can help you figure out the rest
## Question 5
5. PCAP Three) Perform OSINT checks. What malware category have these IPs been attributed to historically? (Format: MalwareType)

   * Again, this one is as simple as plugging the IPs into VT and checking their relationships or historical data:
   * <img width="976" height="157" alt="image" src="https://github.com/user-attachments/assets/43ba3509-1f58-4987-8834-3b25734add5d" />
   * Be sure to check all the IPs with communications.
## Question 6 
6. PCAP Three) What ATT&CK technique is most closely related to this activity? (Format: TXXXX)

   * This one ties into the Mitre ATT&CK Framework. Do a google search on the Malware Family to uncover which Techinique this applies too:
## Question 7  
7. PCAP Four) Go to View > Time Display Format > Seconds Since Beginning of Capture. How long into the capture was the first TXT record query made? (Use the default time, which is seconds since the packet capture started) (Format: X.xxxxxx)

   * Follow the instructions, once you have done that, you can do a simple string search for the text "txt"; This will show you that the query was made over DNS, and then You can sort by Protocol to reveal the first interaction containing a txt record request:
   * <img width="2030" height="406" alt="image" src="https://github.com/user-attachments/assets/fec4a298-e0df-4c9d-b471-ccf089b3b795" />
## Question 8
8. PCAP Four) Go to View > Time Display Format > UTC Date and Time of Day. What is the date and timestamp? (Format: YYYY-MM-DD HH:MM:SS)

   * This one will simply require you to follow the instructions, and then copy and paste the Time & Date format upto seconds
## Question 9  
9. PCAP Four) What is the ATT&CK subtechnique relating to this activity? (Format: TXXXX.xxx)

    * You can google search __DNS TXT ATT&CK__ for the sub-technique this attacker is using
  

# Conclusion
Overall this is a pretty easy investigation. It will not require you to do extensive wireshark filtering, but will require you to use your OSINT skills to uncover some of the answers

I hope this helps, to whoever can find it :)


     


