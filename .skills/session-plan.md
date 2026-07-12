A topic for the next session has already been chosen. Investigate what the session actually needs to cover so it doesn't unnecessarily repeat material, and don't skip anything genuinely new. Follow every rule in CLAUDE.md exactly. You are researching and planning only — do not conduct any Socratic dialogue, do not talk to the user, do not write or edit any files. Return a plan as text; the calling conversation will run the actual session using it.

## Steps

1. **Identify the topic's full prerequisite chain.** List every concept the chosen topic depends on, as deeply as the chain goes (a dependency's own dependencies count too).

2. **Check coverage of each prerequisite.** Check `INDEX.txt`'s `sessions` map first — it quickly confirms a prerequisite is covered and gives the exact file (or source) to open. Then read that specific `HISTORY-YYYY.MM.md` entry and `src/bin/*.md` notes file for the specific fact/result the new session should reuse rather than re-derive — per the "Notes writing style" rule on citing earlier sessions instead of re-deriving them. Don't grep every history file when INDEX.txt already points at the right one.

3. **Flag genuinely new material.** Identify what within the chosen topic is not covered by any existing prerequisite — this is what needs full Socratic derivation from first principles, anchored with a concrete small example before any formal question, per "Pacing and assumed knowledge".

4. **Flag missing prerequisites.** Check `INDEX.txt`'s `unlocks_index` map directly — it already flags every concept marked "(not yet its own session)", which is exactly this case (named as a dependency but never completed as its own session, as opposed to a concept merely mentioned in passing). Cross-check against the history files if INDEX.txt looks stale. Say so explicitly. This may mean a review is owed first (per "Theory review"), or that the topic isn't actually ready to start yet.

5. **For book-study topics**, also pull the exact Exercises and Supplementary Problems list and order for the section directly from the PDF (find it via `find . -name "*.pdf"`, read with `pdftotext`), so the session can follow strict book order per "Book study sessions". Note whether the book section itself re-covers something already implemented as a `src/bin` file (as happened when Section 2.2 re-derives the Sieve of Eratosthenes) — if so, say that part should be a quick review, not fresh teaching.

6. **For coding topics**, suggest a filename following the "File naming" grouping convention (shared prefix with related existing files) — but only as a suggestion; the user names their own file. Also name the algorithm's standard term from the literature if one exists, so the user can look it up under its recognized name — this is a citation of terminology, not a code suggestion; never propose an actual function/variable identifier.

7. **Report back** a structured plan: (a) prerequisites to cite-and-confirm briefly, each with its source file and the specific fact to reuse, (b) any prerequisite needing a deeper review first and why, (c) what's genuinely new and a suggested concrete small-example anchor for opening it, (d) for book sessions, the exact ordered problem list, (e) for coding sessions, a suggested filename. Do not include a Socratic question script — the calling conversation designs the actual dialogue from this plan.
