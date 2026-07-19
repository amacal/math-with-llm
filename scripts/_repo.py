"""Shared helpers for the scripts/ tools. Read-only unless a script explicitly does otherwise."""
import glob
import os
import sys

import yaml

REPO_ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
INDEX_PATH = os.path.join(REPO_ROOT, "INDEX.yml")


def load_index(path=INDEX_PATH):
    with open(path) as f:
        return yaml.safe_load(f)


def history_files():
    """All HISTORY-YYYY.MM.yml paths, chronological (filenames sort as plain strings, per CLAUDE.md)."""
    return sorted(glob.glob(os.path.join(REPO_ROOT, "HISTORY-*.yml")))


def load_history_entries():
    """Yield (entry_dict, source_filename) across every HISTORY file, in file order (newest file last,
    but each file's own entries list is already newest-first per the append convention)."""
    for path in history_files():
        with open(path) as f:
            doc = yaml.safe_load(f)
        fname = os.path.basename(path)
        for entry in (doc.get("entries") or []):
            # An unquoted `date: 2026-07-19` is valid YAML timestamp syntax, so PyYAML hands
            # back a datetime.date rather than a string — normalize so string comparisons work.
            if "date" in entry and not isinstance(entry["date"], str):
                entry["date"] = entry["date"].isoformat()
            yield entry, fname


def display_scalar(value):
    """A HISTORY/INDEX list item that contains an unescaped colon parses as a single-key dict
    instead of a string (a latent formatting issue in some existing entries). Reassemble it so
    query tools can still show the intended text instead of crashing or printing {...}."""
    if isinstance(value, dict) and len(value) == 1:
        (k, v) = next(iter(value.items()))
        return f"{k}: {v}"
    return str(value)


def die(message):
    print(f"error: {message}", file=sys.stderr)
    sys.exit(1)
