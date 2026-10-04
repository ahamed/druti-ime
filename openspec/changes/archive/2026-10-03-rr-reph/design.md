## Context

`process_consonant` writes a hasant before any consonant typed after a kar-taking consonant in the
buffer, র included, so `rt` gives `র্ত`. `rules::rassaw_ri` makes ঋ from a buffer ending in `র্র`
(typed `rr`) followed by `i`, and ঋ-kar when a consonant is stacked before it. Only a silent `o`
separates two consonants.

The composer keeps the whole word pending (whole-word-pending D1). After a Backspace, the next
consonant starts a new letter (whole-word-pending D3).

How this rule was chosen:
- An earlier draft, "smart hasant", resolved whole words using a verb-root list, a backtick join key,
  and a re-check after every key. It was rejected: the word list made output hard to predict, and
  text changed after it was shown (`কর্তা` → `করতাম`).
- A four-letter variant (র before ত/ল/ব/ছ only, with a doubled consonant for reph) needed a set of
  letters to memorise.
- This design was settled in a review with the author.

Evidence from a 2.4M-word Bengali corpus (hermitdave/FrequencyWords, subtitles):

| র + consonant, mid-word | tokens |
|---|---|
| reph (র্C, excluding র্য) | 40,825 |
| unwritten vowel (র C + kar) | 67,306 |
| র্য (typed with `y` today; `rry` or `rrz` after this change) | 4,372 |

## Goals / Non-Goals

**Goals:**
- Two rules a typist can hear: a single `r` never forms reph (`rr` does), and nothing joins after a
  breathy letter, হ, ড়, ঢ় or য় except a ফলা (D4b).
- Nothing before the last vowel ever changes. Each key changes only the cluster it is typed into.
- ঋ and every spelling that uses `o` keep working as before; `y` always gives a visible য-ফলা.

**Non-Goals:**
- Which other consonant pairs join (`dekhte` → `দেখ্তে` today), including `ড়` / `ঢ়` (`poRte` →
  `পড়্তে`). That is the `conjunct-rules` change, which builds on this one.
- A setting to keep the old behaviour.
- Nouns that need `o` for other consonant pairs (`ekoTa`, `aponi`); out of scope.

## Decisions

### D1. The rule lives in the engine, decided by the key being typed
The engine already rewrites only its buffer, which is the open cluster. All the new behaviour is a
buffer rule:
- `process_consonant`: when the last letter in the buffer is `র`, the consonant is written without a
  hasant, and the buffer restarts with it. An `r` key there is the arming rule (D2), and `y` stays
  য-ফলা (D4).
- `process_vowel`: when the buffer ends in an armed `র্`, `i` makes ঋ (D3); any other vowel removes the
  hasant first.

No rule reads past the cluster or looks at later keys, so the output is a fixed function of the keys,
like today. Bulk conversion, the composer and both bindings get the behaviour unchanged, because they
all drive the engine.

*Alternative:* a word-level pass in the composer, as in the dropped smart hasant draft. Rejected: it
rewrites text after it is shown, and needs a word list.

### D2. `rr` shows a visible hasant
The second `r` inserts `্`, so `korr` shows `কর্`. The armed state is just "the buffer ends in `র্`".
The engine never otherwise leaves a buffer ending in a hasant, so there are no hidden flags. A
consonant then follows the hasant: `process_consonant` already writes no extra hasant when the last
unit isn't a consonant, so `র্` + `ত` → `র্ত`. Aspiration, `kkh` and the nasal rules then work on the
consonant as usual (`orrth` → `অর্থ`).

*Alternative:* show `কর` and keep the reph pending invisibly, which was the author's first idea.
Rejected in review: `kor` and `korr` would look the same, and Backspace couldn't show what it would
undo.

### D3. A vowel after `rr`
- `i` keeps today's ঋ rule, now reading `র্` instead of `র্র`: with a consonant stacked before the র
  (`krr` → `ক্র্`), the `্র্` becomes ঋ-kar (`কৃ`); otherwise `র্` becomes `ঋ`.
- Any other vowel deletes the hasant, then is processed as after `র`. That covers the kar, the silent
  `o`, and `O` keeping the buffer for `ঐ`/`ঔ`.

Today `rr` is used only for ঋ, so ঋ typing is unchanged for users. Reph never comes before a vowel, so
`rr` + vowel has no other use.

### D4. `y` and `z`
Unicode writes "র + য-ফলা" and "reph over য" with the same code points (`র্য`), and fonts draw them as
reph. A single `r` must never make reph (D1), so `y` after a lone র writes the visible য-ফলা form
instead:
- After a র that is not র-ফলা and not armed, `ja_fala` writes ZWJ + `্য` (`র‍্য`, U+09B0 U+200D U+09CD
  U+09AF), the form used in র‍্যাব and র‍্যালি. Avro, Khipro and OpenBangla's fixed layouts write the
  same sequence.
- After র-ফলা (`bry`), `ja_fala` writes `্য` with no ZWJ, as today; the stacked র already shows.
- After an armed `র্`, `y` appends `য`, giving reph over য: `porryonto` → `পর্যন্ত`, `karryo` → `কার্য`.
- `z` is an ordinary consonant mapped to `য` and follows D1/D2 (`porzonto` → `পরযন্ত`, `porrzonto` →
  `পর্যন্ত`).

The ZWJ is part of the য-ফলা letter: letter Backspace removes ZWJ, hasant and য together, and every
length counts it as one UTF-16 unit.

The cost: র্য is reph in 99% of corpus occurrences (পর্যন্ত, সূর্য, কার্যক্রম), so those words need
`rr`, like every other reph. The ZWJ form is about 2% of র+য in Wikipedia text. The author chose one
rule ("only `rr` makes reph") over the frequency.

Changes from today:
- `ry` used to give `র্য` (reph); it now gives `র‍্য`.
- `rz` used to give `র্য`; it now gives `রয`.
- `rry` and `rrz` are new.

### D4b. Nothing joins after a breathy letter
The second rule of this change, chosen in review over the 107-pair list of `conjunct-rules` because
it is a category a typist can hear, not a list to memorise:
- After খ ঘ ছ ঝ ঠ ঢ থ ধ ফ ভ, হ, ড়, ঢ় or য়, `process_consonant` writes the consonant without a hasant and
  restarts the buffer, exactly as after a single র (D1).
- The ফলা keys still join: `r`, `l`, `m`, `n`, `N`, `w`, and `y` (`ja_fala`). The test is on the key,
  not the letter, for ব: `w` joins (ধ্বনি), `b` does not (দেখবে, বুঝবে). `w` and `b` both give ব
  everywhere else, so nothing changes for other letters.
- The decision uses the letter as it stands when the key is typed. Aspiration only ever rewrites the
  last letter, so a letter already written never becomes breathy behind a join.

Evidence (subtitle and Wikipedia corpora, research in `conjunct-rules/research.md`): খ+ত, ঝ+ত, খ+ব
have zero joined tokens; য়+গ and য়+ত appear thousands of times apart and never joined. With `rr-reph`
alone, 57.9% / 70.6% of words need no extra `o`; with this rule about 62% / 73%. Joining a ফলা after
these letters keeps চিহ্ন, ব্রাহ্মণ, ফ্লাইট, ভ্রমণ, ধ্বংস typable without any join key. The cost is
হ+ল: `tahle` gives তাহ্লে, so তাহলে is typed `tahole` (or fixed by `autocorrect`).

### D5. Backspace
Letter Backspace already treats a lone trailing hasant as one letter, so Backspace on `কর্` gives `কর`.
In the composer, a Backspace ends the cluster (whole-word-pending D3). So the next key starts a new
letter: `t` gives `করত`, and so does `r` (`করর`).
- To re-type a reph, remove the `র` too and type `rr` again.
- Typing `r` again could instead re-arm the reph. That would make one consonant key read the letter
  before a Backspace, which D3 forbids for every consonant. Keeping D3 whole is simpler to learn.

The engine's own `process_backspace` resumes the cluster (rust-engine-core "Resuming the cluster from
the output"), so at engine level an `r` after it does re-arm. Hosts use the composer, so they don't
see this; the engine fixtures already pin the difference.

### D6. Default, version and the config rule
- The rule replaces the old one with no setting.
- `openspec/config.yaml` asks that new behaviour default to the current one "unless a change says so
  explicitly". This change says so.
- Reason: the rule is core typing, and two variants would split users' muscle memory and double the
  fixture set.
- The app version goes to 2.0.0, because existing reph spellings (`korta`) change meaning.

### Behaviour and fixture changes
This change intentionally alters engine output. Fixtures edited in the same commit:
- Counts below were measured before the `ry` (D4) and breathy-letter (D4b) rules, which add more; the
  regeneration script reports the final numbers.
- `engine/unit.json`: 5 cases that type `r` before a consonant or `rr`. The ঋ cases keep their final
  output, but their per-key actions change: `r` now inserts `্`, and `i` deletes `্র্`.
- `engine/words.json`: 18 cases.
- `engine/random.json`: 201 of the 2,400 sequences.
- `engine/transpile.json`: 9 cases.

They are regenerated with a one-off script that replays each affected case on the new engine. The
script asserts that every case without an `r` before a consonant key, `rr`, `ry`, or a consonant key after a breathy letter, হ, ড়, ঢ় or য় is unchanged, and stays out of the
repo. The review diff shows only these cases. `composer/` fixtures are unchanged; new composer cases
are added for the ime-composer requirement.

## Risks / Trade-offs

- **[Trade-off] Every reph costs one extra key** (about 1.7% of words in the corpus). In return,
  about 2.9% of words lose an unspoken `o`. The rule is one line to learn.
- **[Risk] Existing users' muscle memory:** `korta` now gives `করতা`, `dhormo` gives `ধরম`, and
  `poryonto` gives `পর‍্যন্ত`. →
  2.0.0 release notes lead with a table from old to new spellings. The README typing table shows
  `rr` first.
- **[Trade-off] Reph after a Backspace needs the র re-typed** (D5).
- **[Risk] Changed random sequences hide an unintended change.** → The regeneration script refuses to
  touch any case without an `r` before a consonant key, `rr`, `ry`, or a consonant key after a breathy letter, হ, ড়, ঢ় or য়. The hosts property test (every host shows the same
  text) runs on the new fixtures.

## Migration Plan

1. Land the engine change with its fixtures and docs, then bump to 2.0.0 and release with the
   spelling table.
2. Rollback is a revert of the change; no data or settings are involved.
