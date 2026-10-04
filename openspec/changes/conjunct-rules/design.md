> **Status: parked (2026-10-03). Do not apply.** The author chose not to ship a joining list: it
> has to be memorised, which works against typing without looking at the screen. `rr-reph` takes only
> its cheapest, audible rule (nothing joins after a breathy letter), and words such as আমরা, একটা,
> আপনি are left to the optional `autocorrect` change. This folder is kept for its research
> (`research.md`) and in case a joining list is offered later as an opt-in setting.

## Context

After `rr-reph`, `process_consonant` still writes a hasant before any consonant typed after a
kar-taking consonant in the buffer, so every pair joins except those starting with র. Only a silent
`o` separates. Today's rules that build a conjunct directly (`nc` → ঞ্চ, `nj` → ঞ্জ, `kkh` → ক্ষ,
`gg` → জ্ঞ, `x` → ক্স) and the `y` rule for য-ফলা stay as they are.

How this design was chosen (full evidence in `research.md`; the simulation figures marked "computed"
there and in this design come from scripts run during the proposal, not kept in the repo):
- **Grammar:** neither Bangla Academy, NCTB nor Paschimbanga Bangla Akademi publishes a closed list of
  permitted conjuncts. Their rules only restrict (no doubling after reph, no ষ/ণ in foreign words). The
  families below come from inherited Sanskrit phonology; usage decides the members.
- **Inventories:** bn.wiktionary's list (via `bnunicodenormalizer`), `bkit` and Avro's pattern table
  agree on 106 of 122 true-conjunct pairs. 15 fonts shaped with HarfBuzz were used only as a negative
  check.
- **Corpora:** hermitdave FrequencyWords subtitles (2.4M tokens, everyday register) and Google
  Dakshina Bengali Wikipedia (12.3M tokens, formal). Pairs that subtitles write apart more often than
  joined were dropped from the table.
- **Other IMEs:** Avro, Khipro and Wikimedia's bn-avro join only listed pairs. Khipro is deterministic,
  has a force-join key (`qq`) and a visible-hasant key (`xx`); all 107 pairs here are in its table.
- **Author decisions** (review): favour everyday writing; exceptions are pairs, not words; the force
  key is the backtick; rare Sanskrit pairs join only where they break no common word; `w` after a
  consonant is ব-ফলা; one change on top of `rr-reph`, same release.

| Rule set | Words right, chat / formal |
|---|---|
| Druti 1.x (join every pair) | 46.9% / 65.3% |
| `rr-reph` alone | 57.9% / 70.6% |
| Table inside words only | 92.2% / 90.0% |
| Table + word-start rule (this change) | 92.4% / 90.6% |
| Same + a suffix exception (needs lookahead; rejected) | ~95.6% / ~91.7% |

## Goals / Non-Goals

**Goals:**
- A rule a typist can learn once: "start of a word: join; inside a word: join only family pairs; `o`
  separates; backtick joins".
- Every decision made by the key being typed, from the open cluster and whether it starts the word.
  No word lists, no lookahead; text before the last vowel never changes.
- The table is data, reviewable pair by pair, and pinned by a fixture.

**Non-Goals:**
- Verb and classifier suffixes on stems that end in a listed pair (জানতে, থাকতে, আসবে, কোনটা). No
  left-to-right rule sees the suffix, and the top 20 stems as exceptions would fix ~13,700 subtitle
  tokens but break 17,370 (51,699 in Wikipedia): কিন্তু, চিন্তা, ডাক্তার, আন্তর্জাতিক. These are
  typed with `o` (`janote`).
- A setting for the old behaviour.
- Typing the ZWJ র‍্য and reph: owned by `rr-reph`.

## Decisions

### D1. Joining is decided in `process_consonant`, per key
For a consonant key after a consonant in the buffer:
1. A র as the first letter, and `y` / `r` keys: `rr-reph` rules, unchanged.
2. `z` (য): never joins (D6).
3. `w`: always joins as ব-ফলা (D6).
4. The cluster starts a word (D2): join.
5. Otherwise join iff (last consonant, new consonant) is in `JOINING_PAIRS` (D3).

Joining appends `্` + consonant to the buffer, as today. Not joining appends the consonant and
restarts the buffer with it, as `rr-reph` D1 does after র. The decision uses the consonant the key
produces (`K` → খ, `S` → শ), so it is fixed when the key is typed.

### D2. "Start of a word" is read from the output
The cluster starts a word when the output before the buffer is empty or ends in something other than
a Bengali letter, kar or sign (space, punctuation, digit, Latin text). Reason: the first syllable's
vowel is always spoken in Bengali, so a typist always types it (`gola` → গলা), and two consonants
typed there with no vowel can only be a conjunct (গ্লাস, স্কুল, স্রষ্টা). Measured gain is small
(+0.2 / +0.6 points of words) but it is what typists expect from loanwords, and it costs nothing
measurable.

Composer: the pending word is all the engine sees for consonant keys (`key_reads_document` stays
vowels, `-`, `.` and quotes), so after a caret move into the middle of a word, the new pending text
counts as a word start. This only matters for the 44 pairs that join at a word start but not inside
(D3), and only when typing into the middle of an existing word.

*Alternative:* a broader second list for word starts. Rejected: "everything joins at the start" is
one fewer table to learn, and those words begin with real clusters anyway.

### D3. The joining table
`data::JOINING_PAIRS`, 107 pairs, written out explicitly (not computed from families) so that every
pair is reviewable and pinned by `engine/data.json`. Families are documentation:

| Family | Pairs |
|---|---|
| Nasal + own row (19) | ঙ্ক ঙ্খ ঙ্গ ঙ্ঘ ঞ্চ ঞ্ছ ঞ্জ ঞ্ঝ ণ্ট ণ্ঠ ণ্ড ন্ত ন্থ ন্দ ন্ধ ম্প ম্ফ ম্ব ম্ভ |
| ন + ট-row, loanwords (3) | ন্ট ন্ঠ ন্ড |
| স/শ/ষ + stop (15) | শ্চ শ্ছ ষ্ক ষ্ট ষ্ঠ ষ্ণ ষ্প ষ্ফ স্ক স্খ স্ট স্ত স্থ স্প স্ফ |
| Doubled letter (12) | ক্ক চ্চ জ্জ ট্ট ড্ড ণ্ণ ত্ত দ্দ ন্ন প্প ব্ব ম্ম |
| Plain + own breathy (4) | চ্ছ জ্ঝ ত্থ দ্ধ |
| র-ফলা (14) | ক্র খ্র গ্র ঘ্র ট্র ড্র ত্র দ্র প্র ফ্র ব্র ভ্র শ্র হ্র |
| ল-ফলা (5) | ক্ল প্ল ফ্ল ব্ল শ্ল |
| ব-ফলা (7) | জ্ব ত্ব দ্ব ধ্ব শ্ব স্ব হ্ব |
| ম-ফলা (6) | ঙ্ম ণ্ম ত্ম ন্ম ষ্ম স্ম |
| ন-ফলা (6) | গ্ন ত্ন ধ্ন শ্ন স্ন হ্ন |
| ক্ষ, জ্ঞ (2) | ক্ষ জ্ঞ |
| Listed (14) | ক্ত ক্স গ্ধ চ্ঞ দ্ঘ দ্ভ ন্স প্ট প্ত ব্দ ল্ট ল্ড ল্প হ্ণ |

How it was built: the 115 pairs found in at least two inventories with at least 20 corpus tokens;
minus the 23 that subtitles write apart more often (ক্ট একটা, প্ন আপনি, ম্র আমরা, ল্ল বললে, স্ল আসলে,
ম্ন সামনে, ল্ম, গ্ল, ন্ব, স্র, হ্ল, ক্ব, ল্ক, ম্ল, …); plus ষ্ণ, হ্র, শ্ল, whose "apart" words carry a
typed vowel (ঘোষণা, শহরে, কৌশল); plus six rare family pairs never written apart (ম্ফ শ্ছ স্খ ঞ্ঝ জ্ঝ
চ্ঞ), giving the report's 101; plus six rare Sanskrit pairs the author added, whose apart words carry a
typed vowel (হ্ণ গ্রহণ, দ্ঘ, ধ্ন সাধনা, ণ্ম, ঙ্ম, ষ্ফ). Rare pairs that would break common words stay out:
ল্গ (ফুলগুলো), দ্গ (বিপদগুলো), প্স (ঝাপসা, উপসাগর), ল্ফ (আলফা, সেলফি), ঘ্ন (মেঘনা), গ্ম (বেগম).

Pairs that never join follow from grammar, which is why they are absent: a breathy letter before
anything but its ফলা (দেখতে, বুঝতে: zero joined tokens), য় ড় ঢ় ৎ (জায়গা, হয়তো), হ except as a ফলা base
(তাহলে), ত except before ত থ ন ম ব য র (ত্ক/ত্স/ত্প in the corpus are misencoded ৎ).

The table is not a deny list, so no pair is ever joined by a missing entry: absent means apart.

### D4. Aspiration re-decides only by removing a hasant
`aspirated_consonant` rewrites the last consonant in place. When that consonant was joined and the
new pair is not in the table (and the cluster doesn't start a word), the rule also deletes the hasant:
`akt` → আক্ত, then `h` → আকথ. This is the same kind of edit `khanda_to` already makes (`ক্ত` + `H` →
`কৎ`), inside the open cluster, caused by the key just typed.

It never adds a hasant to a pair written apart (`ekS` → একশ, then `h` → একষ, not এক্ষ): that would need
the letter before the buffer, and ক্ষ already has its own key sequence (`kkh`).

### D5. The backtick
- After a consonant: append `্` and keep it in the buffer, exactly like the armed reph of `rr-reph`
  D2. The next consonant follows the hasant (`process_consonant` writes no extra hasant after a
  non-consonant), so the join needs no table lookup.
- After র: identical to a second `r`, so ``kor`ta`` → কর্তা and ``r`i`` → ঋ. One mechanism, two keys.
- A second backtick: append ZWNJ and end the cluster, giving the Bangla Academy's visible hasant
  (উদ্‌দীন).
- A vowel after the hasant: remove the hasant, then process the vowel as after the consonant
  (`rr-reph` D3).
- With no consonant before it: pass through as today (unmapped key).
- Backspace on the hasant removes only the hasant (letter Backspace already treats a lone trailing
  hasant as one letter); in the composer the next consonant then starts a new letter
  (whole-word-pending D3).

Why the backtick: the author's choice. `q` (ক) and `x` (ক্স) are taken, so Khipro's `qq`/`xx` can't
be used. It is needed only for the ~20 dropped pairs inside a word (ডক্টর, স্বপ্ন, সম্রাট, আল্লাহ,
উল্লেখ, ফিল্ম, নিম্ন, হিংস্র, ফাল্গুন), never at the start of a word.

*Alternative:* join every real conjunct and use only `o` (no join key). Rejected in review: words
right falls to 75% / 85%, because আমরা, একটা, আপনি, তাহলে, আসলে, সামনে would all need `o`.

### D6. `w`, `b`, `z`, `y`
- `w` after a consonant: append `্ব`, whatever the table says (`pokw` → পক্ব, `somonwoy` →
  সমন্বয়). Elsewhere `w` is ব. This gives every ব-ফলা word a spelling without the backtick.
- `b` after a consonant follows D1 like any consonant (`dekhbe` → দেখবে, `biSbas` → বিশ্বাস).
- `z` is the letter য and never joins (`kz` → কয); a backtick before it still joins. য-ফলা is `y`.
- `y`: `ja_fala`, unchanged by this change (see `rr-reph` D4 for `y` after র).
- `j` stays জ and `z` stays য: standalone জ is 1.46× more frequent than standalone য in subtitles and
  2.15× in Wikipedia. Native romanizers write both as "j" ~85% of the time, so frequency decides.

### D7. How users learn it
The README and release notes explain the rule in this order. This is the text the author asked for:

> **How consonants join.** Druti never guesses: whether two consonants join depends only on the two
> letters and where they are in the word.
>
> 1. **A vowel separates.** Type `o` between consonants to keep them apart: `janote` জানতে.
> 2. **At the start of a word, consonants join:** `glas` গ্লাস, `skul` স্কুল, `prothom` প্রথম.
> 3. **Inside a word, they join only in family pairs**, the conjuncts you learned at school:
>    - a nasal before its own row: `kintu` কিন্তু, `ponc` পঞ্চ, `kompon` কম্পন
>    - স/শ/ষ before a stop: `rasta` রাস্তা, `koShTo` কষ্ট, `niScoy` নিশ্চয়
>    - the same letter twice: `pakka` পাক্কা, `sottor` সত্তর (except ল্ল)
>    - a letter before its breathy partner: `iccha` ইচ্ছা, `buddhi` বুদ্ধি
>    - ফলা after the letters that take it: `potro` পত্র, `Sukla` শুক্লা, `biSbas` বিশ্বাস, `atma` আত্মা
>    - ক্ষ, জ্ঞ, and a few more: `bhokto` ভক্ত, `Sobdo` শব্দ, `golpo` গল্প
>
>    Every other pair stays apart with no `o`: `korte` করতে, `dekhte` দেখতে, `amra` আমরা,
>    `ekTa` একটা, `apni` আপনি, `bolle` বললে.
> 4. **Special keys:** `rr` for reph (`korrta` কর্তা), `y` for য-ফলা (`bakyo` বাক্য), `w` for ব-ফলা
>    (`pokw` পক্ব).
> 5. **To join a pair that stays apart, put a backtick between:** ``Dok`Tor`` ডক্টর, ``al`lah``
>    আল্লাহ, ``som`raT`` সম্রাট. Two backticks show a hasant: ``ud``dIn`` উদ্‌দীন.
>
> In one line: *start of a word, join; inside a word, family pairs join; `o` separates; backtick
> joins.*

A full table of the 107 pairs, with one example word each, follows in an appendix, generated from
`JOINING_PAIRS` so the docs can't drift from the engine.

### D8. Default, version, order
- No setting, same reasoning as `rr-reph` D6; the change says explicitly that it overrides the
  `openspec/config.yaml` default-behaviour constraint.
- Lands after `rr-reph` (it relies on the buffer restart and armed-hasant mechanics), and ships with
  it in 2.0.0. Release notes extend the `rr-reph` table: `dekh`te`-style spellings disappear;
  `ekoTa` → `ekTa` still works; new spellings for dropped pairs (``Dok`Tor``).

### Behaviour and fixture changes
This change intentionally alters engine output. Fixtures edited in the same commit as the engine
change:
- `engine/unit.json`, `engine/words.json`, `engine/random.json`, `engine/transpile.json`: every case
  where a consonant key follows a consonant inside a word, or that types a backtick or `w` after a
  consonant, or `z` after a consonant. A one-off script (kept out of the repo) replays each case on
  the new engine, rewrites only those cases, refuses to change any other case, and prints the counts
  for the PR description.
- `engine/data.json`: gains `joining_pairs`.
- `composer/`: new cases for the backtick; existing cases change only where they type such a pair.

## Risks / Trade-offs

- **[Trade-off] Suffix forms of listed pairs need `o`** (জানতে `janote`, থাকতে `thakote`, আসবে
  `asobe`): most of the 7.6% of chat words the rules get wrong. No left-to-right rule can see a
  suffix; stem lists measured worse.
- **[Trade-off] ~20 real conjuncts need a backtick inside a word** (ডক্টর, স্বপ্ন, সম্রাট, আল্লাহ,
  উল্লেখ, ফিল্ম, নিম্ন). ল্ল is the clearest cost: formal text joins it 16:1. Kept apart because বললে,
  বললাম are far more common in everyday typing.
- **[Risk] "Family pairs" is a list, not a closed rule**, notably the ফলা pairs and the 14 listed
  pairs. → The docs give the family rule first and the generated table second; the table is small.
- **[Risk] Composer word-start after a caret move** treats the new pending text as a word start
  (D2). → Affects only the 44 start-only pairs typed into an existing word; documented.
- **[Risk] Aspiration can remove a hasant already shown** inside the open cluster (D4). → Same kind
  of edit as today's `H` → ৎ, caused by the key just typed; nothing before the cluster changes.
- **[Risk] Regenerated random fixtures hide an unintended change.** → The script refuses to touch
  cases outside the affected key patterns; the hosts property test runs on the new fixtures.

## Migration Plan

1. Land `rr-reph`, then this change with its fixtures, docs and the generated pair table.
2. Release 2.0.0 with the combined old→new spelling table.
3. Rollback is a revert; no data or settings involved.
