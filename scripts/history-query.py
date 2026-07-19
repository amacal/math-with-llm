#!/usr/bin/env python3
"""Read-only lookups across every HISTORY-YYYY.MM.yml. Never writes anything.

Usage:
  history-query.py files
  history-query.py recent <N>
  history-query.py by-file <src/bin/path.rs>
  history-query.py by-title <substring>
  history-query.py since <YYYY-MM-DD>
  history-query.py grep <pattern>
"""
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from _repo import die, display_scalar, history_files, load_history_entries  # noqa: E402


def _print_entry(entry, fname):
    file_ = entry.get("session", {}).get("file", "—")
    print(f"{entry.get('date')}  {entry.get('title')}  [{fname}, {file_}]")


def cmd_files(args):
    for path in history_files():
        print(os.path.basename(path))


def cmd_recent(args):
    n = int(args[0]) if args else 5
    entries = list(load_history_entries())
    entries.sort(key=lambda pair: pair[0].get("date", ""), reverse=True)
    for entry, fname in entries[:n]:
        _print_entry(entry, fname)


def cmd_by_file(args):
    if not args:
        die("usage: by-file <src/bin/path.rs>")
    target = args[0]
    found = False
    for entry, fname in load_history_entries():
        if entry.get("session", {}).get("file") == target:
            _print_entry(entry, fname)
            found = True
    if not found:
        die(f'no HISTORY entry with session.file == "{target}"')


def cmd_by_title(args):
    if not args:
        die("usage: by-title <substring>")
    needle = args[0].lower()
    found = False
    for entry, fname in load_history_entries():
        if needle in (entry.get("title") or "").lower():
            _print_entry(entry, fname)
            found = True
    if not found:
        die(f'no HISTORY entry with title containing "{args[0]}"')


def cmd_since(args):
    if not args:
        die("usage: since <YYYY-MM-DD>")
    cutoff = args[0]
    entries = [(e, f) for e, f in load_history_entries() if (e.get("date") or "") >= cutoff]
    entries.sort(key=lambda pair: pair[0].get("date", ""), reverse=True)
    for entry, fname in entries:
        _print_entry(entry, fname)


def _flatten_text(value):
    if isinstance(value, list):
        return " ".join(display_scalar(v) for v in value)
    if value in (None, "—"):
        return ""
    return display_scalar(value)


def cmd_grep(args):
    if not args:
        die("usage: grep <pattern>")
    needle = args[0].lower()
    found = False
    for entry, fname in load_history_entries():
        session = entry.get("session") or {}
        haystacks = {
            "title": entry.get("title") or "",
        }
        for field in ("attempted", "explored", "tried", "corrections", "bugs_found",
                      "completed", "not_completed", "open_questions", "notes"):
            haystacks[field] = _flatten_text(session.get(field))
        hits = [field for field, text in haystacks.items() if needle in text.lower()]
        if hits:
            _print_entry(entry, fname)
            print(f"    matched in: {', '.join(hits)}")
            found = True
    if not found:
        die(f'no HISTORY entry matches "{args[0]}"')


COMMANDS = {
    "files": cmd_files,
    "recent": cmd_recent,
    "by-file": cmd_by_file,
    "by-title": cmd_by_title,
    "since": cmd_since,
    "grep": cmd_grep,
}


def main():
    if len(sys.argv) < 2 or sys.argv[1] not in COMMANDS:
        print(__doc__)
        sys.exit(0 if len(sys.argv) < 2 else 1)
    COMMANDS[sys.argv[1]](sys.argv[2:])


if __name__ == "__main__":
    main()
