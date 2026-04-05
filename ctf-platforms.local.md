# CTF / Wargame Inventory Scope (local)

Generated: 2026-04-04

Goal:
Store a local inventory of publicly discoverable CTF rooms/challenges from the platforms below, with explicit completeness metadata and unresolved targets.

## Public inventory artifacts (ignored in git)

- `ctf-room-inventory.local.md` (newline-delimited challenge names; one challenge per line)
- `ctf-room-inventory.local.json` (machine-readable snapshot with metadata)

Last generated: `2026-04-04`

Current captured public records: `13234`

- 247CTF: `72` challenges
- Blue Team Labs Online: `57` writeup-indexed labs/investigations
- CMD Challenge: `59` shell-wargame challenges
- CTFlearn: `217` challenges
- CTFtime: `2792` events
- CTFtime Event Challenges: `23` encoding/decoding-themed writeup-indexed challenge names
- Encoding Blog Examples: `28` blog-discovered encoding/cipher/esolang challenge names
- CryptoHack: `308` challenges
- CyberDefenders: `231` labs
- Exploit Education: `88` challenge/level pages
- Hack The Box: `552` records (`551` public machines + `1` writeup-derived retired target)
- Hack This Site: `77` challenge names from mixed official-page and writeup indexing
- Hacker101: `28` writeup-indexed challenges
- Hackropole: `542` archived challenges
- LetsDefend: `135` challenges
- Microcorruption: `25` levels
- OverTheWire: `199` level entries
- PentesterLab: `693` unique public exercise slugs
- PortSwigger Web Security Academy: `269` labs
- ROP Emporium: `8` challenges
- RingZer0 Team: `419` challenges
- Root-Me: `294` writeup-indexed challenges
- TryHackMe: `1003` rooms
- UnderTheWire: `80` level entries
- VulnHub: `144` machines
- W3Challs: `98` challenges
- picoCTF: `398` writeup-indexed challenges
- pwn.college: `4239` challenges
- pwnable.kr: `59` challenges
- pwnable.tw: `46` challenges
- pwnable.xyz: `51` challenges

Hard limitation:
No single authoritative global source exists for “every single CTF room/challenge everywhere,” because many platforms gate content behind accounts, retire content, and hide private/invite-only material.

## Platforms to include

- [247CTF](https://www.247ctf.com/)
- [Hack The Box](https://www.hackthebox.com/)
- [TryHackMe](https://tryhackme.com/)
- [CTFtime](https://ctftime.org/)
- [Root-Me](https://www.root-me.org/)
- [RingZer0 Team](https://ringzer0ctf.com/)
- [CTFlearn](https://ctflearn.com/)
- [Hacker101](https://www.hacker101.com/)
- [picoCTF](https://picoctf.org/)
- [CryptoHack](https://cryptohack.org/)
- [Hackropole](https://hackropole.fr/en/challenges/)
- [OverTheWire](https://overthewire.org/wargames/)
- [UnderTheWire](https://www.underthewire.tech/)
- [pwn.college](https://pwn.college/)
- [pwnable.kr](https://pwnable.kr/)
- [pwnable.tw](https://pwnable.tw/)
- [pwnable.xyz](https://pwnable.xyz/challenges/)
- [W3Challs](https://pwn.w3challs.com/)
- [Hack This Site](https://www.hackthissite.org/)
- [VulnHub](https://www.vulnhub.com/)
- [Microcorruption](https://microcorruption.com/)
- [Exploit Education](https://exploit.education/)
- [ROP Emporium](https://ropemporium.com/)
- [PortSwigger Web Security Academy](https://portswigger.net/web-security/)
- [PentesterLab](https://www.pentesterlab.com/)
- [CyberDefenders](https://cyberdefenders.org/)
- [Blue Team Labs Online](https://blueteamlabs.online/)
- [LetsDefend](https://letsdefend.io/)
- [CMD Challenge](https://cmdchallenge.com/)
- Blog-discovered encoding/cipher examples

## Requested entries (examples)

- Hack The Box: `Invite`
- TryHackMe: `Capture the Flag`

## Requested entries (explicitly tracked)

- Platform: Hack The Box
  - Target: Invite
  - Status: writeup_discovered
  - Notes: current public HTB indexes do not expose it; retained from public writeup discovery against the retired invite flow.

- Platform: TryHackMe
  - Target: Capture the Flag
  - Status: resolved_public
  - Notes: resolved to public room slug `capture`.

## Output files (all ignored by `.gitignore`)

- `ctf-room-inventory.local.md` (newline-delimited challenge names)
- `ctf-room-inventory.local.json` (machine-readable metadata)

## Requested canonical targets

Known target names to keep in scope:

- `Invite` (Hack The Box)
- `Capture the Flag` (TryHackMe room naming family)

## Collection status by platform

- [247CTF](https://www.247ctf.com/): completed via public `challenges/type/*` JSON endpoints linked from the public dashboard (`72` challenges).
- [Hack The Box](https://www.hackthebox.com/): completed for public `/machines` page (`551` records).
- [TryHackMe](https://tryhackme.com/): completed via public sitemap `sitemaps/rooms.xml` (`1003` records).
- [CTFtime](https://ctftime.org/): completed via public events API for event inventory; targeted passes also captured `23` encoding/decoding-focused challenge names from public CTFtime writeup/task pages under the synthetic `CTFtime Event Challenges` bucket.
- Blog-discovered encoding/cipher examples: targeted writeup/blog passes captured `28` names from public posts and retrospectives under the synthetic `Encoding Blog Examples` bucket.
- [RingZer0 Team](https://ringzer0ctf.com/): completed via public challenges index.
- [OverTheWire](https://overthewire.org/wargames/): completed via public `games.json` level metadata.
- [UnderTheWire](https://www.underthewire.tech/): completed via official app bundle wargame metadata (`80` level entries across Century, Cyborg, Groot, Oracle, and Trebek).
- [pwnable.kr](https://pwnable.kr/): completed for publicly visible challenge index cards.
- [pwnable.tw](https://pwnable.tw/): completed via public `/challenge/` challenge list.
- [pwnable.xyz](https://pwnable.xyz/challenges/): completed via public challenge cards (`51` challenges).
- [CTFlearn](https://ctflearn.com/): completed via public `/challenge/api/1/browse`.
- [CryptoHack](https://cryptohack.org/): completed via public category pages (`308` challenge entries).
- [Hackropole](https://hackropole.fr/en/challenges/): completed via public challenge archive table (`542` challenges).
- [W3Challs](https://pwn.w3challs.com/): completed via public AJAX JSON challenge list (`98` challenges).
- [pwn.college](https://pwn.college/): completed via public `/dojos`, dojo module pages, and module challenge pages (`4239` challenge entries).
- [VulnHub](https://www.vulnhub.com/): completed via public paginated `/entry/` cards (`144` machines).
- [Exploit Education](https://exploit.education/): completed via public site navigation (`88` challenge/level pages).
- [ROP Emporium](https://ropemporium.com/): completed via public challenge cards (`8` challenges).
- [PortSwigger Web Security Academy](https://portswigger.net/web-security/): completed via public sitemap + lab pages (`269` labs).
- [PentesterLab](https://www.pentesterlab.com/): completed via public `/exercises` pagination (`693` unique slugs; the site advertises `699` rows but repeats `6` slugs).
- [CyberDefenders](https://cyberdefenders.org/): completed via public embedded Next.js lab state (`231` labs).
- [LetsDefend](https://letsdefend.io/): completed via public embedded Next.js challenge state (`135` challenges).
- [Microcorruption](https://microcorruption.com/): completed via official app bundle `App.d0920593.js` (`25` levels).
- [CMD Challenge](https://cmdchallenge.com/): completed via official front-end bundle `assets/index-e4affdb0.js` (`59` shell-wargame challenges across the main, `oops`, and `12days` tracks).
- [Hacker101](https://www.hacker101.com/): best-effort writeup-indexed inventory via public `testert1ng/hacker101-ctf` README (`28` challenge names); official platform challenge index is not publicly exposed.
- [picoCTF](https://play.picoctf.org/): best-effort writeup-indexed inventory via public `Cajac/picoCTF-Writeups`, `HHousen/PicoCTF-2022`, `slashben/hadassah-picoctf-2023-writeups`, `noamgariani11/picoCTF-2024-Writeup`, `snwau/picoCTF-2025-Writeup`, `asatpathy314/picoctf-2025`, and `imattas/PicoCTF-2026-writeups` (`398` challenge names); official practice index remains Cloudflare/auth blocked.
- [Root-Me](https://www.root-me.org/): best-effort writeup-indexed inventory via public `aadityadhruv-zz/wargaming-challenges` README (`294` official challenge URLs), cross-checked against `ton11797/RootMe-Challenges` and `stefanman125/root-me-challenges`; direct official enumeration remains blocked by anti-bot/proof-of-work.
- [Hack This Site](https://www.hackthissite.org/): mixed official-page and writeup-indexed inventory via public HTS challenge-tutorial articles, public realistic mission pages, and public `norbert-dev/Hack_This_Site`, `divyanshusahu/HackThisSite`, `Sivnerof/HackThisSite`, `NotSurprised/HackThisSite-Writeup`, `fix-you/HackThisSite_WriteUp`, `alicansa/HackThisSite`, and `Bechma/HackThisSite-Programming` repos (`77` challenge names); realistic missions `1-16` are now covered from official mission pages or official HTS tutorial titles, but phone phreaking and parts of application/programming coverage are still missing.
- [Blue Team Labs Online](https://blueteamlabs.online/): partial writeup-indexed inventory via public `nguyenhuudang04/blue-team-labs-online-writeups`, `jaimealruiz/BTLO-writeups`, `skerrittrichard-hash/BTLO-Writeups`, `Baniur/Writeups`, `ntduong273/btlo_writeup`, and `VincentHuynh2956/Vhuynh-BlueTeamLabs-Writeups` (`57` names); official platform catalog remains login-gated.

## Fullness ledger

- Hack The Box:
  - Complete for `Machines` public index.
  - `Invite` is retained as a writeup-discovered retired target.
  - Incomplete for the broader public challenge/private challenge-style catalog.
- TryHackMe:
  - Complete for public room sitemap.
  - `Capture the Flag` resolves to room slug `capture`.
- CTFtime:
  - Complete for public event inventory via the events API.
  - A targeted writeup-indexed sub-bucket now adds `23` encoding/decoding-themed event challenge names, including `Totem`, `Never Ending Crypto Redux`, `Home Base`, `Encode Mania`, `three-step-program`, `Gotta Decrypt em All`, `Morset`, `CR1 Ultracoded`, `Too Secret`, `Black Is The New Rose`, `Danger Zone`, `base646464`, `Magically Delicious`, `Crypto Infinite`, `Mind Boggle`, `Lost in Transliteration`, `bynary encoding`, `Nobody uses the eggplant emoji`, `Emoji m***********. Do you speak it?`, `Hello`, `Baby Encoder`, `Warmup Encoder`, and `Shakespeare and Encoding`.
  - This sub-bucket is intentionally narrow and not a general authoritative challenge catalog for all CTFtime events.
- Encoding Blog Examples:
  - A targeted blog/writeup bucket now adds `28` names: `1200 Transmissions`, `Base-p-`, `Base64by32`, `BaseFFFF+1`, `BASEic Encoding`, `Beware the Idles of March`, `CaesarMirror`, `Cattle`, `Chicken Wings`, `Comprezz`, `Convert HEX to base64`, `Cover your Bases`, `Discount Programming Devices`, `Flag delivery`, `Gaius`, `Hail Caesar!`, `No Need for Brutus`, `Obfuscation Station`, `Ping Me`, `Ran Somewhere`, `Something Sw33t`, `Steg Ultimate`, `Sus`, `Too Many Bits`, `TXT Message`, `VeeBeeEee`, `Zimmer Down`, and `Zulu`.
  - These are intentionally provenance-scoped examples discovered from public blog posts and retrospectives, not an official platform inventory.
- 247CTF, CMD Challenge, CTFlearn, CTFtime, CryptoHack, CyberDefenders, Exploit Education, Hackropole, LetsDefend, Microcorruption, OverTheWire, PentesterLab, PortSwigger Web Security Academy, ROP Emporium, RingZer0 Team, UnderTheWire, VulnHub, W3Challs, pwn.college, pwnable.kr, pwnable.tw, pwnable.xyz:
  - Completed from public indexes, public APIs, or official app bundles at scrape time.
- Root-Me:
  - `294` challenge names are captured from a broad public writeup index and keep official Root-Me challenge URLs.
  - Cross-checking against smaller public Root-Me repos did not produce a material net-new expansion after normalization.
  - Not authoritative against the live platform because anti-bot protection blocks direct enumeration.
- Hacker101:
  - `28` challenge names are captured from a public writeup index.
  - Not authoritative against the live platform because Hacker101 does not publish a public full challenge list.
- picoCTF:
  - `398` challenge names are captured from public multi-year writeup indexes, including broader 2022 gap-fill coverage plus public 2025 and 2026 writeup collections.
  - Not authoritative against the live platform because the public practice catalog is Cloudflare/auth blocked.
- Hack This Site:
  - `77` challenge names are captured from a mix of official HTS mission pages, official HTS challenge-tutorial articles, and public writeup indexes.
  - Realistic missions `1-16` are now covered, mostly from public official mission pages; application missions `1-5`, `13`, and `16` and programming missions `1-6`, `8`, `11`, and `12` are also captured.
  - Still partial; phone phreaking and parts of application/programming coverage remain missing, and the category index pages are still login-gated.
- Blue Team Labs Online:
  - `57` lab/investigation names are captured from public writeup indexes.
  - Coverage includes `Cerulean`, `Piggy`, `Bruteforce`, and `PowerShell Analysis - Keylogger`, but the live platform catalog still requires login and is not publicly enumerable.
- Additional discovery notes:
  - Community-led discovery surfaced 247CTF, Hackropole, W3Challs, UnderTheWire, Exploit Education, pwnable.xyz, ROP Emporium, and CMD Challenge as viable public catalogs; those are now included.
  - Separate blog/example sweeps added encoding-specific names from public Huntress, Hacktoberfest, and related writeup posts, plus additional CTFtime task/writeup examples for encoding-focused challenges.
  - FreeHackQuest is publicly reachable, but this pass did not merge it because the public SPA did not expose a clean title-level challenge list without deeper websocket/API reverse-engineering.
  - WeChall was useful as a discovery aid for candidate sites, but it was not used as authoritative inventory data.
- Remaining hard gaps in `Platforms to include`:
  - Root-Me, Hack This Site, and Blue Team Labs Online remain non-authoritative because live-platform enumeration is blocked, partial, or gated.

## Minimum schema for authoritative snapshot

- `platform`
- `category` (`event`, `room`, `machine`, `challenge`, `practice`)
- `name`
- `slug_or_id`
- `url`
- `first_seen`
- `last_seen`
- `visibility` (`public`, `private`, `invite-only`, `retired`)
- `source`

## Why a true complete static list is still impossible

To satisfy “every single CTF,” this workspace would need:

- Authenticated access for all gated platform APIs.
- Periodic crawling of content changes and deletions.
- Historic snapshots retained for retired and removed content.
- Deduping between mirrors and rebranded events.

Until those requirements are met, this file is the source-of-truth for scoped public inventory, not truly global finality.
