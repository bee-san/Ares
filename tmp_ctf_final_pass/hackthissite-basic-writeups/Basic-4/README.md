# HackThisSite – Basic Mission 4 Writeup

---

## Challenge Description

An email script has been set up to send the password to the administrator.  

Sam hardcoded the password into the script. Since the password is long and complex, he created a feature that emails the password to himself in case he forgets it.

---

## Step 1 – Inspect the HTML Source

Right-click the page and select **Inspect**

Inside the HTML, we find the following form:

<img width="832" height="102" alt="image" src="https://github.com/user-attachments/assets/c8ad8711-5914-4b66-bdeb-fe6b1f3f8a9a" />

This field contains the administrator’s email address.Although it is hidden, it can still be modified using browser developer tools.

This vulnerability is known as: Form Field Tampering

Hidden fields are not secure because users can edit them before submitting the form.

---

## Step 2 – Modify the Email Address

In the Elements tab, locate:
value="sam@hackthissite.org"

Replace it with your own account email address

---


## Step 4 – Submit the Form

After editing the email:

Click the Submit button.

You will see a confirmation message:

Password reminder successfully sent to your email address.

---

## Step 5 – Retrieve the Password

Open your email inbox.

Find the email addressed to "Sam".

Copy the password from the message and enter the password into the password field.

Click submit to complete the level.

---

## Explanation

This level demonstrates why client-side controls are not secure.

Even though the email field was hidden, it could still be modified using browser developer tools.

The server trusted the value sent from the client without proper validation.

This is an example of insecure design and improper input validation.




