Investigate the repo and propose exactly 5 candidate topics for the next session. Follow every CLAUDE.md rule, especially "Problem history" and "Book study sessions". Research/report only — no user contact, no AskUserQuestion, no picking a winner. Return findings as text; the calling conversation presents them.

## Steps

1. **Done set.** Read INDEX.yml first — `sessions` keys ARE the done set. Cross-check against the `src/bin/*.rs` file list. Fall back to reading every HISTORY-YYYY.MM.yml in full only if INDEX.yml looks stale/incomplete (e.g. a `.rs` file with no matching `sessions` entry).

2. **Dependency graph.** Read directly from INDEX.yml's `prerequisite_index` and each session's `derived_from`/`unlocks` — genuine prerequisites already kept separate from softer `uses_concepts`/`related_to`. Use `branches`/`open_gaps` for "what's underexplored". HISTORY carries no dependency field by design — if INDEX.yml looks stale, re-derive from `src/bin/*.md`/`*.rs` directly, never from a HISTORY `Depends on`/`Unlocks` field (none exists). Use HISTORY only for session-event citations (a specific bug, a specific abandoned approach) when needed.

3. **Book-study position.** If any `**Source:**` entries exist, find the most recent, identify book/chapter/section. Locate the PDF (`find . -name "*.pdf"`), read its TOC via `pdftotext`, determine the next section in book order. If the most recent book session didn't complete every Exercise/Supplementary Problem for its section, the "next" candidate is finishing that section, not advancing — say so explicitly.

4. **Generate exactly 5 candidates** spanning both tracks (book continuation, if in progress, takes one slot; rest are coding-track). Per candidate:
   - Confirm not already in the done set.
   - Confirm its full prerequisite chain is already covered — except the stepping-stone exception in "Problem history", named explicitly with the larger target it unlocks.
   - Classify: harder variant/extension, shared-idea-different-domain, gap-filling, or stepping-stone.
   - 1-2 sentences on why it's interesting given what's done.
   - Its prerequisite chain (existing sessions/files it draws on).

5. **Force genuine breadth.** Don't let all 5 be the next link in the most recent chain — check whether a cross-domain option (probability, linear algebra, numerical methods, ML, physics) is viable given the done set, include at least one if so.

6. **Report back** a plain list of the 5 (title, track, classification, rationale, prerequisite chain). Not a user-facing menu, no questions asked — structured findings for the calling conversation to present interactively.
