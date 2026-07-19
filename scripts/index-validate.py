#!/usr/bin/env python3
"""Mechanical structural checks over INDEX.yml — the four checks CLAUDE.md's
"Fast index (INDEX.yml)" section step 4 already asks for. Read-only, never writes.
Exit code 0 if every check passes, 1 if any fails.

Usage: index-validate.py [path-to-INDEX.yml]
"""
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from _repo import load_index  # noqa: E402

BACKWARD_FIELDS = ["prerequisites", "uses_concepts", "derived_from", "related_to"]
FORWARD_FIELDS = ["unlocks", "future_targets"]


def as_list_or_none(value, problems, where):
    if value in (None, "—"):
        return []
    if not isinstance(value, list):
        problems.append(f'{where}: expected a list or "—", got {value!r}')
        return []
    return value


def check_forward_references_resolve(sessions, problems):
    """prerequisites/uses_concepts/derived_from/related_to always point at completed sessions."""
    for title, s in sessions.items():
        for field in BACKWARD_FIELDS:
            for ref in as_list_or_none(s.get(field), problems, f'{title}.{field}'):
                if ref not in sessions:
                    problems.append(f'{title}.{field} references unknown session "{ref}"')


def check_unlocks_resolve(sessions, global_future, problems):
    """unlocks/future_targets (per-session) point at either a real session or a global future_targets entry."""
    for title, s in sessions.items():
        for field in FORWARD_FIELDS:
            for ref in as_list_or_none(s.get(field), problems, f'{title}.{field}'):
                if ref not in sessions and ref not in global_future:
                    problems.append(
                        f'{title}.{field} references "{ref}", which is neither a completed '
                        f'session nor a global future_targets entry'
                    )


def check_future_target_not_completed(sessions, global_future, problems):
    for ft in global_future:
        if ft in sessions:
            problems.append(f'future_targets contains "{ft}", but it is already a completed session')


def check_reverse_index(sessions, index, forward_field, index_name, problems, unwrap_required_by=False):
    """Bidirectional check between a session's forward field and a top-level reverse index."""
    for key, entry in index.items():
        titles = entry.get("required_by", []) if unwrap_required_by else entry
        for title in titles or []:
            if title not in sessions:
                problems.append(f'{index_name}["{key}"] references unknown session "{title}"')
                continue
            forward = as_list_or_none(sessions[title].get(forward_field), problems, f'{title}.{forward_field}')
            if key not in forward:
                problems.append(
                    f'{index_name}["{key}"] lists "{title}", but {title}.{forward_field} '
                    f'does not include "{key}"'
                )
    for title, s in sessions.items():
        for key in as_list_or_none(s.get(forward_field), problems, f'{title}.{forward_field}'):
            entry = index.get(key)
            titles = (entry.get("required_by", []) if unwrap_required_by else entry) if entry else []
            if title not in (titles or []):
                problems.append(
                    f'{title}.{forward_field} lists "{key}", but {index_name}["{key}"] '
                    f'does not list "{title}" back'
                )


def check_completed_by_date(sessions, by_date, problems):
    seen = {}
    for date, titles in by_date.items():
        for title in titles:
            seen[title] = seen.get(title, 0) + 1
            if title not in sessions:
                problems.append(f'completed_by_date["{date}"] references unknown session "{title}"')
    for title, s in sessions.items():
        count = seen.get(title, 0)
        if count == 0:
            problems.append(f'"{title}" does not appear anywhere in completed_by_date')
        elif count > 1:
            problems.append(f'"{title}" appears {count} times in completed_by_date (should be exactly once)')


def main():
    idx = load_index(sys.argv[1]) if len(sys.argv) > 1 else load_index()
    sessions = idx.get("sessions") or {}
    global_future = idx.get("future_targets") or {}
    problems = []

    check_forward_references_resolve(sessions, problems)
    check_unlocks_resolve(sessions, global_future, problems)
    check_future_target_not_completed(sessions, global_future, problems)
    check_reverse_index(
        sessions, idx.get("prerequisite_index") or {}, "prerequisites",
        "prerequisite_index", problems, unwrap_required_by=True,
    )
    check_reverse_index(
        sessions, idx.get("concept_index") or {}, "concepts",
        "concept_index", problems,
    )
    check_reverse_index(
        sessions, idx.get("capability_index") or {}, "capabilities",
        "capability_index", problems,
    )
    check_completed_by_date(sessions, idx.get("completed_by_date") or {}, problems)

    print(f"{len(sessions)} sessions checked")
    if not problems:
        print("[ok] all structural checks passed")
        sys.exit(0)

    print(f"[FAIL] {len(problems)} problem(s) found:")
    for p in problems:
        print(f"  - {p}")
    sys.exit(1)


if __name__ == "__main__":
    main()
