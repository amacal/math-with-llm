#!/usr/bin/env python3
"""Append one new HISTORY entry from a small YAML fragment file, via targeted text
splicing — it never parses+rewrites the whole target file, so every byte outside the
new insertion (and, on month rollover, the single `next:` line it backfills) is left
untouched. This is the one INDEX/HISTORY writer CLAUDE.md's tooling proposal endorses;
INDEX.yml itself is still patched by hand (see scripts/index-validate.py instead).

Fragment file shape (exactly these keys, in this order):
  date: YYYY-MM-DD
  title: "Exact Concept Title"
  session:
    file: src/bin/foo.rs   # or — for book sessions
    source: —               # or "Book Title, Chapter N, Section N.N"
    status: completed
    attempted: [...]        # each list field is either the string — or a non-empty list
    explored: [...]
    tried: [...]
    corrections: [...]
    bugs_found: [...]       # always — for book sessions
    completed: [...]
    not_completed: [...]
    open_questions: [...]
    notes: [...]

Usage:
  history-append.py <fragment.yml> [--dry-run]
"""
import os
import re
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from _repo import REPO_ROOT, die, display_scalar  # noqa: E402

import yaml  # noqa: E402

SESSION_FIELDS = [
    "file", "source", "status", "attempted", "explored", "tried", "corrections",
    "bugs_found", "completed", "not_completed", "open_questions", "notes",
]
LIST_FIELDS = [f for f in SESSION_FIELDS if f not in ("file", "source", "status")]
DATE_RE = re.compile(r"^\d{4}-\d{2}-\d{2}$")


def quote_if_needed(text):
    text = str(text)
    special_lead = text[:1] in list("-?:,[]{}#&*!|>'\"%@`") if text else True
    needs_quote = (
        text == ""
        or special_lead
        or ": " in text
        or text != text.strip()
        or text.strip().lower() in ("true", "false", "null", "~", "yes", "no")
        or re.match(r"^-?\d+(\.\d+)?$", text.strip())
    )
    if needs_quote:
        return '"' + text.replace("\\", "\\\\").replace('"', '\\"') + '"'
    return text


def normalize_fragment(frag):
    """A bare `date: 2026-07-19` parses as datetime.date (valid YAML timestamp syntax), and a
    bare list item containing ": " parses as a single-key dict instead of a string — the same
    two PyYAML quirks discovered in the existing HISTORY files. Recover the intended value in
    both cases rather than silently miscounting or hard-failing on an easy authoring mistake."""
    if isinstance(frag.get("date"), object) and not isinstance(frag.get("date"), str):
        try:
            frag["date"] = frag["date"].isoformat()
        except AttributeError:
            pass
    session = frag.get("session")
    if isinstance(session, dict):
        for field in LIST_FIELDS:
            val = session.get(field)
            if isinstance(val, list):
                session[field] = [display_scalar(item) for item in val]
    return frag


def validate_fragment(frag):
    problems = []
    top_keys = set(frag.keys())
    if top_keys != {"date", "title", "session"}:
        problems.append(f"top-level keys must be exactly date/title/session, got {sorted(top_keys)}")
        return problems

    date = frag.get("date")
    if not isinstance(date, str) or not DATE_RE.match(date):
        problems.append(f'date must be "YYYY-MM-DD", got {date!r}')

    title = frag.get("title")
    if not isinstance(title, str) or not title.strip():
        problems.append(f"title must be a non-empty string, got {title!r}")

    session = frag.get("session")
    if not isinstance(session, dict):
        problems.append("session must be a mapping")
        return problems

    session_keys = list(session.keys())
    if session_keys != SESSION_FIELDS:
        problems.append(
            f"session keys must be exactly {SESSION_FIELDS} in that order, got {session_keys}"
        )

    for field in LIST_FIELDS:
        if field not in session:
            continue
        val = session[field]
        if val == "—":
            continue
        if not isinstance(val, list) or not val:
            problems.append(
                f'session.{field} must be the string "—" or a non-empty list, got {val!r}'
            )
        elif isinstance(val, list) and any(not isinstance(x, str) for x in val):
            problems.append(f"session.{field} list items must all be strings")

    return problems


def format_entry(frag):
    lines = []
    lines.append(f"  - date: {frag['date']}")
    lines.append(f'    title: "{frag["title"].strip()}"')
    lines.append("    session:")
    session = frag["session"]
    for field in ("file", "source", "status"):
        lines.append(f"      {field}: {session[field]}")
    for field in LIST_FIELDS:
        val = session[field]
        if val == "—":
            lines.append(f"      {field}: —")
        else:
            lines.append(f"      {field}:")
            for item in val:
                lines.append(f"        - {quote_if_needed(item)}")
    return lines


def new_month_header(month_str, filename, previous_filename):
    prev = previous_filename if previous_filename else "—"
    return [
        f"# {filename} — chronological, factual session-event log for {month_str}.",
        "# See CLAUDE.md's \"Session history\" section for the schema and field semantics.",
        "# This file is evidence of WHAT HAPPENED during each session, not a math reference and not a",
        "# relationship graph. Mathematical detail lives in src/bin/*.md; prerequisites/relationships live",
        "# in INDEX.yml. Entries are append-oriented: see CLAUDE.md's \"Historical immutability rule\" before",
        "# editing an existing entry.",
        f'month: "{month_str}"',
        f"previous: {prev}",
        "next: —",
        "entries:",
    ]


def existing_history_filenames():
    return sorted(
        f for f in os.listdir(REPO_ROOT)
        if re.match(r"^HISTORY-\d{4}\.\d{2}\.yml$", f)
    )


def main():
    args = [a for a in sys.argv[1:] if a != "--dry-run"]
    dry_run = "--dry-run" in sys.argv
    if len(args) != 1:
        print(__doc__)
        sys.exit(1)

    with open(args[0]) as f:
        frag = yaml.safe_load(f)
    frag = normalize_fragment(frag)

    problems = validate_fragment(frag)
    if problems:
        for p in problems:
            print(f"error: {p}", file=sys.stderr)
        sys.exit(1)

    date = frag["date"]
    month_str = date[:7]                       # "YYYY-MM"
    month_file_stem = date[:7].replace("-", ".")  # "YYYY.MM"
    target_name = f"HISTORY-{month_file_stem}.yml"
    target_path = os.path.join(REPO_ROOT, target_name)
    entry_lines = format_entry(frag)

    if os.path.exists(target_path):
        with open(target_path) as f:
            content = f.read()
        marker = "\nentries:\n"
        idx = content.find(marker)
        if idx == -1 and content.startswith("entries:\n"):
            idx = -len(marker) + 1  # entries: is the very first line, no leading blank
        if content.find("entries:\n") == -1:
            die(f'{target_name}: no top-level "entries:" line found — refusing to guess where to splice')
        split_at = content.find("entries:\n") + len("entries:\n")
        new_content = content[:split_at] + "\n".join(entry_lines) + "\n" + content[split_at:]

        if dry_run:
            print(f"--- would splice into existing {target_name} right after 'entries:' ---")
            print("\n".join(entry_lines))
            return

        with open(target_path, "w") as f:
            f.write(new_content)
        print(f"appended to {target_name} ({len(entry_lines)} lines spliced after 'entries:')")
        return

    # New month: create the file and backfill the previous file's `next:` line.
    existing = existing_history_filenames()
    previous_filename = existing[-1] if existing else None

    header = new_month_header(month_str, target_name, previous_filename)
    new_content = "\n".join(header) + "\n" + "\n".join(entry_lines) + "\n"

    if dry_run:
        print(f"--- would create new file {target_name} ---")
        print(new_content, end="")
        if previous_filename:
            print(f"--- would backfill {previous_filename}'s 'next: —' -> 'next: {target_name}' ---")
        return

    if previous_filename:
        prev_path = os.path.join(REPO_ROOT, previous_filename)
        with open(prev_path) as f:
            prev_content = f.read()
        if "\nnext: —\n" not in ("\n" + prev_content):
            die(
                f"{previous_filename} does not have a bare 'next: —' line to backfill — "
                f"refusing to overwrite something already set"
            )
        patched = ("\n" + prev_content).replace("\nnext: —\n", f"\nnext: {target_name}\n", 1)[1:]
        with open(prev_path, "w") as f:
            f.write(patched)
        print(f"backfilled {previous_filename}: next -> {target_name}")

    with open(target_path, "w") as f:
        f.write(new_content)
    print(f"created {target_name} (previous: {previous_filename or '—'})")


if __name__ == "__main__":
    main()
