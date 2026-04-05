# HackThisSite – Basic Mission 2 Writeup

---

## Challenge Text

A slightly more difficult challenge involving an incomplete password script.

Network Security Sam created a password protection script that loads the real password from an unencrypted text file and compares it with the user-entered password. However, he neglected to upload the password file.

---

## Step 1 – Analyze the Challenge Description

The challenge states that:

- The password is stored in a text file
- The script loads the password from that file
- The script compares the file password with user input
- However, the password file was not uploaded

This indicates that the script cannot retrieve the actual password.

---

## Step 2 – Understand the Vulnerability

Normally, authentication works like this:

stored_password == entered_password


But in this case:


stored_password = null (file missing)
entered_password = empty (user enters nothing)

So the comparison becomes:

null == null → true


This allows authentication bypass.

---

## Step 3 – Exploit the Vulnerability

Steps performed:

1. Leave the password field empty
2. Click the submit button

The system grants access successfully.

<img width="751" height="118" alt="image" src="https://github.com/user-attachments/assets/92a0f70f-ce16-44b5-9c36-aa141d0fcb60" />



This confirms that the authentication logic is flawed.

---



## Explanation

The authentication mechanism fails because the password file was not uploaded. By submitting an empty password, authentication is bypassed successfully.





