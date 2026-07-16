Audit the effectiveness of the prompts governing this repo (CLAUDE.md and every file in
.skills/) using evidence from the last N sessions in HISTORY-YYYY.MM.yml. You are auditing
the prompts, not the user and not any single session's math content. Do not talk to the
user. Do not edit any file. Return findings as text only.

## Steps

1. Read the last N history entries in full (not just INDEX.yml) and check three
   separate things per session: (a) whether `corrections`, `bugs_found`, or `not_completed`
   needed an unusually long list to capture what happened, which is a signal about
   calibration; (b) whether INDEX.yml's `prerequisite_index` for this session, cross-checked
   against its `corrections`/`bugs_found` entries, suggests the session had to backtrack to
   an earlier concept mid-session (e.g. a correction that restates a fact a genuine
   prerequisite should already have supplied), which is a sign the plan agent under-flagged
   a missing prerequisite; and (c) whether `open_questions` or `not_completed` repeatedly
   names the same kind of gap across sessions, pointing at friction the current field set
   doesn't have a dedicated place to record. Do not read hedging language into the notes
   file's prose as a proxy for this — the notes file is pure mathematical content and should
   not carry session-friction signals at all; that evidence now belongs in HISTORY's fixed
   fields, and its absence there is itself a finding worth reporting.

2. For each rule in CLAUDE.md's "Teaching style" and "Pacing and assumed knowledge"
   sections, look for concrete evidence in the last N sessions' notes/history that the
   rule was followed AND that following it produced a session that closed with a
   genuine correctness/complexity argument (not just a completed one). A rule with no
   discoverable evidence of impact in N sessions is a candidate for rewording or removal.

3. Check whether session-select's proposals over the last N sessions actually spanned
   domains as intended (`.skills/session-select.md`'s "Force genuine breadth" step), or
   whether breadth was suggested but the user always picked the chain-continuation
   option — this is a signal about whether the rule needs to be stronger (e.g.
   requiring justification when breadth is declined) rather than just present.

4. Check whether any notes file needed to violate the 700-1500 word target with a
   stated justification more than once for the same reason (e.g. repeatedly "thin
   wrapper" or repeatedly "large synthesis") — if a pattern recurs, the target band
   itself may need adjusting rather than continuing to except individual sessions.

5. Identify up to 3 specific, minimal wording changes to CLAUDE.md or a .skills/*.md
   file that address a gap found in steps 1-4, each justified by a specific citation
   to a session where the current wording's absence caused friction, ambiguity, or a
   missed opportunity. If more than 3 evidence-backed issues are found, rank them by
   how much friction or ambiguity they caused and report only the top 3; batch any
   remaining valid-but-lower-priority findings into the "unverified ideas" list along
   with genuinely speculative suggestions, clearly distinguishing the two kinds within
   that list. Do not propose changes unsupported by cited evidence from the actual
   history — speculative improvements with no session evidence must never be counted
   toward the 3.

6. Report back: (a) up to 3 evidence-backed wording changes with citations, (b) any
   unverified/speculative ideas worth trying but not yet evidenced, (c) a one-line
   verdict on whether the current prompt set is keeping pace with the user's growth
   or is now either too slack (redundant reps) or too tight (recurring friction) at
   the current stage.
