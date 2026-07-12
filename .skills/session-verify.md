Verify that the most recently written notes file and history entry conform to every rule in CLAUDE.md. Report each violation specifically — quote the offending text and cite the rule it breaks.

## What to check

**Identify the files.** Find the most recently modified `src/bin/*.md` notes file and its corresponding entry in the appropriate `HISTORY-YYYY.MM.md`.

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

1. **Placement.** The entry must be at the top of the entries section (most recent first). Flag if it is not.

2. **Template completeness.** Must have: date heading, `**File:**` or `**Source:**` line, one paragraph, `**Depends on:**`, `**Unlocks:**`. Flag any missing field.

3. **Paragraph length.** 5–8 sentences. Flag if shorter or longer.

4. **No display math.** History entries must use plain ASCII math only (`a^k mod n`, not `$$a^k \pmod n$$`). Flag any `$$` blocks.

5. **No labels.** Paragraph must be flowing prose with no "Key result:", "Key insight:", or similar label prefixes. Flag any such labels.

6. **Unlocks backfill.** Check whether any prior history entry whose concept is listed in this entry's `Depends on` field has a blank or stale `Unlocks` field that should now name this concept. Flag any that need updating.

**INDEX.txt — check consistency:**

1. **New entry present.** The new session's Concept Title must appear as a key under `sessions`, with matching file-or-source, date, depends_on, and unlocks lists (bare concept titles, no parenthetical explanations).

2. **depends_on_index updated.** For every concept in this session's Depends-on list, this session's Concept Title must appear in that concept's list in `depends_on_index`.

3. **unlocks_index updated.** For every concept in this session's Unlocks list, this session's Concept Title must appear in that concept's list in `unlocks_index`.

4. **Resolved flags removed.** If this session's own Concept Title was previously flagged "(not yet its own session)" anywhere in `unlocks_index`, that flag must now be gone.

5. **Backfill reflected on BOTH sides.** If step 6 above (history Unlocks backfill) found a stale field that needed updating, confirm it was fixed in TWO places, not one: the backfilled concept's own `unlocks` list under `sessions`, AND its appearances in `unlocks_index`. A fix applied only to `unlocks_index` while the `sessions` entry still shows the old/stale text is itself a violation — check both explicitly, don't just spot-check one.

Flag as a violation any of the above left stale or inconsistent.

## Output format

List each violation as: **[File:Line] Rule violated — quoted offending text**. If no violations are found, say so explicitly and give the word count of the notes file.
