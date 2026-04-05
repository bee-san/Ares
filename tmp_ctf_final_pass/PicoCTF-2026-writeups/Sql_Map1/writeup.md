# Sql Map1 - picoCTF 2026

**Category:** Web Exploitation
**Points:** 300

## Challenge Description

You've been hired by a shadowy group of pentesters who love a good puzzle. The system looks ordinary, but appearances lie.

We are given a web application URL hosting what appears to be a standard login page or search interface backed by an SQL database. The challenge name is a direct hint: use **sqlmap** to automate discovery and exploitation of SQL injection vulnerabilities.

## Approach

The challenge presents a web application with one or more user-input fields that are vulnerable to SQL injection. The name "Sql Map1" strongly hints that the intended solution path is to use `sqlmap`, the popular open-source SQL injection automation tool.

### Reconnaissance

1. **Identify the injection point.** Browse the application and identify forms or URL parameters that interact with the database. Common targets include login forms (`username`, `password` fields) and search/filter parameters in GET or POST requests.

2. **Capture the request.** Use browser developer tools (Network tab) or Burp Suite to capture the exact HTTP request being sent when the form is submitted. Note the URL, method (GET/POST), parameters, and any cookies or headers required (especially session cookies).

3. **Confirm SQL injection.** Manually test with a simple payload like `' OR 1=1 --` to confirm the parameter is injectable before unleashing sqlmap.

### Exploitation with sqlmap

sqlmap automates the entire process:
- **Detection:** It fingerprints the database type (SQLite, MySQL, PostgreSQL, etc.) and identifies the injection technique (UNION-based, blind boolean, blind time-based, error-based, stacked queries).
- **Enumeration:** It lists databases, tables, and columns.
- **Extraction:** It dumps table contents, which typically contain the flag.

For picoCTF challenges, the backend is commonly **SQLite** and the flag is usually stored in a dedicated table (e.g., `flags`, `secrets`, or `more_table`).

### Key sqlmap flags

- `-u URL` -- target URL with the injectable parameter marked
- `--data "param=value"` -- for POST requests
- `--cookie "session=..."` -- include session cookies if required
- `--forms` -- automatically detect and test forms on the page
- `--dbs` -- enumerate databases
- `--tables -D dbname` -- list tables in a database
- `--dump -T tablename` -- dump all rows from a table
- `--dump-all` -- dump everything
- `--batch` -- non-interactive mode (accept defaults)
- `--level 3 --risk 2` -- increase detection sensitivity

## Solution

### Step 1: Identify the target

Navigate to the challenge URL in a browser. Note the form fields and the URL structure.

### Step 2: Run sqlmap against the target

For a GET-based injection point:
```bash
sqlmap -u "http://<challenge-url>/search?query=test" --batch --dump-all
```

For a POST-based login form:
```bash
sqlmap -u "http://<challenge-url>/login" --data "username=admin&password=pass" --batch --dump-all
```

If the application requires a session cookie:
```bash
sqlmap -u "http://<challenge-url>/search?query=test" --cookie "session=<your-session-cookie>" --batch --dump-all
```

### Step 3: Let sqlmap enumerate and dump

sqlmap will:
1. Detect the injection type and DBMS
2. Enumerate all tables
3. Dump all table contents

### Step 4: Find the flag

Look through the dumped output for a value matching `picoCTF{...}`. It is typically in a table like `flags`, `secrets`, or `hidden`.

### Alternative: Use --forms for automatic form detection

```bash
sqlmap -u "http://<challenge-url>/" --forms --batch --dump-all
```

This tells sqlmap to crawl the page, find all forms, and test each field automatically.

## Solution Script

```
python3 solve.py
```

The script automates the process: it connects to the challenge URL, identifies the injectable endpoint, runs sqlmap programmatically, and extracts the flag from the dumped output.

## Flag

```
picoCTF{...}  (placeholder - actual flag varies per instance)
```
