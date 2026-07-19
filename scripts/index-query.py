#!/usr/bin/env python3
"""Read-only structural lookups over INDEX.yml. Never writes anything.

Usage:
  index-query.py titles
  index-query.py session "<Exact Concept Title>"
  index-query.py prereqs "<Exact Concept Title>"
  index-query.py uses "<Exact Concept Title>"
  index-query.py required-by "<Concept Title>"
  index-query.py future-targets
  index-query.py branches
  index-query.py open-gaps
  index-query.py concept <tag>
  index-query.py capability <tag>
  index-query.py by-date [YYYY-MM-DD]
"""
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from _repo import die, load_index  # noqa: E402


def cmd_titles(idx, args):
    for title in sorted(idx["sessions"]):
        print(title)


def cmd_session(idx, args):
    if not args:
        die("usage: session \"<Exact Concept Title>\"")
    title = args[0]
    s = idx["sessions"].get(title)
    if s is None:
        die(f'no session titled "{title}" (try: index-query.py titles)')
    for key, value in s.items():
        if isinstance(value, list):
            print(f"{key}:")
            for item in value:
                print(f"  - {item}")
        else:
            print(f"{key}: {value}")


def _field_list(idx, args, field):
    if not args:
        die(f'usage: {field} "<Exact Concept Title>"')
    title = args[0]
    s = idx["sessions"].get(title)
    if s is None:
        die(f'no session titled "{title}" (try: index-query.py titles)')
    val = s.get(field)
    if val in (None, "—"):
        print("—")
        return
    for item in val:
        print(item)


def cmd_prereqs(idx, args):
    _field_list(idx, args, "prerequisites")


def cmd_uses(idx, args):
    _field_list(idx, args, "uses_concepts")


def cmd_required_by(idx, args):
    if not args:
        die('usage: required-by "<Concept Title>"')
    concept = args[0]
    entry = (idx.get("prerequisite_index") or {}).get(concept)
    if entry is None:
        die(f'no prerequisite_index entry for "{concept}"')
    for title in entry.get("required_by") or []:
        print(title)


def cmd_future_targets(idx, args):
    targets = idx.get("future_targets") or {}
    for title, info in targets.items():
        mentioned_by = ", ".join(info.get("mentioned_by") or []) or "—"
        print(f"{title}  [status: {info.get('status')}, mentioned_by: {mentioned_by}]")


def cmd_branches(idx, args):
    for name, info in (idx.get("branches") or {}).items():
        frontier = ", ".join(info.get("frontier") or []) or "—"
        print(f"{name}")
        print(f"  frontier: {frontier}")


def cmd_open_gaps(idx, args):
    gaps = idx.get("open_gaps") or {}
    for section, value in gaps.items():
        print(f"{section}:")
        if value in (None, "—"):
            print("  —")
        elif isinstance(value, list):
            for item in value:
                if isinstance(item, dict):
                    print(f"  - {item.get('claim')}  [{item.get('status')}]")
                else:
                    print(f"  - {item}")
        else:
            print(f"  {value}")


def _index_lookup(idx, args, section):
    if not args:
        die(f"usage: {section} <tag>")
    tag = args[0]
    entries = (idx.get(section) or {}).get(tag)
    if entries is None:
        die(f'no "{tag}" entry in {section}')
    for title in entries:
        print(title)


def cmd_concept(idx, args):
    _index_lookup(idx, args, "concept_index")


def cmd_capability(idx, args):
    _index_lookup(idx, args, "capability_index")


def cmd_by_date(idx, args):
    by_date = idx.get("completed_by_date") or {}
    if args:
        titles = by_date.get(args[0])
        if titles is None:
            die(f'no sessions completed on "{args[0]}"')
        for title in titles:
            print(title)
        return
    for date in sorted(by_date, reverse=True):
        for title in by_date[date]:
            print(f"{date}  {title}")


COMMANDS = {
    "titles": cmd_titles,
    "session": cmd_session,
    "prereqs": cmd_prereqs,
    "uses": cmd_uses,
    "required-by": cmd_required_by,
    "future-targets": cmd_future_targets,
    "branches": cmd_branches,
    "open-gaps": cmd_open_gaps,
    "concept": cmd_concept,
    "capability": cmd_capability,
    "by-date": cmd_by_date,
}


def main():
    if len(sys.argv) < 2 or sys.argv[1] not in COMMANDS:
        print(__doc__)
        sys.exit(0 if len(sys.argv) < 2 else 1)
    idx = load_index()
    COMMANDS[sys.argv[1]](idx, sys.argv[2:])


if __name__ == "__main__":
    main()
