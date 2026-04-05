# Writeup BTLO Investigations - Cerulean (Digital Forensics)

<img width="1920" height="1080" alt="image" src="https://github.com/user-attachments/assets/96a2c06b-b4fe-4fec-8b88-2d2c9772fead" />

**Scenario:** You’re the lead security analyst at Cerulean Inc., a respected manufacturer of industrial control systems. Your SIEM alerts you to suspicious RDP connections to the Production Department—especially Jane’s PC (the head of the department). It was determined later on that the attacks were malicious. You are tasked to analyze the triage artifacts on her computer and investigate the RDP connections for possible exfiltration.

Reference Article: https://www.securityblue.team/blog/posts/investigating-insider-threats-rdp-activity-magnet-forensics

**Tools:** Magnet AXIOM Examiner, Event Log Explorer, EZViewer, Hindsight, MFTECmd, RegistryExplorer, Slack-Parser, TimelineExplorer, T1021.001

**Difficulty:** Easy
**OS:** Windows

## Q1: When did Jane receive the malicious mail from an attacker pretending to be from IT Support? Check the web history to help us better timeline the series of events. (Format: YYYY-MM-DD HH:MM:SS UTC)


***=> 2024-11-05 20:45:03 UTC***

## Q2: The threat actor immediately, after RDP’ing, tries to log into other storage-based resources. What is the one with the most traffic? (Format: Storage Name)


***=> Google Drive***

## Q3: It looks like the threat actor’s motive is data exfiltration via RDP. What ITM ID corresponds with this technique? (Format: XXXXX.XXX) 


***=> IF001.001***

## Q4: Let’s step back for a bit. Jane’s account mistakenly has admin rights. What role did they assign to her? (Format: Job Role (Title))


***=> MSFT Admin (Database Specialist)***

## Q5: It seems like she downloaded Slack before the RDP session. Our main point of communication is Teams, so this is strange. What is the installation date and time of this software? (Format: YYYY-MM-DD HH:MM:SS UTC)


***=> 2024-11-05 20:08:55 UTC***

## Q6: There is enough evidence of Slack being used on Jane’s machine. Can you provide the unofficial URL being utilized for communication? (Format: hxxps://url.tld) 


***=> https://ceruleaninc.slack.com/***

## Q7: Provide the initial time and Origin IP Address for the RDP connections to Jane’s workstation. (Format: MM/D/YYYY H:MM:SSS XX UTC, XXX[.]XXX[.]XXX[.]XXX)


***=> 11/5/2024 8:58:38 PM UTC, 104[.]203[.]174[.]169***

## Q8: Our Project Venus plans were leaked, triggering the defenses. What are the four documents in alphabetical order? (Hint: examine the Windows Defender Logs) (Tip: remove 'project venus' and 'cerulean' from the document names). (Format: Doc1, Doc2, Doc3, Doc4)


***=> Energy Storage, Research, Solar Panel Tech, Wind Turbine Design***
