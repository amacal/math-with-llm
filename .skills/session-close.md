Write the notes file and session history entry for the session that just ended. Follow every CLAUDE.md rule exactly.

## Steps

1. **Identify the file.** Find the `src/bin/*.rs` file created/primarily worked on this session. Book-reading session (no `.rs`) → identify the source instead.

2. **Write the notes file** at `src/bin/{filename}.md` (same base name). Follow CLAUDE.md's "Notes writing style" exactly:
   - Section order: `# Title`, `## Overview`, zero+ bespoke theory/derivation sections, `## Correctness`, `## Complexity`, `## Edge cases` (if applicable), `## Worked example`.
   - No `## Depends on`/`## Unlocks` — relationship structure lives solely in INDEX.yml.
   - Full prose paragraphs only, everywhere, including the worked example — no bullets.
   - All math on its own `$$...$$` line, never inline. Spell divisibility/similar relations in prose.
   - Every formula: a sentence before (why) and after (what it means).
   - Closely related equations may share one `$$` block via `\qquad`.
   - Patient-student-to-peer voice — slow, explicit, nothing obvious.
   - No one-sentence paragraphs — attach orphans to a neighbor.
   - Concept defined in an earlier session's notes → cite that file by name, state only the reused fact, don't re-derive.
   - Target 700–1500 words. Shorter (thin wrapper) or longer (multi-concept synthesis) → say so explicitly in Overview or Complexity.
   - Worked example: non-trivial (exercises the interesting case), verifiable mentally in under a minute, traced fully in prose.

3. **Write the history entry.** Correct `HISTORY-YYYY.MM.yml` from the `.rs` file's introducing git commit date (or today if just created); create the monthly file if needed (`month`/`previous`/`next` header, backfill the prior month's `next`). Prepend one item to that file's `entries`:

   ```yaml
   - date: YYYY-MM-DD
     title: "Exact Concept Title"
     session:
       file: src/bin/{filename}.rs   # or — for book sessions
       source: —                     # or "Book Title, Chapter N, Section N.N" for book sessions
       status: completed
       attempted: [...]
       explored: [...]
       tried: [...]
       corrections: [...]
       bugs_found: [...]             # always — for book sessions
       completed: [...]
       not_completed: [...]
       open_questions: [...]
       notes: [...]
   ```

   Fill every field per CLAUDE.md's field semantics, `—` for empty (never omit a field, never an empty list, never mix `—` into a populated list). Observable session events only — no algorithm descriptions/proofs/theorems/complexity derivations (→ notes file, cited by fact if reused), no `Depends on`/`Unlocks` field, no personal info/personality/psychological interpretation/subjective judgment.

4. **Check word count** (`wc -w`). Outside 700–1500 without a stated justification in the file → revise.

5. **Update INDEX.yml incrementally — never a full regeneration.** Read the current INDEX.yml once. Treat every already-completed session's existing fields as ground truth; do not re-read every old `src/bin/*.md`/`*.rs` file to re-derive them from scratch. Then:
   a. Determine the new session's own entry: read its `.rs`/`.md`, classify `prerequisites`/`uses_concepts`/`derived_from`/`related_to`/`unlocks`/`future_targets` honestly against CLAUDE.md's "Fast index (INDEX.yml)" tests, comparing against the *existing* INDEX.yml sessions (not by re-scanning their source files). `reuses_code` stays `—`. Add `summary`/`concepts`/`capabilities`. Insert under `sessions`.
   b. Mechanically patch every derived section with just this session's contribution: add its title to each of its `prerequisites` concepts' `required_by` in `prerequisite_index`; add to `concept_index`/`capability_index` per its tags; add to `completed_by_date` under today's date; remove itself from `future_targets` if previously listed there, add anything newly named; update `branches`/`open_gaps`/`selection_context` only where this session actually changes the frontier (extends a branch, closes a gap, becomes the new most-recent session in its branch).
   c. Retrospective revision of an *older* session's `prerequisites`/`derived_from`/`unlocks`/`related_to` is allowed but is a deliberate, targeted edit — make it only when this new session's evidence specifically implicates that older classification (state the reason in your report). Never a routine side effect, never a from-scratch re-derivation of an older session just to double-check it, never mirrored into that older session's HISTORY entry.
   d. Validate structurally: every session referenced anywhere exists in `sessions`; no future target is also a completed session; every reverse index (`prerequisite_index`/`concept_index`/`capability_index`) matches the forward fields it came from; every session appears exactly once in `completed_by_date`.
