# Paper 2 - picoCTF 2025

**Category:** Web Exploitation
**Points:** 500
**Flag:** `picoCTF{i_l1ke_frames_on_my_canvas_953d5fff}`

## Challenge Description

A file-sharing service backed by Redis lets users upload files and have an admin bot visit uploaded pages. The goal is to recover a 32-character hex secret that the bot carries in a cookie and submit it to `/flag` to retrieve the flag.

## Source Code Analysis

The application is a Bun/TypeScript server with a Redis backend. Key observations:

**Redis configuration** (`docker-compose.yml`):

```
redis-server --maxmemory 512M --maxmemory-policy allkeys-lru
```

Redis is capped at 512 MB with an **LRU eviction policy** — when memory is full, the least-recently-used key gets evicted.

**Upload & storage** — files are stored as base64-encoded JSON in Redis keys with a 10-minute TTL:

```typescript
await redis.set(`file|${id}`, data, 'EX', 10 * 60);
```

**The secret** — when the bot visits, a random 32-hex-char secret is generated, set as a browser cookie, and stored in Redis with a 60-second TTL:

```typescript
const secret = randomBytes(16).toString('hex');
await browser.setCookie({ name: 'secret', value: secret, domain: host, sameSite: 'Strict' });
await redis.set('secret', secret, 'EX', 60);
```

**The `/secret` endpoint** — returns the secret from the cookie as an HTML attribute, parseable by XSLT:

```typescript
const secret = req.cookies.get('secret') || '0123456789abcdef'.repeat(2);
return new Response(`<body secret="${secret}">${secret}\n${payload}</body>`, ...);
```

**The `/flag` endpoint** — uses `getdel`, meaning the secret is deleted on the first guess attempt regardless of correctness. We get exactly **one shot**:

```typescript
const secret = await redis.getdel('secret');
```

**Content Security Policy** — `script-src 'none'` blocks all JavaScript. `default-src 'self' 'unsafe-inline'` restricts all resource loads to the same origin. No data exfiltration to external servers is possible.

## Vulnerability: XSLT + Redis LRU Side Channel

Since JavaScript is completely blocked, we use **XSLT** to read the secret and **Redis LRU eviction** as a side channel to exfiltrate it bit by bit.

Chrome natively processes XML documents with embedded `<?xml-stylesheet?>` processing instructions. An XSLT stylesheet can use `document('/secret')` to fetch the `/secret` endpoint (same-origin, so the bot's cookie is included) and read the secret from the `<body secret="...">` attribute. It can then conditionally render `<img>` tags that cause the browser to make GET requests to specific marker files, selectively "touching" them in the Redis LRU cache.

## Attack Overview

The attack has five phases, all executed within a single 60-second bot visit window:

### Phase 1: Upload Marker Pairs

For each of the 128 bits in the secret (32 hex chars x 4 bits), we upload two marker files: a **zero-marker** and a **one-marker**. We use 10 independent replicas for redundancy, giving 2560 total marker pairs (5120 files).

Each marker is a 20 KB random blob — small enough to fit many in cache but large enough to be reliably detected via `HEAD` requests.

### Phase 2: Upload Prefill Buffer

We upload 1500 "sacrificial" 60 KB filler entries. These serve as an LRU age buffer between the markers and the Redis `secret` key, protecting the secret from accidental eviction during the postfill phase.

### Phase 3: Upload XSLT Payloads

For each replica, we generate a self-contained XSLT document that:

1. Fetches `/secret` via `xsl:document()` to read the secret attribute.
2. For each bit position, tests whether the corresponding hex character belongs to a predetermined character set using `xsl:if` with `contains()` and `substring()`.
3. Conditionally renders `<img>` tags pointing to either the zero-marker or one-marker.

The bit encoding uses four character sets that partition hex digits into groups, allowing each hex character to be resolved from four binary tests:

```
bit 0: "01234567" vs "89abcdef"
bit 1: "012389ab" vs "4567cdef"
bit 2: "014589cd" vs "2367abef"
bit 3: "02468ace" vs "13579bdf"
```

A launcher HTML page embeds all 10 XSLT documents as iframes.

### Phase 4: Trigger Bot Visit + Eviction

We trigger the bot to visit the launcher page and wait ~25 seconds for the browser to process all XSLT iframes and load the conditional marker images. Each image load performs a `redis.get()` on the marker key, refreshing its LRU access timestamp.

After the wait, we upload ~4500 large (60 KB) postfill entries to push total Redis memory past the 512 MB limit, triggering LRU eviction. The untouched markers (the ones the bot's XSLT did **not** load) have the oldest access timestamps and are evicted first.

### Phase 5: Probe + Decode + Submit

We issue `HEAD` requests against all marker files. A marker that returns `Content-Length > 100` is alive; one that returns a small "not found" response was evicted.

For each bit position across all replicas, we take a **majority vote**: if more replicas show the zero-marker alive and one-marker dead, the bit is `0`, and vice versa. With 10 replicas and ~80 differential signals per replica, we achieve 128/128 bit accuracy.

The decoded 32-character hex secret is submitted to `/flag` to retrieve the flag.

## Key Constraints

- **One guess only.** The `/flag` endpoint uses `getdel`, deleting the secret after the first attempt. The decode must be correct on the first try.

- **No JavaScript.** `script-src 'none'` forces us to use XSLT as the in-browser computation primitive.

- **Same-origin only.** `default-src 'self'` prevents exfiltrating data to an external server — we must use the server's own storage (Redis) as the side channel.

- **Protecting the secret key.** With `allkeys-lru`, the Redis `secret` key itself can be evicted by our postfill. The prefill buffer and careful postfill sizing ensure the secret key survives. Too much postfill evicts it; too little gives insufficient signal.

- **Timing budget.** The secret has a 60-second TTL. We must upload markers, wait for the browser, postfill, probe, and submit all within this window.

## Solution Script

```
python3 solve.py
```

The solver auto-calibrates upload/probe speeds, computes optimal parameters (replicas, wait time, postfill count), and executes the full attack in one run.

## Flag

```
picoCTF{i_l1ke_frames_on_my_canvas_953d5fff}
```
