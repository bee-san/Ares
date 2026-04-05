# BTLO PowershellAnalysis-Keylogger
Write up of the BTLO Powershell Analysis Keylogger Challenge

Link to Investigation: https://blueteamlabs.online/home/challenge/powershell-analysis-keylogger-9f4ab9a11c

## Scenario
A suspicious PowerShell script was found on one of our endpoints. Can you work out what it does?

## Question 1
What is the SHA256 hash value for the PowerShell script file?
You can run a Sha256 Check on the Inner file to determine the SHA256 Value:
<img width="538" height="43" alt="image" src="https://github.com/user-attachments/assets/0d468994-2fc1-404d-a307-75bba5c065c9" />

## Question 2 
What email address is used to send and receive emails?
The Powershell Script is easy to analyze once it is open in txt: 
<img width="1078" height="260" alt="image" src="https://github.com/user-attachments/assets/5c52cc0a-8744-41fc-aa6f-d4bdf2543649" />

## Question 3 
What is the password for this email account?
This information can be found in the code block above:
<img width="1078" height="260" alt="image" src="https://github.com/user-attachments/assets/f217f02b-1c7f-458c-b19f-f71918400271" />

## Question 4
What port is used for SMTP?
Same thing again:
<img width="1078" height="260" alt="image" src="https://github.com/user-attachments/assets/533da719-4f1b-4c44-a19c-485faffeb80a" />

## Question 5
What DLL is imported to help record keystrokes?
You can find the Information below:
<img width="569" height="132" alt="image" src="https://github.com/user-attachments/assets/997199c6-11e9-47dc-b179-62340b8faf91" />

## Question 6
What directory is the generated txt file put in?
Same thing as the previous question:
<img width="569" height="132" alt="image" src="https://github.com/user-attachments/assets/9e99faf1-0f95-4e14-8f22-2b70f3e39745" />



# Conclusion
Overall, this was an easy challenge; Realistically, an attacker could encode their Powershell Script and Obfuscate the commands, this would just serve to make it more tedious to deobfuscate and analyze.
