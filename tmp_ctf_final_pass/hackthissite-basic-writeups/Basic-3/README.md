# HackThisSite – Basic Mission 3 Writeup

---

## Challenge Description

This time Network Security Sam remembered to upload the password file, but there were deeper problems than that.

The page provides a password input field for authentication.

---

## Step 1 – Inspect the Page Source

Open Developer Tools:

Right click → Inspect 

Go to the **Elements tab**

While examining the HTML form, the following hidden input field is visible:


```html
<input type="hidden" name="file" value="password.php">
```

<img width="1291" height="124" alt="image" src="https://github.com/user-attachments/assets/5386adee-3a91-4a7e-b896-1ff805dcf235" />

This reveals that the password is stored in:

password.php



---

## Step 2 – Access the Password File Directly

Since the file name is exposed, try accessing it directly in the browser.

Original page:

https://www.hackthissite.org/missions/basic/3/

Modify the URL:

https://www.hackthissite.org/missions/basic/3/password.php


---

## Step 3 – Retrieve the Password

The password file displays the password in plain text.

<img width="715" height="187" alt="image" src="https://github.com/user-attachments/assets/b0f311a5-22fb-4951-a616-c477ed6c6512" />


Enter the password and click submit.

Access is granted.

---
