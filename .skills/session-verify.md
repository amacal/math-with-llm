Verify that the most recently written notes file and history entry conform to every rule in CLAUDE.md. Report each violation specifically — quote the offending text and cite the rule it breaks.

## What to check

**Identify the files.** Find the most recently modified `src/bin/*.md` notes file and its corresponding entry in the appropriate `HISTORY-YYYY.MM.yml`.

**Notes file — check each rule:**

1. **Section order.** Sections must appear in this order: `# Title`, `## Overview`, bespoke theory sections (any names), `## Correctness`, `## Complexity`, `## Edge cases` (optional), `## Worked example`. Flag any missing required section, any section appearing out of order, and any forbidden sections (`## Depends on`, `## Unlocks`).

2. **Prose only.** No bullet points or numbered lists anywhere in the file, including the worked example. Flag any `-`, `*`, or `1.` list markers.

3. **Math formatting.** All math must be in `$$...$$` on its own display line. Flag any inline math (backtick math, parenthetical formulas, `$...$`). Flag any divisibility or similar relations written as inline symbols (e.g. `p | n`) rather than spelled out in prose or placed in a `$$` block.

4. **Formula framing.** Every `$$` block must be preceded by a sentence explaining why it is about to appear, and followed by a sentence explaining what it means. Flag any `$$` block missing either neighbor.

5. **Paragraph length.** Flag one-sentence paragraphs that stand alone rather than being attached to a neighboring paragraph.

6. **Word count.** Run `wc -w` on the file. Flag if it falls outside 700–1500 words without an explicit justification statement in the Overview or Complexity section.

7. **Worked example.** Verify it is non-trivial (not a degenerate edge case), small enough to verify mentally in under a minute, and written entirely in prose (no step-by-step bullet trace).

8. **Citation vs. re-derivation.** If a concept was covered in a prior session's notes file, verify it is cited by filename rather than re-derived from scratch. Flag any full re-derivation of a previously covered result.

9. **No personal data.** Flag any names, email addresses, or other personal information.

**History entry — check each rule:**

1. **Placement.** The entry must be at the top (index 0) of the correct monthly file's `entries` list (most recent first). Flag if it is not.

2. **Schema completeness.** Must have `date`, `title`, and a `session` block with every fixed field present: `file`, `source`, `status`, `attempted`, `explored`, `tried`, `corrections`, `bugs_found`, `completed`, `not_completed`, `open_questions`, `notes`. Flag any missing field — every field must be present even when empty.

3. **Empty-field convention.** Every empty field must be exactly `—`, never an empty list and never omitted. Flag any list that mixes `—` with real entries, and flag `bugs_found` if it isn't `—` on a book-study entry (no code, so no software defects are possible).

4. **No `Depends on` / `Unlocks`.** Flag any such field, or anything resembling one, anywhere in the entry — that structure belongs solely in `INDEX.yml`.

5. **No mathematical exposition.** Flag any full algorithm description, proof, theorem statement, or complexity derivation inside the entry — that belongs in the notes file. A short factual reference naming a result (e.g. "Compared observed periods against the Hull-Dobell conditions") is fine; reproducing the result is not.

6. **No personal/psychological content.** Flag any personal information, personality description, psychological interpretation, inferred learning style, or subjective judgment of intelligence, ability, motivation, or behavior. `corrections` entries must be factual before/after statements about an assumption, never an evaluation of the person who held it.

7. **Field discipline.** Spot-check that `completed` items are observable outcomes, not vague statements like "understood X" or "gained insight"; that `not_completed` isn't silently empty when the session prose/notes mention deferred work; and that `open_questions` isn't populated with an invented "natural next step" that wasn't actually raised.

8. **No retrospective edits to older entries.** If any *other* (older) HISTORY entry was modified as part of this session's close, flag it unless the change is one of the "Historical immutability rule" exceptions (factual error, formatting, filename/date correction, or a genuinely-omitted same-session item) — a change made because of a new dependency, reuse, or future target discovered later is a violation and belongs in `INDEX.yml` instead.

**INDEX.yml — check consistency:**

1. **New entry present and correctly classified.** The new session's Concept Title must appear as a key under `sessions`, with `kind`, `file`/`source`, `date`, and the full relationship field set (`prerequisites`, `uses_concepts`, `reuses_code`, `derived_from`, `related_to`, `unlocks`, `future_targets`, `summary`, `concepts`, `capabilities`). Since HISTORY no longer carries a `Depends on`/`Unlocks` field to copy from, verify each relationship was classified against the stricter tests in CLAUDE.md's "Fast index (INDEX.yml)" section using `src/bin/*.md`/`*.rs` as primary evidence — e.g. flag a `prerequisites` entry that is really just chronology, or a harder algorithm listed as a prerequisite of a simpler one.

2. **reuses_code is `—`.** Every session's `reuses_code` field must be `—`, per the repo's no-cross-file-code-reuse constraint. A non-empty value here is a violation of that constraint, not a legitimate index entry.

3. **prerequisite_index updated.** For every concept in this session's `prerequisites` list, this session's Concept Title must appear under that concept's `required_by` list in `prerequisite_index`, and nowhere else that wasn't genuinely a prerequisite.

4. **concept_index / capability_index updated.** Every tag in this session's `concepts` and `capabilities` lists must map back to this session in `concept_index` / `capability_index` respectively.

5. **completed_by_date updated.** This session's Concept Title must appear exactly once, under the correct date key, and nowhere else.

6. **future_targets resolved structurally.** If this session's own Concept Title was previously listed under the canonical `future_targets:` section, it must now be removed from there (a topic cannot be both a completed session and a future target). Flag any bare string flag like "(not yet its own session)" baked into an identifier anywhere in the file — that information must live in `future_targets`' `status: not-completed` field instead.

7. **Retrospective revisions reflected everywhere, and only in INDEX.yml.** If session-close's step 5 revised an earlier session's `prerequisites`/`derived_from`/`unlocks`/`related_to` fields, confirm the change is consistent on both sides within INDEX.yml (the earlier session's own field AND its appearances in `prerequisite_index`/other reverse indexes) — a fix applied in only one place while another still shows stale data is itself a violation. Confirm that same revision was **not** mirrored into the earlier session's HISTORY entry (see "No retrospective edits to older entries" above).

Flag as a violation any of the above left stale or inconsistent.

## Output format

List each violation as: **[File:Line] Rule violated — quoted offending text**. If no violations are found, say so explicitly and give the word count of the notes file.
