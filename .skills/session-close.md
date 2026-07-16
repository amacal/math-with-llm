Write the notes file and session history entry for the session that just ended. Follow every rule in CLAUDE.md exactly.

## Steps

1. **Identify the file.** Find the `src/bin/*.rs` file created or primarily worked on this session. If it was a book-reading session (no .rs file), identify the source instead.

2. **Write the notes file** at `src/bin/{filename}.md` (same base name as the .rs file). Follow these rules from CLAUDE.md exactly:
   - Section order: `# Title`, `## Overview`, zero or more bespoke theory/derivation sections, `## Correctness`, `## Complexity`, `## Edge cases` (if applicable), `## Worked example`
   - No `## Depends on` or `## Unlocks` sections — relationship structure lives solely in `INDEX.yml`
   - Full prose paragraphs only — no bullet points, anywhere, including in the worked example
   - All math on its own display line using `$$...$$`. No inline math inside sentences. Spell divisibility and similar relations in prose
   - Every formula gets a sentence before (why you are about to write it) and after (what it means)
   - Closely related equations may share one `$$` block separated by `\qquad`
   - Write as a patient student explaining to a peer — slow, explicit, nothing obvious
   - No one-sentence paragraphs — attach orphan sentences to a neighbor
   - When a concept was defined in an earlier session's notes, cite that file by name and state only the reused fact; do not re-derive it
   - Target 700–1500 words. If shorter (thin wrapper) or longer (synthesis of many concepts), say so explicitly in Overview or Complexity
   - The Worked example must be non-trivial (exercises the interesting case) and small enough to verify mentally in under a minute; trace it fully in prose

3. **Write the history entry.** Determine the correct `HISTORY-YYYY.MM.yml` file from the git commit date of the .rs file (or today's date if the file was just created); create the monthly file if it doesn't exist yet (see CLAUDE.md's "Session history" section for the `month`/`previous`/`next` header and backfilling the prior month's `next`). Prepend one new item to the top of that file's `entries` list, following this shape exactly:

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

   Fill every field per CLAUDE.md's field semantics — `attempted`/`explored`/`tried`/`corrections`/`bugs_found`/`completed`/`not_completed`/`open_questions`/`notes` — using `—` for any field with no entries (never omit a field, never leave a list empty, never mix `—` into a populated list). Record only observable session events: what was attempted, explored, tried, corrected, found broken, completed, deferred, or left open. Do **not** write a mathematical summary paragraph — no algorithm descriptions, proofs, theorem statements, or complexity derivations; those live in the notes file, cited by fact if reused. Do **not** add a `Depends on` or `Unlocks` field, or anything resembling one — that belongs solely in `INDEX.yml`. Do **not** include any personal information, personality description, psychological interpretation, or subjective judgment of the user — only observable facts about the work.

4. **Check word count** of the notes file with `wc -w`. If it falls outside 700–1500 without a stated justification in the file, revise.

5. **Regenerate INDEX.yml in full.** Do not hand-patch individual entries. Re-derive the whole file primarily from every `src/bin/*.md` notes file and `src/bin/*.rs` implementation (the evidence for relationships), using `HISTORY-2026.06.yml`, `HISTORY-2026.07.yml` (and any later monthly files) only for session-event context — never read a `Depends on`/`Unlocks` field out of HISTORY, since it doesn't carry one. Classify the new session's relationships to earlier sessions honestly against the strict tests in CLAUDE.md's "Fast index (INDEX.yml)" section — `prerequisites` only for what's genuinely necessary first, `uses_concepts`/`derived_from`/`related_to` for the softer cases, `reuses_code` stays `—` (this repo forbids code reuse across `src/bin/*.rs` files). Because INDEX.yml (unlike HISTORY) is allowed to change retrospectively, also re-examine whether the new session's evidence should revise an *earlier* session's `prerequisites`/`derived_from`/`unlocks`/`related_to` fields — apply that revision only to INDEX.yml, never to the earlier HISTORY entry. Then update every derived section: `prerequisite_index`, `concept_index`, `capability_index`, `completed_by_date`, `future_targets` (remove the new session's own title if it was previously listed there as a target; add any newly-named future work), and `branches`/`open_gaps`/`selection_context` if the new session changes which branch is most recently active. Validate structurally before finishing: every session referenced anywhere exists in `sessions`, no future target is also a completed session, every reverse index (`prerequisite_index`, `concept_index`, `capability_index`) matches the forward fields it was built from, and every session appears exactly once in `completed_by_date`.
