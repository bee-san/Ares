# ORDER ORDER - picoCTF 2026

**Category:** Web Exploitation
**Points:** 300

## Challenge Description

Can you try to get the flag from our website. I've prepared my queries everywhere! I think!

The developer believes they have parameterized all SQL queries, but missed one -- the ORDER BY clause. Since ORDER BY takes a column name or expression (not a value), it cannot be parameterized using standard prepared statement placeholders. This is a common oversight even among security-aware developers.

## Approach

### Identifying the Vulnerability

When interacting with the web application, we notice a sorting feature -- likely a table of items (e.g., metals, products, users) that can be sorted by clicking column headers or via a query parameter like `?order=name` or `?sort=atomic_number`.

Inspecting the requests reveals that the sort/order parameter is passed directly into the SQL query. While the developer correctly used parameterized queries for WHERE clauses and user input fields, the ORDER BY clause was left vulnerable because:

1. **ORDER BY cannot be parameterized** in most SQL libraries -- prepared statements only protect *values*, not *identifiers* (column names, table names, sort directions).
2. The developer likely assumed all queries were safe since they used parameterized queries "everywhere."

### The Vulnerable Code Pattern

The backend likely looks something like this:

```python
# Parameterized (safe) - WHERE clause
query = "SELECT * FROM items WHERE category = ?"
cursor.execute(query, (user_input,))

# NOT parameterized (vulnerable!) - ORDER BY clause
order = request.args.get('order', 'name')
query = f"SELECT * FROM items ORDER BY {order}"
cursor.execute(query)
```

### Exploitation Strategy: Boolean-Based Blind SQL Injection via ORDER BY

Since ORDER BY doesn't produce direct output, we use a **blind SQL injection** technique with `CASE WHEN` conditional expressions. The idea:

- Inject a `CASE WHEN (condition) THEN column_a ELSE column_b END` into the ORDER BY clause.
- If the condition is **true**, results sort by `column_a` (e.g., alphabetical by name).
- If the condition is **false**, results sort by `column_b` (e.g., numerical by ID).
- By observing which sort order is returned, we can infer one bit of information per request.

We use SQLite's `substr()` function to extract the flag character-by-character:

```sql
CASE WHEN (SELECT substr(flag,1,1) FROM flag)='p' THEN name ELSE id END
```

## Solution

### Step 1: Confirm the Injection Point

Test with a valid column name vs. an invalid one:
- `?order=name` -- works, sorts alphabetically
- `?order=1;--` -- may cause an error or different behavior

Test with a CASE expression:
- `?order=CASE WHEN 1=1 THEN name ELSE id END` -- sorts by name (true)
- `?order=CASE WHEN 1=2 THEN name ELSE id END` -- sorts by id (false)

If the sort order changes, the injection is confirmed.

### Step 2: Enumerate the Database

Discover table names (SQLite):
```sql
CASE WHEN (SELECT count(*) FROM sqlite_master WHERE type='table' AND name='flag')>0 THEN name ELSE id END
```

### Step 3: Extract the Flag Character-by-Character

For each position `i` in the flag, iterate through possible characters and check:
```sql
CASE WHEN (SELECT substr(flag,{i},1) FROM flag)='{char}' THEN name ELSE id END
```

Observe the response sort order to determine if the character matches.

### Step 4: Optimize with Binary Search

Instead of testing each character individually (up to ~95 printable ASCII characters), use binary search on the ASCII value:
```sql
CASE WHEN (SELECT unicode(substr(flag,{i},1)) FROM flag)>{mid} THEN name ELSE id END
```

This reduces each character extraction from ~95 requests to ~7 requests.

## Solution Script

```
python3 solve.py
```

## Flag

```
picoCTF{...}  (placeholder - actual flag varies per instance)
```
