# BTLO Bruteforce
Write up of the BTLO Bruteforce Challenge

Link to Investigation: https://blueteamlabs.online/home/challenge/bruteforce-16629bf9a2

## Scenario
Can you analyze logs from an attempted RDP bruteforce attack?

One of our system administrators identified a large number of Audit Failure events in the Windows Security Event log.

There are a number of different ways to approach the analysis of these logs! Consider the suggested tools, but there are many others out there!

## Question 1
How many Audit Failure events are there?

* You can run a count on "audit failures' or event id 4625:
* <img width="659" height="324" alt="image" src="https://github.com/user-attachments/assets/d52c4089-d3a6-4758-875f-5fa5adfdded7" />

## Question 2 
What is the username of the local account that is being targeted?

* You can search for the account name, or scrolling through the log files you can see the account name clearly:
* <img width="666" height="140" alt="image" src="https://github.com/user-attachments/assets/e34f4fe5-c073-49dd-bc88-4075426174ff" />


## Question 3 
What is the failure reason related to the Audit Failure logs?

* The reason can be located within the event itself:
* <img width="561" height="81" alt="image" src="https://github.com/user-attachments/assets/e8c8908d-8595-4f07-b553-df1c93c5b8a6" />


## Question 4
What is the Windows Event ID associated with these logon failures?

* As Cyber Security Practioners I feel like this one should be a gimme, lol.
* <img width="1089" height="41" alt="image" src="https://github.com/user-attachments/assets/6e116e6b-bb3e-4159-8346-33e7194d959a" />


## Question 5
What is the source IP conducting this attack?

* You can view this information within the event itself:
* <img width="465" height="87" alt="image" src="https://github.com/user-attachments/assets/d454d0ff-20b9-459d-8bd5-29be2f1c417b" />

## Question 6
What country is this IP address associated with?

* If you put the Source IP information into any OSINT IP database, the country of origin will be available to view

## Question 7
What is the range of source ports that were used by the attacker to make these login requests?

* This one will require you to recall your sorting knowledge from Text Editors; For me I used N++ which has a line operation to sort lexicographically ascending.
* Searching by __Source Port:__ will show the lowest port number and highest port number:
* <img width="1537" height="343" alt="image" src="https://github.com/user-attachments/assets/35b8f3e7-157b-4e02-b992-74be646842d3" />



# Conclusion
Good Practice with reviewing Windows Event Logs and Text Editor functions for easier searches. 
