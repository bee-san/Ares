# Basic Mission 11 – Directory Traversal & .htaccess

**Objective:** exploit Apache misconfiguration to access hidden files.

---

## Approach
1. Browse through `/music/`, look for hidden directories.
2. Find `.htaccess` in path like `/e/l/t/o/n/.htaccess`.
3. Extract protected folder (`DaAnswer`) and fetch `.txt`.
4. Login at `index.php` using the answer.

---

## Solution Steps
1. Navigate directories: `/e/`, then `/l/`, `/t/`, `/o/`, `/n/`.
2. Found `.htaccess`, saw redirect to `DaAnswer`.
3. Retrieved `DaAnswer.txt`: “The answer is right here…”
4. Entered that into `index.php`, mission complete.

---

## Concept
Shows file/directory traversal and Apache `.htaccess` control file importance.

---

## Takeaways
- Hide sensitive areas properly—don’t rely on `.htaccess` alone.
