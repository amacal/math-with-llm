Investigate the repo and propose exactly 5 candidate topics for the next session. Follow every CLAUDE.md rule, especially "Problem history" and "Book study sessions". Research/report only — no user contact, no AskUserQuestion, no picking a winner. Return findings as text; the calling conversation presents them. No scripts — every lookup is `Read`/`Grep`/`Glob` against `.index/`/`.history/`, per `.index/schema.yml` and `.history/schema.yml`.

## Steps

1. **Done set.** `ls .index/sessions/` (each directory's `meta.yml` `title:` field is the exact Concept Title) — this IS the done set. Cross-check against the `src/bin/*.rs` file list. Fall back to reading every `.history/*/*.yml` in full only if `.index/` looks stale/incomplete (e.g. a `.rs` file with no matching `.index/sessions/*/meta.yml` entry).

2. **Dependency graph.** `grep -rl "<concept title>" .index/sessions/*/prerequisites.yml` for reverse "required-by" lookups; `Read .index/sessions/<slug>/derived_from.yml`/`unlocks.yml` for a session's own forward relationships — genuine prerequisites already kept separate from softer `uses_concepts.yml`/`related_to.yml`. `ls .index/branches/` + `Read` each for branch structure, `ls .index/open-gaps/*/` + `Read` each for "what's underexplored". `.history/` carries no dependency field by design — if `.index/` looks stale, re-derive from `src/bin/*.md`/`*.rs` directly, never from a `.history/` `Depends on`/`Unlocks` field (none exists). Use `grep -l '"<title>"' .history/*/*.yml` or `grep -r "<term>" .history/*/*.yml` only for session-event citations (a specific bug, a specific abandoned approach) when needed.

3. **Book-study position.** If any book-study sessions exist (`grep -l 'kind: "book-study"' .index/sessions/*/meta.yml`), find the most recent by date, identify book/chapter/section from its `source` field. Locate the PDF (`find . -name "*.pdf"`), determine the next section in book order by rendering candidate TOC/section-heading pages to PNG via `pdftoppm`/`pdftocairo` and reading them with multimodal vision — never `pdftotext`, not even for the TOC (per "Book study sessions"'s total ban on text-extraction against this PDF). If the most recent book session didn't complete every Exercise/Supplementary Problem for its section, the "next" candidate is finishing that section, not advancing — say so explicitly.

4. **Generate exactly 5 candidates** spanning both tracks (book continuation, if in progress, takes one slot; rest are coding-track). Per candidate:
   - Confirm not already in the done set.
   - Confirm its full prerequisite chain is already covered — except the stepping-stone exception in "Problem history", named explicitly with the larger target it unlocks.
   - Classify: harder variant/extension, shared-idea-different-domain, gap-filling, or stepping-stone.
   - 1-2 sentences on why it's interesting given what's done.
   - Its prerequisite chain (existing sessions/files it draws on).

5. **Force genuine breadth.** Don't let all 5 be the next link in the most recent chain — check whether a cross-domain option (probability, linear algebra, numerical methods, ML, physics) is viable given the done set, include at least one if so.

6. **Report back** a plain list of the 5 (title, track, classification, rationale, prerequisite chain). Not a user-facing menu, no questions asked — structured findings for the calling conversation to present interactively.
