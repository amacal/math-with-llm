A topic for the next session is already chosen. Investigate what it actually needs to cover so it doesn't unnecessarily repeat material, and doesn't skip anything genuinely new. Follow every CLAUDE.md rule. Research/plan only — no Socratic dialogue, no user contact, no file writes. Return a plan as text; the calling conversation runs the actual session from it.

## Steps

1. **Full prerequisite chain.** List every concept the topic depends on, as deep as the chain goes (a dependency's own dependencies count too).

2. **Check coverage of each prerequisite.** `scripts/index-query.py session "<title>"` first — confirms coverage fast, gives the exact file/source, plus `prerequisites`/`derived_from`/`uses_concepts` for the genuine dependency shape. Then read that prerequisite's `src/bin/*.md` for the specific fact/result the new session should reuse rather than re-derive (per "Notes writing style"). `scripts/history-query.py by-title "<title>"` to find its HISTORY entry, read that one entry for session-event context (a known bug already hit, an approach already tried and abandoned, work already deferred) — HISTORY carries no math content and no dependency field. Don't grep every history file when INDEX.yml already points at the right one.

3. **Flag genuinely new material.** What within the topic isn't covered by any existing prerequisite — this needs full Socratic derivation from first principles, anchored with a concrete small example before any formal question (per "Pacing and assumed knowledge").

4. **Flag missing prerequisites.** `scripts/index-query.py future-targets` — lists every topic explicitly named as future work but not yet done (`status: not-completed`), exactly this case. Cross-check history files if INDEX.yml looks stale. Say so explicitly — may mean a review is owed first (per "Theory review"), or the topic isn't actually ready yet.

5. **Book-study topics:** also pull the exact Exercises/Supplementary Problems list and order directly from the PDF (`find . -name "*.pdf"`), rendered to PNG via `pdftoppm`/`pdftocairo` and read via multimodal vision — never `pdftotext`, not even to locate the right physical page (per "Book study sessions"), for strict book order. Note if the section itself re-covers something already implemented as a `src/bin` file (as happened when Section 2.2 re-derives the Sieve of Eratosthenes) — if so, say that part should be a quick review, not fresh teaching.

6. **Coding topics:** suggest a filename following the grouping convention (shared prefix with related existing files) — a suggestion only, the user names their own file. Also name the algorithm's standard literature term if one exists, so the user can look it up under its recognized name — a terminology citation, never an actual function/variable identifier.

7. **Report back** a structured plan: (a) prerequisites to cite-and-confirm briefly, each with source file and the specific fact to reuse; (b) any prerequisite needing a deeper review first, and why; (c) what's genuinely new, plus a suggested concrete small-example anchor for opening it; (d) book sessions: the exact ordered problem list; (e) coding sessions: a suggested filename. No Socratic question script — the calling conversation designs the actual dialogue from this plan.
