Investigate the repo and propose exactly 5 candidate topics for the next session. Follow every rule in CLAUDE.md exactly, especially "Problem history" and "Book study sessions". You are researching and reporting only — do not talk to the user, do not call AskUserQuestion, do not pick a winner. Return your findings as text; the calling conversation will present them.

## Steps

1. **Build the "done" set.** List every `src/bin/*.rs` file. Read every `HISTORY-YYYY.MM.md` file (the alphabetically-last one is the most recent month, but earlier months matter too) and collect every Concept Title from every entry — both `**File:**` (coding) and `**Source:**` (book) sessions. Anything in this set is off-limits as a fresh proposal.

2. **Build the dependency graph.** From the `Depends on:` / `Unlocks:` fields across all history files, work out which concepts are transitively "covered" (reachable from completed sessions) versus which are named as dependencies but never actually completed as their own session.

3. **Find the book-study position.** If any `**Source:**` entries exist, find the most recent one and identify its book, chapter, and section. Locate the book's PDF (`find . -name "*.pdf"` from the repo root) and read its table of contents (via `pdftotext`) to determine the immediate next section in book order. If the most recent book session did not complete every Exercise and Supplementary Problem for its section (check the notes/history text for this), the "next" book candidate is finishing that same section, not advancing — say so explicitly.

4. **Generate exactly 5 candidates** spanning both tracks (the book continuation, if one is in progress, normally occupies one slot; the rest are coding-track). For each:
   - Confirm it is NOT in the "done" set from step 1.
   - Confirm its full prerequisite chain is already covered — except for the stepping-stone exception in CLAUDE.md's "Problem history", which must be named explicitly as such along with the larger target it unlocks.
   - Classify it: harder variant/extension, shared-idea-different-domain, gap-filling, or stepping-stone.
   - State in 1-2 sentences why it's interesting given what's already been done.
   - List its prerequisite chain (which existing sessions/files it draws on).
5. **Force genuine breadth.** Do not let all 5 candidates be the next link in the most recent chain. Deliberately check whether a cross-domain option (probability, linear algebra, numerical methods, ML, physics) is viable given the current "done" set, and include at least one if it is.

6. **Report back** a plain list of the 5 candidates (title, track, classification, rationale, prerequisite chain). Do not format this as a user-facing menu or ask any questions — just return the structured findings for the calling conversation to present interactively.
