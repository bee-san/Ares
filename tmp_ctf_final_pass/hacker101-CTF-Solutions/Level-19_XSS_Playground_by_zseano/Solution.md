# XSS Playground by zseano

**Difficulty:** Moderate  
**Category:** Web  
**Flags:** 1/1  
**Author:** zseano

---

## 🧠 Overview

Despite the misleading name "XSS Playground," this challenge is actually about API endpoint exploitation rather than Cross-Site Scripting. The challenge involves discovering a hidden API endpoint and using proper authentication headers to retrieve the flag.

---

## 🔍 Initial Investigation

After researching the website, I quickly realized this challenge was not about XSS at all - the name appears to be intentionally misleading. The real objective is to exploit an API endpoint through proper reconnaissance and authentication bypass.

---

## 🕵️ Step 1: Discovering the API Endpoint

I began by exploring the website's JavaScript files to understand the application's structure. By navigating to:

```
https://fe5eef9c59de7a6245bc72e75e6ffb59.ctf.hacker101.com/custom.js
```

I found references to an API endpoint that caught my attention:

```javascript
api/action.php?act=getemail
```

This endpoint appeared to be the key to solving the challenge.

---

## 🔑 Step 2: Authentication Header Discovery

Further analysis of the `custom.js` file revealed an important authentication mechanism:

```
X-SAFEPROTECTION: enNlYW5vb2Zjb3Vyc2U=
```

This Base64-encoded value appeared to be required for accessing the protected API endpoint.

> **Note:** The Base64 string `enNlYW5vb2Zjb3Vyc2U=` decodes to `zseanoofcourse`, which is a reference to the challenge author.

---

## 🚀 Step 3: Exploiting the API Endpoint

With the endpoint and authentication header identified, I crafted a curl request to access the protected API:

```bash
curl -v -H "X-SAFEPROTECTION: enNlYW5vb2Zjb3Vyc2U=" --http1.1 "https://fe5eef9c59de7a6245bc72e75e6ffb59.ctf.hacker101.com/api/action.php?act=getemail"
```

### Request Breakdown:
- `-v`: Verbose output for debugging
- `-H "X-SAFEPROTECTION: enNlYW5vb2Zjb3Vyc2U="`: Authentication header
- `--http1.1`: Force HTTP/1.1 protocol
- Target URL with the discovered API endpoint

---

## 🎯 Step 4: Flag Extraction

The curl request returned the following JSON response:

```json
{'email':'zseano@ofcourse.com','flag':'^FLAG^89ec6cd190ffb06f93bc09fa5c389f6a2ad8d2849ec8518c71f7c525526a2a2e$'}
```

However, I noticed the flag was missing the closing `FLAG$` marker. After manually adding this to complete the flag format:

**Complete Flag:**
```
^FLAG^89ec6cd190ffb06f93bc09fa5c389f6a2ad8d2849ec8518c71f7c525526a2a2e$FLAG$
```

---

## 🏁 Solution Summary

The key steps to solve this challenge were:

1. **Ignore the misleading challenge name** - This wasn't about XSS at all
2. **Perform reconnaissance** - Examine JavaScript files for API endpoints
3. **Identify authentication mechanisms** - Find required headers in the source code
4. **Craft the exploit** - Use curl with proper headers to access the protected endpoint
5. **Format the flag correctly** - Add the missing `FLAG$` suffix

---

## 💡 Key Takeaways

1. **Challenge names can be misleading** - Always investigate thoroughly regardless of the stated category
2. **Client-side reconnaissance is crucial** - JavaScript files often contain sensitive API information
3. **Authentication headers matter** - Many APIs use custom headers for access control
4. **Flag formatting** - Sometimes flags need manual formatting to match the expected structure

---

## 🔧 Tools Used

- **curl** - Command-line tool for HTTP requests
- **Browser Developer Tools** - For examining JavaScript files
- **Base64 decoder** - To understand the authentication token

---

## 🎭 Final Thoughts

While initially disappointing that this wasn't actually an XSS challenge, it provided valuable practice in:
- API endpoint discovery
- Authentication bypass techniques  
- Client-side reconnaissance
- Understanding misleading challenge descriptions

The challenge effectively demonstrates that real-world security testing requires looking beyond obvious attack vectors and thoroughly investigating all application components.

---

**Challenge Status: ✅ Completed**