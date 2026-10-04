# A 101-pair whitelist fixes Druti's conjuncts

> **Decisions made after this report** (see `proposal.md` and `design.md`): `y` after a single `r`
> gives the ZWJ form র‍্য, so `poryonto` → পর‍্যন্ত and reph over য is typed `rr` (this report's last
> section recommended পর্যন্ত; the author chose "only `rr` makes reph"). The force-join key is the
> backtick (O1). The table favours everyday writing (O2). Every pair joins at the start of a word,
> rather than a broader list (O3). `w` after a consonant is ব-ফলা (O4). Six rare Sanskrit pairs were
> added: হ্ণ দ্ঘ ধ্ন ণ্ম ঙ্ম ষ্ফ (O6), giving 107 pairs. The word-start accuracy figures here
> came from a script with a bug in its word-start check; `design.md` has the corrected numbers.

Bengali grammar has no closed, generative rule for which consonants may join. The Bangla Academy's spelling rules only *restrict*: no doubling after reph, no ষ/ণ in foreign words, ং in sandhi. The productive regularities (homorganic nasal + stop, sibilant + stop, gemination, the phalas, ক্ষ/জ্ঞ) are inherited Sanskrit phonology, and usage decides which pairs actually occur. Even so, a deterministic "join only if whitelisted" policy can be built and checks out well. I took the grammar classes, kept the pairs that the conjunct inventories agree on, and dropped those that corpus text writes apart more often than joined. That leaves **101 pairs, every one of which is already in Khipro's conjunct table**. On subtitle text this list gets **92.9% of consonant-pair occurrences and 92.3% of words right**. Today's join-everything engine gets **51% / 47%**, and **61% / 58%** once the pending `rr`-reph change lands. The list scores within 0.1 points of the best possible frequency-tuned list. Most remaining errors are verb and classifier suffixes on stems that end in a joinable pair (জানতে, থাকতে, আসবে, কোনটা). No rule that only sees the keys typed so far can fix those, and a verb-stem list loses more than it gains. On the two smaller questions, the data supports the current mappings: roman `j` should stay জ (জ is 1.46–2.15× more frequent than standalone য), and `poryonto` should give পর্যন্ত, because র+য is joined in 99% of corpus occurrences. On that last point Druti departs from Avro and Khipro, where `ry`/`rz` produce the ZWJ form র‍্য.

Provenance tags used below: **[code]** = read from source or confirmed by running the engine; **[corpus]** and **[font]** = measured from downloaded data; **[snippet]** = backed only by a search-result excerpt; **[BK]** = standard grammar from background knowledge, not checked against a primary source in this research. Figures marked "computed for this report" come from re-running the research corpus data with the whitelists described here.

## Grammar supplies the classes; usage supplies the members

No official body publishes an exhaustive list of permitted conjuncts. The Bangla Academy rules that touch clusters are all restrictive. After a reph, a consonant is not doubled (অর্চনা, অর্জন, not অর্চ্চনা) ([sattacademy, snippet](https://sattacademy.com/question/zlzehwj3P0)). Foreign words avoid ষ and ণ, so স্টেশন and কর্নার are correct ([teachers.gov.bd, snippet](https://teachers.gov.bd/blog/details/845266)). Before ক-বর্গ, a sandhi-final ম becomes ং, with ঙ also accepted (সংগীত / সঙ্গীত) ([sattacademy, snippet](https://sattacademy.com/question/satt-image-ayy5etinbk)). Teaching material defines a যুক্তবর্ণ and gives example lists such as ক্ষ, হ্ম, ঞ্চ, ক্ক and ক্ত, with no generative rule ([sattacademy guide, snippet](https://sattacademy.com/guide/যুক্তবর্ণ)). **I found no official closed inventory.** The Academy PDFs and NCTB grammar could not be opened, so the full rule paraphrases are [BK].

What does predict most pairs is Sanskrit place-of-articulation logic, all [BK]. A nasal joins the stops of its own বর্গ (ঙ্ক, ঞ্চ, ণ্ট, ন্ত, ম্প). A sibilant agrees with the stop after it (শ্চ, ষ্ট, স্ত). Unaspirated stops double, and a doubled aspirate is written unaspirated + aspirate (চ্ছ, ত্থ, দ্ধ). য, র, ল, ব, ম and ন stack as phalas. Reph goes over almost anything. ক্ষ and জ্ঞ are frozen units. The ণত্ব rule (ণ before ট-বর্গ in তৎসম words, never ণ্ত) is backed by a snippet ([sattacademy](https://sattacademy.com/mcq/%E0%A6%95%E0%A7%8B%E0%A6%A8-%E0%A6%AC%E0%A6%B0%E0%A7%8D%E0%A6%97%E0%A7%80%E0%A7%9F-%E0%A6%A7%E0%A7%8D%E0%A6%AC%E0%A6%A8%E0%A6%BF%E0%A6%B0-%E0%A6%86%E0%A6%97%E0%A7%87-%E0%A6%A8-%E0%A6%B8%E0%A6%AC%E0%A6%B8%E0%A6%AE%E0%A7%9F-%E0%A6%AE%E0%A7%82%E0%A6%B0%E0%A7%8D%E0%A6%A7%E0%A6%A8%E0%A7%8D%E0%A6%AF-%E0%A6%A3-%E0%A6%B9%E0%A7%9F)). The Academy's origin-based rules add the loan pairs ন্ট/ন্ড and স্ট. Two facts matter for an IME. First, these rules over-generate. Second, pronunciation does not predict joining. Bengali drops the inherent vowel in করতে and বলতে but writes those words unjoined and without a hasant. An unmarked consonant is ambiguous between C and C + /ɔ/, and Bengali does not mark the suppressed schwa ([Johny & Jansche, SLTU 2018](https://www.isca-archive.org/sltu_2018/johny18_sltu.pdf)). A conjunct is lexical, so the engine must never join a pair just because the vowel is silent.

The inventories that exist agree on a core and differ at the edges. I compared three machine-readable lists:

- bn.wiktionary's "বাংলা যুক্তবর্ণের তালিকা", as shipped in `bnunicodenormalizer`: 352 entries, 215 distinct adjacent pairs ([PyPI](https://pypi.org/project/bnunicodenormalizer/)) [code].
- `bkit`'s `_valid_conjunct_pairs`: 202 pairs ([PyPI](https://pypi.org/project/bkit/)) [code].
- Avro's pattern table: 188 virama-producing patterns, about 120 pairs ([PyAvroPhonetic](https://pypi.org/project/PyAvroPhonetic/)) [code].

Of the 122 true-conjunct pairs these lists name, **106 are in all three**. I shaped all 35×35 consonant pairs in 15 open fonts with HarfBuzz [font]. Every font builds reph and phala generically for every consonant. They differ on true conjuncts: 120–168 pairs in most fonts, and only 61 in the monospace Mitra Mono. They also ligate about 20 Sanskrit pairs that no list or corpus uses (ক্ন, গ্দ, ব্ভ). So fonts over-generate and are useful only as a negative check. A listed pair that no font joins (ঙ্ষ, স্চ, ন্শ, ল্চ) is suspect, and none of the 15 fonts joins the খণ্ড-ত misencodings ত্ক, ত্স, ত্প ([Google Fonts](https://fonts.google.com/?subset=bengali); [Ubuntu fonts-beng-extra](https://packages.ubuntu.com/noble/fonts-beng-extra)).

## 101 pairs survive grammar, lists, fonts and corpus

I built the whitelist in four steps:

1. Start from the inventory's 115 "core" pairs: true conjuncts plus র-ফলা and ব-ফলা pairs, each found in two or more lists with at least 20 corpus tokens.
2. Remove the 23 pairs that subtitle text writes apart more often than joined.
3. Put back ষ্ণ, হ্র and শ্ল. Their "apart" words (ঘোষণা, শহরে, কৌশল) carry a vowel that typists actually type: in the Dakshina romanizations, ষ+ণ had a silent vowel 0 times out of 149 ([Dakshina](https://github.com/google-research-datasets/dakshina)).
4. Add six rare Sanskrit pairs that are never written apart.

The result maps cleanly onto grammar classes. Reph (র + C, typed `rr`) and য-ফলা (C + `y`) remain productive and are not listed pair by pair.

| Rule class | Basis | Pairs |
|---|---|---|
| Homorganic nasal + stop (19) | varga agreement [BK] | ঙ্ক ঙ্খ ঙ্গ ঙ্ঘ ঞ্চ ঞ্ছ ঞ্জ ঞ্ঝ ণ্ট ণ্ঠ ণ্ড ন্ত ন্থ ন্দ ন্ধ ম্প ম্ফ ম্ব ম্ভ |
| ন + ট-বর্গ in অ-তৎসম words (3) | Academy ণ/ন rule [BK, snippet] | ন্ট ন্ঠ ন্ড |
| Gemination (12) | [BK] | ক্ক চ্চ জ্জ ট্ট ড্ড ণ্ণ ত্ত দ্দ ন্ন প্প ব্ব ম্ম |
| Unaspirated + own aspirate (4) | [BK] | চ্ছ জ্ঝ ত্থ দ্ধ |
| Sibilant + stop (13) | ষত্ব; স্ট for loans | শ্চ শ্ছ ষ্ক ষ্ট ষ্ঠ ষ্প স্ক স্খ স্ট স্ত স্থ স্প স্ফ |
| Frozen (2) | [BK]; Druti types `kkh`, `gg` | ক্ষ জ্ঞ |
| র-ফলা (14) | phala | ক্র খ্র গ্র ঘ্র ট্র ড্র ত্র দ্র প্র ফ্র ব্র ভ্র শ্র হ্র |
| ল-ফলা (5) | phala | ক্ল প্ল ফ্ল ব্ল শ্ল |
| ব-ফলা (7) | phala | জ্ব ত্ব দ্ব ধ্ব শ্ব স্ব হ্ব |
| ম-ফলা (4) | phala | ত্ম ন্ম ষ্ম স্ম |
| ন/ণ-ফলা (6) | phala, ণত্ব | গ্ন ত্ন শ্ন স্ন হ্ন ষ্ণ |
| Listed heterorganic (12) | sandhi and loans; list only | ক্ত ক্স গ্ধ চ্ঞ দ্ভ ন্স প্ট প্ত ব্দ ল্ট ল্ড ল্প |

Two cross-checks support the list. All 101 pairs appear in Khipro's 179-pair table, and every one except the seven ব-ফলা pairs appears in Avro's table. Avro types ব-ফলা by rule with `w` ([okkhor](https://github.com/gulshan/okkhor); [avrophonetic.json](https://github.com/OpenBangla/OpenBangla-Keyboard/blob/master/data/avrophonetic.json)) [code]. Its pairwise form also loses nothing on longer clusters. Every 3- and 4-member cluster in the lists, and every one found in the corpus, breaks down into adjacent pairs that are themselves in the inventory: ন্ত্র, স্ত্র, ন্ধ্য, ত্ত্ব, উজ্জ্বল's জ্জ্ব [corpus]. Putting ষ্ণ back keeps তীক্ষ্ণ and কৃষ্ণ working.

**The deny classes** need no list, because each is a rule backed by corpus counts:

- **Nukta letters and ৎ never join.** য়+গ (জায়গা) and য়+ত (হয়তো) appear 2,832 and 2,793 times apart in subtitles and never joined. Druti joins them today.
- **An aspirate joins only as a phala base.** খ+ত (দেখতে), ঝ+ত (বুঝতে), খ+ন (এখনো), থ+ম (প্রথমে) and খ+ব (দেখবে) have zero joined occurrences in subtitles. This one class covers most native verb stems that end in an aspirate.
- **ঙ and ঞ join only within their own বর্গ.**
- **ত joins only ত থ ন ম ব য র.** The 61/40/25 corpus tokens of ত্ক/ত্স/ত্প are misencoded চিৎকার/চিকিৎসা/উৎপাদন ([FrequencyWords](https://github.com/hermitdave/FrequencyWords)) [corpus].
- **Doubled aspirates, র্র and হ্হ never occur.**
- **হ joins only as a phala base.** তাহলে alone accounts for 6,621 apart occurrences.
- **Everything not listed is written apart.**

**Exception candidates** are the joined side of the 23 pruned pairs plus the rare Sanskrit tail. Each needs an explicit join key, or a positional rule (below), rather than a place on the default list.

| Pruned pair | Subs joined / apart | Wiki joined / apart | Lost joined words | Words it protects |
|---|---|---|---|---|
| ক্ট | 1,535 / 25,459 | 18,281 / 111,855 | ডক্টর, অক্টোবর, ভিক্টর, ডিরেক্টর | একটা, একটি, একটু |
| প্ন | 989 / 25,283 | 1,441 / 10,971 | স্বপ্ন | আপনি, আপনার |
| ম্র | 206 / 20,565 | 9,073 / 11,682 | সম্রাট, সাম্রাজ্য | আমরা, তোমরা |
| ল্ল | 1,586 / 2,073 | 30,939 / 1,966 | আল্লাহ, উল্লেখ, কুমিল্লা | বললে, বললাম |
| স্ল / ম্ন / ল্ম / গ্ল | small | mixed | স্লিপ, নিম্ন, ফিল্ম, গ্লাস | আসলে, সামনে, গোলমাল, লাগলো |
| ল্ক / ন্ব / ব্ধ / দ্ম / হ্ম / স্র / জ্র / শ্ম | small | mixed | উল্কা, সমন্বয়, উপলব্ধি, পদ্ম, ব্রহ্ম, হিংস্র, বজ্র, শ্মশান | কালকে, জানবে, সাবধান, বদমাশ, রহমান, সরাসরি |

The rare tail that every list includes but that is written apart in practice (দ্গ, দ্ঘ, গ্ম, ক্ম, ল্গ, প্স, ঘ্ন, হ্ণ, হ্ল, ম্ল, ল্ফ, ঙ্ম, ণ্ম, ধ্ন) belongs here too. Khipro writes several of these with a visible hasant by design (দ্‌গ, দ্‌ঘ, ল্‌ভ, গ্‌ণ), following the Academy's transparent-conjunct style ([Khipro docs](https://github.com/khiproteam/khipro/blob/main/content/documentation/_index.md)) [code]. ল্ল is the clearest register conflict. Formal text joins it about 16 to 1, while chat text writes it apart more often because of বললে.

## The whitelist nearly doubles correct words, but suffixes stay out of reach

About 50–75 pairs are truly ambiguous: common in both a joined and an apart form with an unwritten vowel. The heaviest are ন+ত, ক+ত, স+ত, স+ব, ন+ট, ক+ল and the র+X pairs. Their apart side is dominated by inflection. **61.9% of all apart occurrences in subtitles sit at a suffix boundary** (-তে/-তাম, -বে/-বো, -লে/-লাম, -ছে, -টা/-টি, -কে, -দের). In Wikipedia text the figure is 26.0% ([FrequencyWords](https://github.com/hermitdave/FrequencyWords); [Dakshina](https://github.com/google-research-datasets/dakshina)) [corpus].

The comparison below scores each policy on joined positions (C1্C2) and apart positions (C1C2 between vowels, with a silent inherent vowel), weighted by word frequency. It was computed for this report. Reph pairs are assumed to be typed correctly with `rr`, and C+য is excluded because `y` and `z` already separate the two cases.

| Policy | Subs pairs | Subs words | Wiki pairs | Wiki words |
|---|---|---|---|---|
| Druti 1.x: join every pair | 51.2% | 46.9% | 71.0% | 65.3% |
| Druti 2.0 `rr`-reph, join the rest | 61.3% | 57.9% | 75.5% | 70.6% |
| Inventory core list, unpruned (115) | 80.4% | 78.6% | 89.4% | 87.1% |
| **Proposed whitelist (101)** | **92.9%** | **92.3%** | **92.0%** | **90.3%** |
| Frequency-optimal list (106) | 93.0% | 92.4% | 92.2% | 90.5% |
| Proposed + suffix exception (needs lookahead) | 96.0% | 95.6% | 93.1% | 91.7% |

The proposed list does as well as an oracle list fitted to subtitle frequencies. The five extra pairs in the oracle list are ত্স/ত্প misencodings and one-off typos, so frequency adds nothing that the grammar classes and inventories did not already capture. The research notes measured the same thing including C+য and got 58.1% for join-all and 93.95% for a frequency whitelist with `rr`-reph ([FrequencyWords](https://github.com/hermitdave/FrequencyWords)) [corpus].

The real accuracy is somewhat higher than the table shows. The apart heuristic counts words such as উপরে, ঘোষণা and ধারণা, whose vowel is spoken and typed (র+ণ silent in 8 of 542 romanizations, প+র in 12 of 388) ([Dakshina lexicons](https://github.com/google-research-datasets/dakshina)) [corpus].

The residual errors are concentrated. Words that come out wrongly joined are থাকতে (1,697), জানতে (1,282), আসবে (1,266), আসতে (1,168), শুনতে (1,025), একসাথে (704), থাকলে (559), মানুষকে (311), জিনিসটা (308) and কোনটা (256). Words that come out wrongly apart are স্বপ্ন (489), আল্লাহ (287), ডক্টর (219), গ্লাস (104) and উল্লেখ (80). A suffix exception would fix most of the first group, but it needs the letters *after* the pair. Druti's `rr`-reph design forbids that: every decision is made by the key being typed, and shown text never changes ([design D1](../archive/2026-10-03-rr-reph/design.md)) [code].

The only key-so-far alternative is to check the stem typed so far, and it fails. The top 20 stems (আস, থাক, জান, শুন, মানুষ, আন, জিনিস, কোন…) cover 65% of the 21,044 suffix-boundary errors in subtitles, recovering about 13,700 occurrences. But they would block **17,370** correct joins in subtitles: কিন্তু (10,481), চিন্তা, ডাক্তার, আস্তে. In Wikipedia text they would block **51,699**, including আন্তর্জাতিক with 9,223 (computed for this report). So typists must type `o` in জানতে-type verb forms (`janote`). The data puts a ceiling of about 3 points on what any lookahead could add.

A positional rule is a small gain that looks safe. At the start of a word, the first syllable's inherent vowel is pronounced, so typists type it (গলা is `gola`). Two consonants typed there with no vowel between them can therefore only be a cluster, and the check can use the broader list (গ্লাস, স্লিপ, স্রষ্টা, ম্লান). That raises joined-pair accuracy from 97.8% to 98.7% in subtitles and from 96.6% to 97.4% in Wikipedia (computed for this report). It adds no errors on the medial apart positions the simulation measures, but it was not measured on word-initial apart words, where it depends on the [BK] claim that the first-syllable vowel is never silent.

## Avro, Khipro and Wikimedia already run whitelists

Every surveyed phonetic engine except ITRANS joins only listed pairs and writes everything else apart, so করতে needs no separator.

**Avro** joins by longest match over about 120 explicit pairs. It adds productive phalas (`r` → ্র and `y`/`Z` → ্য after any consonant, `w` → ্ব) and `rr` for reph. Its rule output is করম for `kormo` and আম্রা for `amra`. A regex dictionary that treats every hasant as optional rescues those words as suggestions, and Riti ranks the raw rule output last ([avrophonetic.json](https://github.com/OpenBangla/OpenBangla-Keyboard/blob/master/data/avrophonetic.json); [avroregexlib.js](https://github.com/sarim/ibus-avro/blob/master/avroregexlib.js); [riti suggestion.rs](https://github.com/OpenBangla/riti/blob/master/src/phonetic/suggestion.rs)) [code]. A deterministic Druti cannot borrow that rescue.

**Khipro** is the closest precedent. Riti describes it as deterministic, "a given sequence of roman keystrokes maps to exactly one Bangla output" ([riti khipro/method.rs](https://github.com/OpenBangla/riti/blob/master/src/khipro/method.rs)). It has:

- 179 joinable pairs and 137 "impossible conjunct" pins;
- `;` to separate letters;
- `qq` for a bare hasant, which forces any join (`kilqqn` → কিল্ন);
- `xx` for a visible hasant + ZWNJ (`udxxdiin` → উদ্‌দীন);
- `/` to break a conjunct that has just formed.

Its documentation states Druti's principle almost word for word: a separator is needed only where a conjunct is possible (`likhte` → লিখতে, but `gol;pw` → গলপো) ([Khipro docs](https://github.com/khiproteam/khipro/blob/main/content/documentation/_index.md); [okkhor khipro.rs](https://github.com/gulshan/okkhor)) [code].

**Wikimedia's bn-avro** is incremental like Druti. It keys each join on the second consonant and the class of allowed first consonants, for example `([কতনপশসহ])t` → `$1্ত`, and a typed `o` or backtick blocks the join ([jquery.ime bn-avro.js](https://github.com/wikimedia/jquery.ime/blob/master/rules/bn/bn-avro.js)) [code]. **ITRANS** is the opposite convention and is what Druti 1.x resembles: every consonant without a vowel takes a virama, so `karte` gives কর্তে and the inherent vowel must be typed ([indic_transliteration](https://pypi.org/project/indic-transliteration/)) [code]. **Fixed layouts** write consonants apart unless a link key comes between them: `g` on Bijoy and National, `/` on Probhat, `h` on Avro Easy and Borno. Pressing the link key twice gives a visible hasant ([National_Jatiya.json](https://github.com/OpenBangla/OpenBangla-Keyboard/blob/master/data/National_Jatiya.json); [Probhat.json](https://github.com/OpenBangla/OpenBangla-Keyboard/blob/master/data/Probhat.json); [riti fixed/method.rs](https://github.com/OpenBangla/riti/blob/master/src/fixed/method.rs)) [code]. Bijoy's `g` rests only on a government teachers' portal ([teachers.gov.bd, snippet](https://teachers.gov.bd/index.php/content/details/934855)).

| IME | Unlisted pair | Separate | Force join | Visible hasant | Reph | য-ফলা | জ / য |
|---|---|---|---|---|---|---|---|
| Avro Phonetic | apart | `o`, `` ` `` | none | `,,` | `rr` | `y`, `Z` | `j` / `z` |
| Khipro | apart | `;`, `o` | `qq` | `xx` | `rr` | `z` (and `r` for র-ফলা) | `j` / `z` |
| Wikimedia bn-avro | apart | `o`, `` ` `` | `,,` (bare ্) | — | `rr` (retroactive) | `y` | `j` / `z` |
| ITRANS | always join | — | — | — | implicit | `y` | `j` / `y` |
| Fixed layouts | apart | — | link key | link ×2 | key | key or link | layout keys |
| Druti 2.0 (planned) | join | `o` | — | — | `rr` | `y` | `j` / `z` |

Druti can borrow Khipro's model nearly whole: a pair table, `rr` reph, productive phalas, one force-join key and the same key doubled for a visible hasant. It cannot borrow Khipro's keys. Druti maps `q` → ক and `x` → ক্স ([data.rs](../../../crates/druti-core/src/data.rs)) [code], so `qq` and `xx` collide. Khipro's slicer `/` also undoes a conjunct already on screen, which breaks Druti's rule that shown text never changes. One engine detail matters for an incremental design. Khipro pins digraphs such as `kth` → কথ and `pth` → পথ so that a joinable first pair does not swallow an `h`. In Druti, aspiration rewrites the last letter (ক্ত + `h` → ক্থ), so the whitelist check must run again on the aspirated pair inside the open cluster.

## Keep `j` for জ, and keep `y` after র as reph + য

**জ is the more frequent standalone letter in both registers.** In subtitles, standalone জ appears 126,006 times against 86,476 for standalone য (1.46×). In Wikipedia text the counts are 804,742 against 375,000 (2.15×). Counting the keystrokes each key would carry (every জ except জ্ঞ, versus standalone য + য্য + র্য), `j` still leads by 1.43× and 2.06× ([FrequencyWords](https://github.com/hermitdave/FrequencyWords); [Dakshina](https://github.com/google-research-datasets/dakshina)) [corpus].

The case for giving `j` to য rests only on word-initial function words in conversation. যে, যদি, যা, যাও, যখন and যায় make standalone য lead word-initially, 81,145 to 65,065. But জ's medial uses (জন্য, কাজ, একজন, নিজের, আজ) outweigh them. Most য in text is য-ফলা (113,662 in subtitles), which Druti types with `y` either way. Native romanizers write *both* letters as "j" about 85% of the time (জ 84.4%, য 86.5%), so there is no intuitive split to honour ([Dakshina lexicons](https://github.com/google-research-datasets/dakshina)) [corpus]. Druti's `j` → জ, `z` → য matches the data and also Avro, Khipro, Probhat and Borno, which eases migration ([data.rs](../../../crates/druti-core/src/data.rs); [avrophonetic.json](https://github.com/OpenBangla/OpenBangla-Keyboard/blob/master/data/avrophonetic.json)) [code].

For `poryonto`, the corpus is decisive. **র+য is joined 4,648 times against 35 apart in subtitles, and 65,659 against 831 in Wikipedia text** (পর্যন্ত, সূর্য, পর্যায়ে, কার্যক্রম). The apart side is compounds such as নির্ভরযোগ্য, which `z` already produces [corpus]. Only 1,245 Wikipedia occurrences use the ZWJ form র‍্য (র‍্যাব), about 2% of joined র+য. The subtitle list had its joiners stripped, so it cannot be measured there.

The other IMEs choose differently:

- **Avro:** `ry` gives the ZWJ form র‍্য, so `poryonto` gives পর‍্যন্ত. Users must type `porrzonto`, and Avro's autocorrect maps `porjonto` → `porrzonto` ([avrophonetic.json](https://github.com/OpenBangla/OpenBangla-Keyboard/blob/master/data/avrophonetic.json); [autocorrect.json](https://github.com/OpenBangla/OpenBangla-Keyboard/blob/master/data/autocorrect.json)) [code].
- **Khipro:** documents `rz` = র‍্য and `porrzonto` = পর্যন্ত.
- **Riti's fixed-layout engine:** inserts ZWJ when র is not itself a র-ফলা ([riti fixed/method.rs](https://github.com/OpenBangla/riti/blob/master/src/fixed/method.rs)) [code].
- **Wikimedia bn-avro:** its cited rule `([ক-হড়ঢ়য়])y` → `$1্য` has no ZWJ case, which yields reph + য.
- **ITRANS:** joins everything, so `ry` also gives র্য.

Druti's planned rule, `y` after র → ্য (পর্যন্ত), with `z` as the apart letter (পরযন্ত) and `rr` + `y` also giving কার্য ([proposal](../archive/2026-10-03-rr-reph/proposal.md)), is the one exception to "a single `r` never forms reph". The 99% join rate justifies it. Druti still has no way to type র‍্যাব.

## Recommendations and open decisions

The recommendations follow directly from the evidence above. Every rule decides from the key being typed and the open cluster only, which matches the `rr`-reph design.

| # | Recommendation | Evidence |
|---|---|---|
| R1 | Replace join-all with the 101-pair table above, stored in `druti-core/src/data.rs` and pinned by fixtures. A non-listed pair is written apart. | Corpus simulation [corpus]; table ⊂ Khipro, ⊂ Avro except ব-ফলা [code] |
| R2 | Keep reph (`rr`) and য-ফলা (`y`) productive; `z` after a consonant never joins. | র+য 99% joined; X+য apart side is `-যোগ্য` compounds [corpus] |
| R3 | Encode the deny classes as rules, not list entries: nukta letters and ৎ; aspirate + non-phala; ঙ/ঞ outside their বর্গ; ত + non-{ত থ ন ম ব য র}; হ + non-phala. | Zero-join counts (দেখতে, বুঝতে, জায়গা); ত্ক/ত্স/ত্প misencodings [corpus, font] |
| R4 | Add one force-join key for exceptions (ডক্টর, স্বপ্ন, সম্রাট, আল্লাহ, ফিল্ম, উদ্গত), and make the same key pressed twice give hasant + ZWNJ. | Khipro `qq`/`xx`, fixed-layout link ×2, Avro `,,` [code] |
| R5 | Re-check the pair whenever aspiration rewrites C2 inside the cluster (`kth` → কথ, not ক্থ). | Khipro digraph pins [code] |
| R6 | Keep `j` → জ and `z` → য. | 1.43–2.06× keystroke share [corpus]; all major IMEs agree [code] |
| R7 | Keep `nc` → ঞ্চ and `nj` → ঞ্জ. | ঞ্জ 1,593 vs ন+জ apart 293; ঞ্চ 1,115 vs 192 (subs) [corpus]. The cost is that আইনজীবী and মানচিত্র need `o`. |
| R8 | Ship no suffix exception and no stem list. Document `o` for জানতে-type verbs (`janote`). | Stem list blocks more correct joins than it fixes [corpus] |

Some choices are the author's. The evidence narrows each one but does not settle it.

| # | Open decision | Options and evidence |
|---|---|---|
| O1 | Which key forces a join? | Backtick, `,,` or `;` are free in Druti; `qq`/`xx` collide with `q`=ক and `x`=ক্স. Backtick was part of the rejected "smart hasant" draft (rejected for its word list, not for the key), but in Avro and Wikimedia backtick *separates*, so Avro users may expect the opposite. |
| O2 | Which register to tune for? | The list is chat-tuned. A formal-tuned list would keep ল্ল (wiki 30,939 / 1,966), ক্ট (অক্টোবর) and ম্র (সম্রাট), at the cost of বললে, একটা and আমরা in casual typing. |
| O3 | Should a word-initial cluster use the broader list (গ্লাস, স্লিপ, স্রষ্টা)? | +0.9 pt joined accuracy, no measured apart cost; rests on [BK] that the first syllable's vowel is always typed. |
| O4 | Should `w` after a consonant always mean ব-ফলা, Avro-style, while `b` follows the table? | It gives পক্ব, বিল্ব and সমন্বয় a natural spelling. Today `w` → ব exactly like `b`. |
| O5 | How is র‍্য (ZWJ) typed? | Needed for র‍্যাব and র‍্যালি (about 2% of র+য in wiki). Avro and Khipro use `ry`/`rz`, which Druti spends on পর্যন্ত. |
| O6 | Should rare Sanskrit pairs (দ্গ, গ্ম, প্স, হ্ণ, ল্গ) default to joined, or require the force key? | Every list includes them, and they are almost never written apart in the corpus, but they are rare. Leaving them out costs little; Khipro writes দ্‌গ and দ্‌ঘ with a visible hasant. |

**Evidence quality.** Verified from primary data:

- IME behaviour: source files, plus probe runs of the `okkhor` crate [code].
- Every corpus figure: FrequencyWords subtitles (2.4 M tokens) and Dakshina Wikipedia (12.3 M tokens) and its romanization lexicon [corpus].
- List contents: PyPI packages [code].
- Font coverage: HarfBuzz over 15 fonts [font].
- Druti's current mappings and planned rules: this repository [code].

Resting on search snippets or background knowledge:

- All Bangla Academy and NCTB rule text [snippet/BK].
- The Sanskrit rule classes [BK].
- The Bijoy link key [snippet].
- The claim that the first syllable's vowel is never silent [BK].
- Behaviour of Gboard, Ridmik, Microsoft and Lipika, which was not reached at all.

The corpora have their own limits:

- The subtitle list has no ZWJ/ZWNJ.
- Wikipedia contains some transliterated noise (ব্জাং).
- No news corpus was reachable.
- Triples were scored pairwise.

The simulation scripts used for the "computed for this report" figures are in this session's scratchpad (`sim_report.py`, `final.py`, `stems.py`, `initial.py`). They read the same corpus data as the research notes' `pair_stats.csv`.

## Conclusion

The grammar question has a better answer than "there are no rules". Bengali has no rule that *licenses* a pair, but it has rules that *forbid* whole classes (aspirate + stop, nukta letters, ত outside its seven partners), and those deny classes carry much of the gain. The ambiguous zone turns out to be mostly morphology rather than phonology. Druti's remaining errors are verb and classifier suffixes, which no left-to-right rule can see and which a stem list makes worse. So the honest design target is about 92–93% of words with no keystroke changes. The rest is covered by teaching `o` for জানতে-type verbs and one force-join key for loanwords and Sanskrit forms.

The second surprise is how little frequency tuning adds. A list built from grammar classes plus the inventories that already exist, with corpus data used only to prune, lands within 0.1 points of the frequency-optimal list. Druti's conjunct table can therefore be justified pair by pair from grammar and published lists, rather than from a corpus snapshot, which makes it easier to review and to pin with golden fixtures.
