# anitools

Small Python utilities for the exact media workflow from this thread:

- convert MKVs to browser-safe AAC audio while copying video/subtitles
- download matching `.srt` sets from Jimaku
- measure subtitle offsets against subtitle tracks already inside the MKVs
- retime external `.srt` files with backups
- assemble videos and subtitles into one player-ready folder

The script is stdlib-only. The only external requirement is `ffmpeg` on `PATH`.

## Requirements

- Python 3.10+
- `ffmpeg` on `PATH`

## File Layout

- `anitools.py`: single-file CLI

## Commands

### 1. Convert MKVs to Chrome-safe AAC

This keeps the first video stream, all audio streams, all subtitle streams, chapters, and font attachments, then re-encodes audio to AAC.

```powershell
python .\anitools.py convert-chrome `
  --input-dir "E:\Summer Pockets S01 1080p WEBRip DD+ x265-EMBER" `
  --output-dir "E:\Summer Pockets S01 1080p WEBRip DD+ x265-EMBER\Player Ready" `
  --overwrite
```

Notes:

- It intentionally maps only `0:v:0`, which avoids broken extra attached-picture video streams.
- It maps `0:t?`, so embedded font attachments are preserved.

### 2. Download Japanese SRTs from Jimaku

Default preset is the `WEBRip.Netflix.ja[cc]` style we used here.

```powershell
python .\anitools.py download-jimaku `
  --entry 9430 `
  --output-dir "E:\Summer Pockets S01 1080p WEBRip DD+ x265-EMBER\Player Ready" `
  --rename-template "[EMBER] Summer Pockets - {episode:02d}.srt" `
  --overwrite
```

Built-in presets:

- `webrip-netflix-ja-cc`
- `nanakoraws`
- `shincaps`

You can also override the built-in filter:

```powershell
python .\anitools.py download-jimaku `
  --entry https://jimaku.cc/entry/9430 `
  --filter-regex "^Summer\.Pockets\.S01E\d{2}.*\.srt$" `
  --output-dir ".\subs"
```

### 3. Measure Subtitle Offsets

This extracts subtitle stream `0:s:0` from each MKV, compares its cue timing to the external `.srt`, and finds the best constant shift.

```powershell
python .\anitools.py measure-offsets `
  --media-dir "E:\Summer Pockets S01 1080p WEBRip DD+ x265-EMBER\Player Ready" `
  --output-json ".\offsets.json"
```

Useful flags:

- `--reference-stream 0`
- `--min-shift -2.0`
- `--max-shift 2.0`
- `--search-step 0.05`
- `--score-step 0.10`

### 4. Apply a Subtitle Retime

Constant shift:

```powershell
python .\anitools.py retime-srt `
  --subtitle-dir "E:\Summer Pockets S01 1080p WEBRip DD+ x265-EMBER\Player Ready" `
  --shift -1.0
```

Per-file offsets from JSON:

```powershell
python .\anitools.py retime-srt `
  --subtitle-dir "E:\Summer Pockets S01 1080p WEBRip DD+ x265-EMBER\Player Ready" `
  --offsets-json ".\offsets.json"
```

Notes:

- Original `.srt` files are copied into `_before_retime` by default.
- The command edits the `.srt` files in place.

### 5. Assemble a Player-Ready Folder

```powershell
python .\anitools.py assemble-player-ready `
  --video-dir "E:\Summer Pockets S01 1080p WEBRip DD+ x265-EMBER\Chrome AAC" `
  --subtitle-dir "E:\Summer Pockets S01 1080p WEBRip DD+ x265-EMBER\Japanese SRT" `
  --output-dir "E:\Summer Pockets S01 1080p WEBRip DD+ x265-EMBER\Player Ready" `
  --overwrite
```

If both directories are on the same volume, you can use hardlinks instead of copies:

```powershell
python .\anitools.py assemble-player-ready `
  --video-dir ".\video" `
  --subtitle-dir ".\subs" `
  --output-dir ".\Player Ready" `
  --link-mode hardlink
```

## Summer Pockets Workflow

This is the exact sequence that reproduces what we did:

```powershell
python .\anitools.py convert-chrome `
  --input-dir "E:\Summer Pockets S01 1080p WEBRip DD+ x265-EMBER" `
  --output-dir "E:\Summer Pockets S01 1080p WEBRip DD+ x265-EMBER\Player Ready" `
  --overwrite

python .\anitools.py download-jimaku `
  --entry 9430 `
  --output-dir "E:\Summer Pockets S01 1080p WEBRip DD+ x265-EMBER\Player Ready" `
  --rename-template "[EMBER] Summer Pockets - {episode:02d}.srt" `
  --overwrite

python .\anitools.py measure-offsets `
  --media-dir "E:\Summer Pockets S01 1080p WEBRip DD+ x265-EMBER\Player Ready" `
  --output-json ".\offsets.json"

python .\anitools.py retime-srt `
  --subtitle-dir "E:\Summer Pockets S01 1080p WEBRip DD+ x265-EMBER\Player Ready" `
  --offsets-json ".\offsets.json"
```

## Limitations

- `measure-offsets` assumes the external subtitle file has the same base name as the MKV.
- `convert-chrome` is tuned for the browser-safe MKV case, not full transcoding to H.264 MP4.
- `download-jimaku` relies on the file list being present in the entry HTML.
