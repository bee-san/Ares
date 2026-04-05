# HackThisSite – Basic Mission 1 Writeup

## Level 1 – The Idiot Test

### Challenge Description

This level is called **"The Idiot Test"**. The page shows a password input box and suggests learning HTML if you don’t know what to do.

This hint indicates that the password may be hidden in the page’s HTML source code.

---


We see:

- Password input field
- Submit button
- Hint suggesting to learn HTML

This strongly suggests inspecting the HTML source.

---

## Step 2 – Inspect the Page Source

Right click anywhere on the page and select:

Inspect


Then go to the **Elements tab**.


---

## Step 3 – Find the Hidden Password

While examining the HTML code, we find this comment:

<img width="556" height="110" alt="image" src="https://github.com/user-attachments/assets/46c3cdd2-1312-4681-a40a-4988384e5409" />


```html
<!-- the first few levels are extremely easy: password is 49cbe777 -->
```

## Step 4 – Enter the Password

Return to the challenge page and enter the password.

---

## Explanation

This challenge demonstrates a very basic but important concept in web security.

Web pages are built using HTML. Sometimes developers leave comments in the HTML source code for reference or debugging purposes.

These comments are not visible on the webpage itself, but they can be viewed using the browser's **Inspect Element** or **View Page Source** feature.

In this case, the password was hidden inside an HTML comment.




