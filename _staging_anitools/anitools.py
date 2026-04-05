#!/usr/bin/env python3
"""Small CLI utilities for prepping anime files for browser and player use."""

from __future__ import annotations

import argparse
import html
import json
import re
import shutil
import subprocess
import sys
import tempfile
import urllib.parse
import urllib.request
from pathlib import Path
from statistics import median


JIMAKU_BASE_URL = "https://jimaku.cc"

PRESET_PATTERNS = {
    "webrip-netflix-ja-cc": re.compile(
        r"^.+S\d+E(\d{2})\..*\.WEBRip\.Netflix\.ja\[cc\]\.srt$"
    ),
    "nanakoraws": re.compile(
        r"^\[NanakoRaws\].* - (\d{2})(?: END)? \(AT-X 1920x1080 x265 AAC\)\.srt$"
    ),
    "shincaps": re.compile(
        r"^\[shincaps\].* - (\d{2}) \(AT-X 1440x1080 MPEG2 AAC\)\.srt$"
    ),
}

TIMESTAMP_RE = re.compile(
    r"(\d{2}:\d{2}:\d{2},\d{3})\s+-->\s+(\d{2}:\d{2}:\d{2},\d{3})"
)


def ensure_tool(name: str) -> str:
    path = shutil.which(name)
    if not path:
        raise SystemExit(f"Required tool not found on PATH: {name}")
    return path


def run(cmd: list[str]) -> None:
    print("+", " ".join(f'"{c}"' if " " in c else c for c in cmd))
    subprocess.run(cmd, check=True)


def fetch_text(url: str) -> str:
    with urllib.request.urlopen(url) as response:
        return response.read().decode("utf-8", errors="replace")


def download_file(url: str, dest: Path) -> None:
    with urllib.request.urlopen(url) as response:
        dest.write_bytes(response.read())


def iter_files(directory: Path, pattern: str) -> list[Path]:
    return sorted(path for path in directory.glob(pattern) if path.is_file())


def parse_jimaku_entry_files(entry_html: str) -> list[dict[str, str]]:
    files: list[dict[str, str]] = []
    for match in re.finditer(r'data-extra="(?P<json>.*?)"', entry_html):
        payload = html.unescape(match.group("json"))
        try:
            obj = json.loads(payload)
        except json.JSONDecodeError:
            continue
        name = obj.get("name")
        url = obj.get("url")
        if not name or not url:
            continue
        files.append(
            {
                "name": name,
                "url": urllib.parse.urljoin(JIMAKU_BASE_URL, url),
            }
        )
    return files


def normalize_jimaku_entry(entry: str) -> str:
    if entry.isdigit():
        return f"{JIMAKU_BASE_URL}/entry/{entry}"
    return entry


def parse_timestamp(value: str) -> float:
    hours, minutes, rest = value.split(":")
    seconds, millis = rest.split(",")
    return (
        int(hours) * 3600
        + int(minutes) * 60
        + int(seconds)
        + int(millis) / 1000.0
    )


def format_timestamp(value: float) -> str:
    if value < 0:
        value = 0.0
    millis_total = int(round(value * 1000))
    hours, rem = divmod(millis_total, 3_600_000)
    minutes, rem = divmod(rem, 60_000)
    seconds, millis = divmod(rem, 1_000)
    return f"{hours:02d}:{minutes:02d}:{seconds:02d},{millis:03d}"


def parse_srt_intervals(path: Path) -> list[tuple[float, float]]:
    text = path.read_text(encoding="utf-8-sig", errors="replace")
    intervals: list[tuple[float, float]] = []
    for start, end in TIMESTAMP_RE.findall(text):
        intervals.append((parse_timestamp(start), parse_timestamp(end)))
    return intervals


def parse_ass_intervals(path: Path) -> list[tuple[float, float]]:
    intervals: list[tuple[float, float]] = []
    for line in path.read_text(encoding="utf-8-sig", errors="replace").splitlines():
        if not line.startswith("Dialogue:"):
            continue
        parts = line.split(",", 9)
        if len(parts) < 10:
            continue
        intervals.append((parse_ass_time(parts[1]), parse_ass_time(parts[2])))
    return intervals


def parse_ass_time(value: str) -> float:
    hours, minutes, seconds = value.split(":")
    return int(hours) * 3600 + int(minutes) * 60 + float(seconds)


def score_overlap(
    target_intervals: list[tuple[float, float]],
    reference_intervals: list[tuple[float, float]],
    shift: float,
    step: float,
) -> int:
    end_time = max(
        max(end for _, end in target_intervals) + abs(shift),
        max(end for _, end in reference_intervals),
    )
    bins = int(end_time / step) + 3
    target_bins = bytearray(bins)
    reference_bins = bytearray(bins)

    for start, end in target_intervals:
        start += shift
        end += shift
        if end <= 0:
            continue
        start_index = max(0, int(start / step))
        end_index = min(bins - 1, int(end / step))
        for index in range(start_index, end_index + 1):
            target_bins[index] = 1

    for start, end in reference_intervals:
        start_index = max(0, int(start / step))
        end_index = min(bins - 1, int(end / step))
        for index in range(start_index, end_index + 1):
            reference_bins[index] = 1

    return sum(1 for left, right in zip(target_bins, reference_bins) if left and right)


def best_shift(
    target_intervals: list[tuple[float, float]],
    reference_intervals: list[tuple[float, float]],
    min_shift: float,
    max_shift: float,
    step: float,
    score_step: float,
) -> float:
    best_value: float | None = None
    best_score = -1
    current = min_shift
    while current <= max_shift + 1e-9:
        overlap = score_overlap(target_intervals, reference_intervals, current, score_step)
        if overlap > best_score:
            best_score = overlap
            best_value = current
        current = round(current + step, 10)
    if best_value is None:
        raise RuntimeError("Unable to calculate a timing shift")
    return best_value


def retime_text(text: str, shift: float) -> tuple[str, int]:
    def replace(match: re.Match[str]) -> str:
        start = parse_timestamp(match.group(1)) + shift
        end = parse_timestamp(match.group(2)) + shift
        if end < start:
            end = start
        return f"{format_timestamp(start)} --> {format_timestamp(end)}"

    return TIMESTAMP_RE.subn(replace, text)


def extract_episode_number(name: str, pattern: re.Pattern[str]) -> int | None:
    match = pattern.search(name)
    if not match:
        return None
    return int(match.group(1))


def command_convert_chrome(args: argparse.Namespace) -> int:
    ffmpeg = ensure_tool("ffmpeg")
    input_dir = Path(args.input_dir)
    output_dir = Path(args.output_dir) if args.output_dir else input_dir / "Chrome AAC"
    output_dir.mkdir(parents=True, exist_ok=True)

    mkv_files = iter_files(input_dir, args.pattern)
    if not mkv_files:
        raise SystemExit(f"No files matched {args.pattern!r} in {input_dir}")

    for source in mkv_files:
        dest = output_dir / source.name
        if dest.exists() and not args.overwrite:
            print(f"SKIP {dest.name}")
            continue
        ffmpeg_overwrite_flag = "-y" if args.overwrite else "-n"
        cmd = [
            ffmpeg,
            "-hide_banner",
            "-nostdin",
            ffmpeg_overwrite_flag,
            "-i",
            str(source),
            "-map",
            args.video_map,
            "-map",
            "0:a",
            "-map",
            "0:s?",
            "-map",
            "0:t?",
            "-map_metadata",
            "0",
            "-map_chapters",
            "0",
            "-c",
            "copy",
            "-c:a",
            "aac",
            "-b:a",
            args.audio_bitrate,
            str(dest),
        ]
        run(cmd)
    return 0


def command_download_jimaku(args: argparse.Namespace) -> int:
    entry_url = normalize_jimaku_entry(args.entry)
    output_dir = Path(args.output_dir)
    output_dir.mkdir(parents=True, exist_ok=True)

    if args.filter_regex:
        pattern = re.compile(args.filter_regex)
    else:
        pattern = PRESET_PATTERNS[args.preset]

    entry_html = fetch_text(entry_url)
    files = parse_jimaku_entry_files(entry_html)
    matches = [item for item in files if item["name"].endswith(".srt") and pattern.search(item["name"])]
    if not matches:
        raise SystemExit("No Jimaku subtitle files matched the requested filter")

    matches.sort(key=lambda item: extract_episode_number(item["name"], pattern) or 999)

    for item in matches:
        episode = extract_episode_number(item["name"], pattern)
        destination_name = args.rename_template.format(
            episode=episode if episode is not None else "",
            original_name=item["name"],
            stem=Path(item["name"]).stem,
        )
        destination = output_dir / destination_name
        if destination.exists() and not args.overwrite:
            print(f"SKIP {destination.name}")
            continue
        print(f"DOWNLOAD {item['name']} -> {destination.name}")
        download_file(item["url"], destination)
    return 0


def command_measure_offsets(args: argparse.Namespace) -> int:
    ffmpeg = ensure_tool("ffmpeg")
    media_dir = Path(args.media_dir)
    subtitle_dir = Path(args.subtitle_dir) if args.subtitle_dir else media_dir
    mkv_files = iter_files(media_dir, args.media_pattern)
    if not mkv_files:
        raise SystemExit(f"No files matched {args.media_pattern!r} in {media_dir}")

    if args.temp_dir:
        temp_root = Path(args.temp_dir)
        temp_root.mkdir(parents=True, exist_ok=True)
        temp_context = None
    else:
        temp_context = tempfile.TemporaryDirectory(prefix="anitools_ref_")
        temp_root = Path(temp_context.name)

    results: dict[str, float] = {}

    try:
        for mkv in mkv_files:
            external_subtitle = subtitle_dir / f"{mkv.stem}{args.subtitle_ext}"
            if not external_subtitle.exists():
                raise SystemExit(f"Missing external subtitle: {external_subtitle}")

            reference_subtitle = temp_root / f"{mkv.stem}.ass"
            cmd = [
                ffmpeg,
                "-hide_banner",
                "-nostdin",
                "-y",
                "-i",
                str(mkv),
                "-map",
                f"0:s:{args.reference_stream}",
                str(reference_subtitle),
            ]
            run(cmd)

            target_intervals = parse_srt_intervals(external_subtitle)
            reference_intervals = parse_ass_intervals(reference_subtitle)
            shift = best_shift(
                target_intervals=target_intervals,
                reference_intervals=reference_intervals,
                min_shift=args.min_shift,
                max_shift=args.max_shift,
                step=args.search_step,
                score_step=args.score_step,
            )
            results[mkv.stem] = shift
            print(f"{mkv.name}: {shift:+.2f}s")
    finally:
        if temp_context is not None:
            temp_context.cleanup()

    if args.output_json:
        output_json = Path(args.output_json)
        output_json.write_text(
            json.dumps(results, indent=2, ensure_ascii=False) + "\n",
            encoding="utf-8",
        )
        print(f"Wrote {output_json}")

    if results:
        shifts = list(results.values())
        print(
            f"Summary: median={median(shifts):+.2f}s min={min(shifts):+.2f}s max={max(shifts):+.2f}s"
        )
    return 0


def command_retime_srt(args: argparse.Namespace) -> int:
    subtitle_dir = Path(args.subtitle_dir)
    backup_dir = Path(args.backup_dir) if args.backup_dir else subtitle_dir / "_before_retime"
    backup_dir.mkdir(parents=True, exist_ok=True)

    offsets: dict[str, float] = {}
    if args.offsets_json:
        offsets = json.loads(Path(args.offsets_json).read_text(encoding="utf-8"))

    subtitle_files = iter_files(subtitle_dir, args.pattern)
    if not subtitle_files:
        raise SystemExit(f"No files matched {args.pattern!r} in {subtitle_dir}")

    for subtitle in subtitle_files:
        if args.shift is not None:
            shift = args.shift
        else:
            if subtitle.stem in offsets:
                shift = offsets[subtitle.stem]
            elif subtitle.name in offsets:
                shift = offsets[subtitle.name]
            else:
                raise SystemExit(f"No offset found for {subtitle.name}")

        backup_path = backup_dir / subtitle.name
        if not backup_path.exists():
            shutil.copy2(subtitle, backup_path)

        text = subtitle.read_text(encoding="utf-8-sig", errors="replace")
        new_text, count = retime_text(text, shift)
        subtitle.write_text(new_text, encoding="utf-8")
        print(f"{subtitle.name}: {shift:+.2f}s ({count} cues)")
    return 0


def command_assemble_player_ready(args: argparse.Namespace) -> int:
    output_dir = Path(args.output_dir)
    output_dir.mkdir(parents=True, exist_ok=True)

    if args.link_mode == "copy":
        copier = shutil.copy2
    else:
        copier = None

    for source_dir, pattern in [
        (Path(args.video_dir), args.video_pattern),
        (Path(args.subtitle_dir), args.subtitle_pattern),
    ]:
        for source in iter_files(source_dir, pattern):
            dest = output_dir / source.name
            if dest.exists():
                if not args.overwrite:
                    print(f"SKIP {dest.name}")
                    continue
                dest.unlink()
            print(f"ADD {source} -> {dest}")
            if args.link_mode == "copy":
                copier(source, dest)
            else:
                dest.hardlink_to(source)
    return 0


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        description="Anime utility commands for browser-safe conversion and subtitle work."
    )
    subparsers = parser.add_subparsers(dest="command", required=True)

    convert = subparsers.add_parser(
        "convert-chrome",
        help="Copy video/subtitles and re-encode audio to AAC for browser playback.",
    )
    convert.add_argument("--input-dir", required=True)
    convert.add_argument("--output-dir")
    convert.add_argument("--pattern", default="*.mkv")
    convert.add_argument("--audio-bitrate", default="192k")
    convert.add_argument("--video-map", default="0:v:0")
    convert.add_argument("--overwrite", action="store_true")
    convert.set_defaults(func=command_convert_chrome)

    download = subparsers.add_parser(
        "download-jimaku",
        help="Download a filtered SRT set from a Jimaku entry page.",
    )
    download.add_argument("--entry", required=True, help="Jimaku entry id or full entry URL.")
    download.add_argument("--output-dir", required=True)
    download.add_argument(
        "--preset",
        choices=sorted(PRESET_PATTERNS),
        default="webrip-netflix-ja-cc",
        help="Built-in file-name filter for a subtitle set.",
    )
    download.add_argument(
        "--filter-regex",
        help="Custom regex filter. Overrides --preset when provided.",
    )
    download.add_argument(
        "--rename-template",
        default="{original_name}",
        help="Python format string. Fields: episode, original_name, stem.",
    )
    download.add_argument("--overwrite", action="store_true")
    download.set_defaults(func=command_download_jimaku)

    measure = subparsers.add_parser(
        "measure-offsets",
        help="Estimate constant subtitle shifts by comparing external SRTs to MKV subtitle timing.",
    )
    measure.add_argument("--media-dir", required=True)
    measure.add_argument("--subtitle-dir")
    measure.add_argument("--media-pattern", default="*.mkv")
    measure.add_argument("--subtitle-ext", default=".srt")
    measure.add_argument("--reference-stream", type=int, default=0)
    measure.add_argument("--min-shift", type=float, default=-2.0)
    measure.add_argument("--max-shift", type=float, default=2.0)
    measure.add_argument("--search-step", type=float, default=0.05)
    measure.add_argument("--score-step", type=float, default=0.10)
    measure.add_argument("--temp-dir")
    measure.add_argument("--output-json")
    measure.set_defaults(func=command_measure_offsets)

    retime = subparsers.add_parser(
        "retime-srt",
        help="Apply a constant shift or JSON offsets map to SRT files, with backups.",
    )
    retime.add_argument("--subtitle-dir", required=True)
    retime.add_argument("--pattern", default="*.srt")
    retime.add_argument("--shift", type=float)
    retime.add_argument("--offsets-json")
    retime.add_argument("--backup-dir")
    retime.set_defaults(func=command_retime_srt)

    assemble = subparsers.add_parser(
        "assemble-player-ready",
        help="Gather videos and subtitles into one folder for a player.",
    )
    assemble.add_argument("--video-dir", required=True)
    assemble.add_argument("--subtitle-dir", required=True)
    assemble.add_argument("--output-dir", required=True)
    assemble.add_argument("--video-pattern", default="*.mkv")
    assemble.add_argument("--subtitle-pattern", default="*.srt")
    assemble.add_argument("--link-mode", choices=["copy", "hardlink"], default="copy")
    assemble.add_argument("--overwrite", action="store_true")
    assemble.set_defaults(func=command_assemble_player_ready)

    return parser


def main(argv: list[str] | None = None) -> int:
    parser = build_parser()
    args = parser.parse_args(argv)
    return args.func(args)


if __name__ == "__main__":
    sys.exit(main())
