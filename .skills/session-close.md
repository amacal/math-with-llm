Write the notes file, the `.history/` entry, and the `.index/` session directory for the session that just ended. Follow every CLAUDE.md rule exactly. No scripts — every write here is a direct `Write`/`Edit`; every lookup is `Read`/`Grep`/`Glob` against `.index/`/`.history/`, per `.index/schema.yml` and `.history/schema.yml`.

## Steps

1. **Identify the file.** Find the `src/bin/*.rs` file created/primarily worked on this session. Book-reading session (no `.rs`) → identify the source instead.

2. **Write the notes file** at `src/bin/{filename}.md` (same base name). Follow CLAUDE.md's "Notes writing style" exactly:
   - Section order: `# Title`, `## Overview`, zero+ bespoke theory/derivation sections, `## Correctness`, `## Complexity`, `## Edge cases` (if applicable), `## Worked example`.
   - No `## Depends on`/`## Unlocks` — relationship structure lives solely in `.index/`.
   - Full prose paragraphs only, everywhere, including the worked example — no bullets.
   - All math on its own `$$...$$` line, never inline. Spell divisibility/similar relations in prose.
   - Every formula: a sentence before (why) and after (what it means).
   - Closely related equations may share one `$$` block via `\qquad`.
   - Patient-student-to-peer voice — slow, explicit, nothing obvious.
   - No one-sentence paragraphs — attach orphans to a neighbor.
   - Concept defined in an earlier session's notes → cite that file by name, state only the reused fact, don't re-derive.
   - Target 700–1500 words. Shorter (thin wrapper) or longer (multi-concept synthesis) → say so explicitly in Overview or Complexity.
   - Worked example: non-trivial (exercises the interesting case), verifiable mentally in under a minute, traced fully in prose.

3. **Write the `.history/` entry.** Determine `date` from the `.rs` file's introducing git commit (or today if just created). Derive `<slug>` from the exact Concept Title via `.index/schema.yml`'s `slug_algorithm`. `Write` a single new file at `.history/<YYYY-MM>/<date>-<slug>.yml` (create the month directory if it doesn't exist yet — nothing else to set up, no header/chaining fields) with exactly the shape in `.history/schema.yml`'s `file_format`:

   ```yaml
   date: "YYYY-MM-DD"
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

   Fill every field per CLAUDE.md's field semantics, `—` for empty (never omit a field, never an empty list, never mix `—` into a populated list; each list field is either the literal `—` or a non-empty list). Observable session events only — no algorithm descriptions/proofs/theorems/complexity derivations (→ notes file, cited by fact if reused), no `Depends on`/`Unlocks` field, no personal info/personality/psychological interpretation/subjective judgment. This `Write` is the entire operation — never touch any other file under `.history/`.

4. **Check word count** (`wc -w`) on the notes file. Outside 700–1500 without a stated justification in the file → revise.

5. **Write the `.index/sessions/<slug>/` directory (10 files) — never a full-tree read, no script.** Treat every already-completed session's existing files as ground truth; do not re-read every old `src/bin/*.md`/`*.rs` file to re-derive them from scratch. Classify honestly against CLAUDE.md's "Fast index (`.index/`)" tests, using targeted lookups only:
   - `grep -l 'title: "<candidate title>"' .index/sessions/*/meta.yml` to confirm a candidate prerequisite/related session actually exists and get its directory.
   - `Read` that session's own `.yml`/`.txt` files (not the whole tree) for the specific reused fact.

   Then `Write` all 10 files in `.index/sessions/<slug>/`: `meta.yml` (`title`/`kind`/`file`/`source`/`date`), `summary.txt` (plain text, one sentence), and `prerequisites.yml`/`uses_concepts.yml`/`derived_from.yml`/`related_to.yml`/`unlocks.yml`/`future_targets.yml`/`concepts.yml`/`capabilities.yml` (each a YAML list of exact titles/tags, or the literal scalar `—`, per `.index/schema.yml`'s `empty_field_convention`). No `reuses_code` file — it's never written, by design.

6. **Future targets.** For each title in this session's own `future_targets.yml` that isn't already a `.index/future-targets/<slug>.yml` file, `Write` one (`title`/`status: not-completed`/`mentioned_by`/`evidence`, per `.index/schema.yml`). If this session's own title matches an *existing* `.index/future-targets/<slug>.yml`, `rm` that file now — a title cannot be both a pending target and a completed session.

7. **Branches.** If this session extends an existing branch, `Edit` `.index/branches/<slug>.yml`: append the title to `sessions`, and replace `frontier` with whatever the new frontier genuinely is (usually just this session, but say so explicitly if a sibling session in the same branch is still also frontier). If this session opens a brand-new branch, `Write` a new `.index/branches/<slug>.yml` (`title`/`sessions`/`frontier`/`explicit_future_targets`).

8. **Everything else is curated, not mechanical — touch only what genuinely changed.** `.index/selection-context/active-branches.yml`, `candidate-signals.yml`, and `reusable-recent-capabilities.yml` are hand-curated judgment (verified against real repo history: these do **not** follow a formula — don't try to make one). `.index/open-gaps/<category>/<slug>.yml` and `.index/edges/<slug>.yml` are curated prose, written rarely. Edit only the specific file this session's evidence actually changes. Never touch `.index/schema.yml` itself as part of a normal close (a session doesn't change the schema). Nothing needs to be written for `recent_sessions` or `explicit_unfinished_targets` — those are computed on demand, never stored (see `.index/schema.yml`'s `computed_not_stored`).

9. **Retrospective revision of an *older* session's fields is still allowed** (that's `.index/`'s whole point of difference from `.history/`), but is a deliberate, targeted `Edit` to that one file — only when this new session's evidence specifically implicates that older classification (state the reason in your report). Never a routine side effect, never a from-scratch re-derivation of an older session just to double-check it, never mirrored into that older session's `.history/` entry.

10. **Validate before finishing — no script, run these directly with Grep/Glob** (the exact checks are in `.index/schema.yml`'s `validation` section):
    - `ls .index/sessions/<slug>/ | wc -l` is exactly 10.
    - Every title in every `prerequisites.yml`/`uses_concepts.yml`/`derived_from.yml`/`related_to.yml` you just wrote resolves: `grep -l 'title: "<ref>"' .index/sessions/*/meta.yml` finds exactly one file.
    - Every title in `unlocks.yml`/`future_targets.yml` resolves to either a session or a `.index/future-targets/*.yml` title.
    - No title exists as both a `.index/sessions/*/meta.yml` title and a `.index/future-targets/*.yml` title.
    - Every tag in `concepts.yml`/`capabilities.yml` matches `^[a-z0-9]+(-[a-z0-9]+)*$`.

11. **Report back** every file you wrote/edited/removed, verbatim, with full paths (the new `.history/` entry path, the `.index/sessions/<slug>/` directory and its 10 files, any future-target/branch files touched, any retrospective edit and its stated reason) — the calling conversation passes this into the verify agent's prompt so it can check the actual files directly instead of re-deriving "what changed" by globbing the whole tree.
