Verify that the most recently written notes file and history entry conform to every rule in CLAUDE.md. Report each violation specifically — quote the offending text and cite the rule it breaks.

## What to check

**Identify the files.** Most recently modified `src/bin/*.md` notes file and its corresponding entry in the appropriate `HISTORY-YYYY.MM.yml`.

**Notes file:**

1. **Section order.** `# Title`, `## Overview`, bespoke theory sections (any names), `## Correctness`, `## Complexity`, `## Edge cases` (optional), `## Worked example`. Flag missing/out-of-order/forbidden sections (`## Depends on`, `## Unlocks`).
2. **Prose only.** No bullets/numbered lists anywhere, including the worked example. Flag any `-`, `*`, `1.` list markers.
3. **Math formatting.** All math in `$$...$$` on its own line. Flag inline math (backtick math, parenthetical formulas, `$...$`) and divisibility/similar relations written as inline symbols (`p | n`) rather than spelled out or in a `$$` block.
4. **Formula framing.** Every `$$` block needs a preceding sentence (why) and a following sentence (what it means). Flag any missing either.
5. **Paragraph length.** Flag standalone one-sentence paragraphs not attached to a neighbor.
6. **Word count.** `wc -w`. Flag if outside 700–1500 without an explicit justification in Overview or Complexity.
7. **Worked example.** Non-trivial (not degenerate), mentally verifiable under a minute, entirely prose (no bullet trace).
8. **Citation vs. re-derivation.** A concept covered in a prior session's notes must be cited by filename, not re-derived. Flag any full re-derivation of a previously-covered result.
9. **No personal data.** Flag names, emails, or other personal information.

**History entry:**

1. **Placement.** Top (index 0) of the correct monthly file's `entries`. Flag if not.
2. **Schema completeness.** `date`, `title`, `session` block with every fixed field present: `file`, `source`, `status`, `attempted`, `explored`, `tried`, `corrections`, `bugs_found`, `completed`, `not_completed`, `open_questions`, `notes`. Flag any missing field, even when empty.
3. **Empty-field convention.** Empty = exactly `—`, never an empty list, never omitted. Flag any list mixing `—` with real entries, and flag `bugs_found` not `—` on a book-study entry (no code, no defects possible).
4. **No `Depends on`/`Unlocks`.** Flag any such field or resemblance anywhere — belongs solely in INDEX.yml.
5. **No mathematical exposition.** Flag any full algorithm description, proof, theorem statement, or complexity derivation (belongs in notes file). A short factual reference naming a result ("Compared observed periods against the Hull-Dobell conditions") is fine; reproducing the result is not.
6. **No personal/psychological content.** Flag personal info, personality description, psychological interpretation, inferred learning style, subjective judgment of intelligence/ability/motivation/behavior. `corrections` must be factual before/after about an assumption, never an evaluation of the person.
7. **Field discipline.** Spot-check: `completed` items are observable outcomes, not vague ("understood X," "gained insight"); `not_completed` isn't silently empty when the session mentions deferred work; `open_questions` isn't populated with an invented "natural next step" that wasn't actually raised.
8. **No retrospective edits to older entries.** Flag any *other* (older) HISTORY entry modified as part of this close, unless it's one of the immutability-rule exceptions (factual error, formatting, filename/date correction, genuinely-omitted same-session item). A change for a newly-discovered dependency/reuse/future-target is a violation — belongs in INDEX.yml instead.

**INDEX.yml — consistency and incremental-update discipline:**

0. **Run `scripts/index-validate.py` first.** Any reported problem is itself a violation to flag — quote its exact output line rather than re-deriving the same check by hand.
1. **New entry present and correctly classified.** New session's Concept Title is a key under `sessions`, with `kind`, `file`/`source`, `date`, and the full relationship field set (`prerequisites`, `uses_concepts`, `reuses_code`, `derived_from`, `related_to`, `unlocks`, `future_targets`, `summary`, `concepts`, `capabilities`). Verify each relationship was classified honestly against CLAUDE.md's "Fast index (INDEX.yml)" tests, comparing against existing INDEX.yml sessions (not re-derived from a HISTORY `Depends on`/`Unlocks` field — none exists) — e.g. flag a `prerequisites` entry that's really just chronology, or a harder algorithm listed as prerequisite of a simpler one.
2. **`reuses_code` is `—`.** Every session's, no exceptions — this repo forbids cross-file code reuse. Non-empty here is a violation.
3. **`prerequisite_index` updated.** For every concept in this session's `prerequisites`, its title appears under that concept's `required_by`, and nowhere it wasn't genuinely a prerequisite.
4. **`concept_index`/`capability_index` updated.** Every tag in this session's `concepts`/`capabilities` maps back to it in the respective index.
5. **`completed_by_date` updated.** This session's title appears exactly once, under the correct date, nowhere else.
6. **`future_targets` resolved structurally.** If this session's own title was previously listed there, it's now removed (can't be both a completed session and a future target). Flag any bare string flag ("(not yet its own session)") baked into an identifier — that belongs in `future_targets`' `status: not-completed` field.
7. **No unwarranted full re-derivation.** Confirm the update looks incremental — an already-completed older session's fields should be untouched unless the write agent gave a specific, stated reason this new session's evidence implicates that older classification. Flag if many older sessions changed with no stated justification, or if the write agent's own report describes re-scanning all sessions from scratch rather than patching just the new one.
8. **Retrospective revisions reflected everywhere, and only in INDEX.yml.** If an earlier session's `prerequisites`/`derived_from`/`unlocks`/`related_to` was revised, confirm it's consistent on both sides within INDEX.yml (the field itself AND its appearances in `prerequisite_index`/other reverse indexes) — a fix in only one place is itself a violation. Confirm it was **not** mirrored into that session's HISTORY entry.

Flag as a violation any of the above left stale or inconsistent.

## Output format

List each violation as: **[File:Line] Rule violated — quoted offending text**. If none found, say so explicitly and give the notes file's word count.
