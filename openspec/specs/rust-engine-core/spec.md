# rust-engine-core Specification

## Purpose
The engine that turns phonetic roman keystrokes into Bengali, keystroke by keystroke. It is the only
implementation of Druti's algorithm, shared by the macOS input source, the web playground and later
an iOS keyboard, and its behaviour is pinned by golden fixtures.
## Requirements
### Requirement: Keystroke behaviour
For any sequence of keys, and for any optional text-before-caret supplied with each key, the engine SHALL
produce the edit actions (insert, replace with a back count, delete with a back count, block split), the
accumulated output and the pending buffer after every key that the committed golden fixtures record for
that sequence with default settings. The Rust engine is the only implementation and the reference for
this behaviour.

#### Scenario: Aspiration rewrites the previous consonant
- **WHEN** the keys `k` then `h` are processed
- **THEN** the second key yields a single replace action of 1 unit with text `খ`, and the output and buffer are both `খ`

#### Scenario: Conjunct built by deleting and reinserting
- **WHEN** the keys `k`, `k`, `h` are processed
- **THEN** the third key yields a delete of 3 units followed by an insert of `ক্ষ`, and the output and buffer are both `ক্ষ`

#### Scenario: Vowel after a consonant becomes a kar
- **WHEN** the keys `a`, `m`, `i` are processed
- **THEN** the output is `আমি` and the buffer is empty

#### Scenario: Silent o breaks kar attachment
- **WHEN** the keys `k`, `o`, `a` are processed
- **THEN** the output is `কআ`

#### Scenario: Numbers and punctuation
- **WHEN** the keys `a`, `.` are processed on a fresh engine
- **THEN** the output is `আ।`

#### Scenario: Decimal point after a digit
- **WHEN** the keys `1`, `.`, `5` are processed on a fresh engine
- **THEN** the output is `১.৫`

#### Scenario: Unmapped keys pass through
- **WHEN** the keys `k`, `?`, `k` are processed
- **THEN** the output is `ক?ক`

### Requirement: Document-aware vowels
A vowel key SHALL produce the dependent vowel sign (kar) only when the character immediately before
the caret (ignoring one trailing combining nukta) is a consonant that can carry a kar, and no silent
`o` came just before; otherwise it SHALL produce the independent vowel. The text before the caret is
the supplied text-before-caret, or the engine's own output when none is supplied.

#### Scenario: Kar attaches to a consonant already in the document
- **WHEN** a fresh engine processes `i` with text-before-caret `ক`
- **THEN** it inserts `ি`

#### Scenario: No document context
- **WHEN** a fresh engine processes `i` with no text-before-caret
- **THEN** it inserts `ই`

### Requirement: Letter backspace
Engine-level backspace SHALL delete the last letter of the output, where a letter is:
- a Bengali consonant (a precomposed nukta letter counts as one), together with a combining nukta
  after it and, when it is joined to a previous consonant, the hasant before it;
- otherwise, a single other Bengali code point: a kar, a sign (`ং ঃ ঁ`), an independent vowel, a
  digit, `।`, or a lone hasant;
- for text that isn't Bengali, one extended grapheme cluster.

Backspace SHALL NOT leave a hasant at the end of the output unless one was already there before the
deleted letter. It SHALL return a single delete action whose back count is the letter's length in
UTF-16 code units. It does nothing on an empty output. It never undoes a keystroke that produced no
visible text, such as the silent `o`. After deleting, the engine SHALL resume the cluster from the
output (see "Resuming the cluster from the output") and clear the silent-`o` state.

#### Scenario: Backspace inside a conjunct
- **WHEN** the keys `k`, `k`, `h` are processed and then backspace
- **THEN** a delete of 2 units is returned, and the output and buffer are both `ক`

#### Scenario: Backspace on a two-consonant conjunct
- **WHEN** the keys `d`, `m` are processed and then backspace
- **THEN** a delete of 2 units is returned, and the output and buffer are both `দ`

#### Scenario: Aspirated consonant is one letter
- **WHEN** the keys `k`, `h` are processed and then backspace
- **THEN** a delete of 1 unit is returned, and the output and buffer are both empty

#### Scenario: Three-consonant conjunct loses one consonant at a time
- **WHEN** the output is assigned `ন্ত্র` and backspace is processed twice
- **THEN** the output is `ন্ত` after the first backspace and `ন` after the second

#### Scenario: Kar is removed on its own
- **WHEN** the keys `k`, `o`, `r`, `o`, `t`, `e` are processed and then backspace three times
- **THEN** the output is `করত`, then `কর`, then `ক`

#### Scenario: Silent o does not use up a backspace
- **WHEN** the keys `k`, `o` are processed and then backspace
- **THEN** a delete of 1 unit is returned and the output is empty

#### Scenario: Remainder continues the cluster
- **WHEN** the keys `d`, `m` are processed, then backspace, then `h`
- **THEN** the output is `ধ`

#### Scenario: Backspace after a resync
- **WHEN** the output is assigned `ক্ত` and then backspace is processed
- **THEN** a delete of 2 units is returned and the output is `ক`

#### Scenario: Emoji is one letter
- **WHEN** the output is assigned `ক👍🏽` and then backspace is processed
- **THEN** a delete of 4 units is returned and the output is `ক`

#### Scenario: Backspace on empty output
- **WHEN** backspace is processed on a fresh engine
- **THEN** no actions are returned

### Requirement: Resuming the cluster from the output
When the host asks the engine to resume the cluster, the engine SHALL make the trailing consonant run
of its output the current buffer. The run is the last consonant that can carry a kar (with its nukta),
plus any consonants joined to it by hasants before it. When the output doesn't end in such a
consonant, the buffer SHALL be empty. The next consonant key then continues the run exactly as if the
run had just been typed. Resuming SHALL NOT change the output or return actions. The engine SHALL
never resume on its own after it ends a cluster itself (a silent `o`, a kar, a space, punctuation, an
unmapped key or English mode).

#### Scenario: Aspiration continues a resumed consonant
- **WHEN** the output is assigned `করত`, the cluster is resumed, and `h` is processed
- **THEN** a replace of 1 unit with `থ` is returned and the output is `করথ`

#### Scenario: Conjunct continues a resumed consonant
- **WHEN** the output is assigned `দ`, the cluster is resumed, and `m` is processed
- **THEN** the output and buffer are both `দ্ম`

#### Scenario: Whole conjunct run is resumed
- **WHEN** the output is assigned `এন্ত`, and the cluster is resumed
- **THEN** the buffer is `ন্ত`

#### Scenario: Nothing to resume after a kar
- **WHEN** the output is assigned `কি`, and the cluster is resumed
- **THEN** the buffer is empty

#### Scenario: Signs are not resumed
- **WHEN** the output is assigned `বাং`, and the cluster is resumed
- **THEN** the buffer is empty

#### Scenario: Silent o still ends the cluster
- **WHEN** the keys `k`, `o`, `m` are processed
- **THEN** the output is `কম`

### Requirement: UTF-16 lengths
All back counts reported by the engine SHALL be measured in UTF-16 code units, matching the host
platforms' string ranges (NSString/NSRange on Apple platforms, JavaScript strings in the browser).

#### Scenario: Back counts match UTF-16 length
- **WHEN** any action with a back count is produced
- **THEN** the count equals the UTF-16 length of the text it removes

### Requirement: Bulk transpilation
Bulk conversion of a roman document SHALL produce the Bengali text recorded in the committed transpile
fixtures for the same input and line-break option, which is the text produced by feeding each character to the engine
as a keystroke (a newline as Enter when line breaks are kept).

#### Scenario: Multi-line document
- **WHEN** `ami banglay gan gai\nami banglar gan gai` is converted with line breaks preserved
- **THEN** the result equals the fixture's expected text, with the newline kept between the two sentences

### Requirement: Platform independence
The engine SHALL have no dependency on any operating system UI framework or I/O. It SHALL build and pass
its tests on Linux and on macOS, and it SHALL build for `wasm32-unknown-unknown`, so the macOS input
source, a later iOS keyboard extension and the web playground all use it.

#### Scenario: Linux test run
- **WHEN** the native test suite runs on a Linux machine
- **THEN** it builds and all golden fixtures pass

#### Scenario: WebAssembly build
- **WHEN** the engine is built for `wasm32-unknown-unknown`
- **THEN** it compiles without platform-specific code

### Requirement: Golden fixtures pin behaviour
The repository SHALL keep the committed fixture files that record the actions, output and buffer after
every key for (a) the unit cases, (b) a curated list of Bengali words and sentences, and (c) seeded
random key sequences drawn from every key the engine handles, plus the composer and bulk transpilation
fixtures. The engine test suite SHALL replay every fixture, and continuous integration SHALL fail when
any fixture fails to replay. Fixtures SHALL change only in the same commit as an intended behaviour change.

#### Scenario: Unintended behaviour change
- **WHEN** an engine rule changes and the fixtures are not updated
- **THEN** the fixture replay fails in CI

#### Scenario: Unicode edge cases stay covered
- **WHEN** the fixtures are replayed
- **THEN** they include text-before-caret ending in a decomposed nukta letter, in a surrogate pair (emoji), and in consonant + chandrabindu

### Requirement: A single r never joins the next consonant
When a consonant key follows a র that ends the current cluster, the engine SHALL write the consonant
after the র without a hasant and start a new cluster with it. This holds for a র typed as র-ফলা too.
`y` (য-ফলা) and a second `r` are handled by their own requirements.

#### Scenario: Infinitive
- **WHEN** the keys `k`, `o`, `r`, `t`, `e` are processed
- **THEN** the output is `করতে`

#### Scenario: Other verb forms and nouns
- **WHEN** `korbo`, `korlam`, `korchi` and `dorkar` are each typed on a fresh engine
- **THEN** the outputs are `করব`, `করলাম`, `করছি` and `দরকার`

#### Scenario: Former reph spelling
- **WHEN** the keys `k`, `o`, `r`, `t`, `a` are processed
- **THEN** the output is `করতা`

#### Scenario: র-ফলা is unchanged
- **WHEN** `prothom` is typed on a fresh engine
- **THEN** the output is `প্রথম`

#### Scenario: Explicit o still separates
- **WHEN** `korote` is typed on a fresh engine
- **THEN** the output is `করতে`

### Requirement: Double r makes reph
An `r` typed directly after a র that ends the current cluster SHALL append a hasant, giving a visible
`র্`. A consonant typed next SHALL follow that hasant, forming reph, and its cluster continues as
usual (aspiration, conjuncts). The keys SHALL NOT change any letter before the র.

#### Scenario: Reph appears as the consonant is typed
- **WHEN** the keys `k`, `o`, `r`, `r`, `t`, `a` are processed
- **THEN** the output is `কর` after the first `r`, `কর্` after the second, `কর্ত` after `t`, and `কর্তা` after `a`

#### Scenario: Aspiration after reph
- **WHEN** `orrtho` is typed on a fresh engine
- **THEN** the output is `অর্থ`

#### Scenario: Reph before a conjunct
- **WHEN** `dhorrmo` and `porrzonto` are each typed on a fresh engine
- **THEN** the outputs are `ধর্ম` and `পর্যন্ত`

#### Scenario: Third r
- **WHEN** `rrri` is typed on a fresh engine
- **THEN** the output is `র্রি`

#### Scenario: Armed reph at a word break
- **WHEN** the keys `k`, `o`, `r`, `r`, space are processed
- **THEN** the output is `কর্ `

### Requirement: A vowel after double r cancels the reph
A vowel key other than `i` typed directly after `র্` from a double `r` SHALL remove that hasant, and
SHALL then act exactly as the same vowel typed after the র. That includes the silent `o`.

#### Scenario: Vowel sign on র
- **WHEN** `korra` is typed on a fresh engine
- **THEN** the output is `করা`

#### Scenario: Silent o after double r
- **WHEN** `korrote` is typed on a fresh engine
- **THEN** the output is `করতে`

### Requirement: Double r and i make ঋ
An `i` typed directly after `র্` from a double `r` SHALL replace it with ঋ-kar when that র is joined
to a consonant before it (র-ফলা), and with the independent ঋ otherwise.

#### Scenario: Independent ঋ
- **WHEN** `rriN` is typed on a fresh engine
- **THEN** the output is `ঋণ`

#### Scenario: ঋ-kar
- **WHEN** `krriShi` and `brriShTi` are each typed on a fresh engine
- **THEN** the outputs are `কৃষি` and `বৃষ্টি`

### Requirement: y after r is য-ফলা
`y` typed after a র SHALL produce য-ফলা, and SHALL NOT form reph:
- after a র that stands alone (not র-ফলা, not armed), the engine SHALL write ZWJ + `্য`, giving the
  visible য-ফলা form `র‍্য` (U+09B0 U+200D U+09CD U+09AF);
- after a র-ফলা, it SHALL write `্য` with no ZWJ, as today;
- after an armed `র্` from a double `r`, it SHALL write `য`, giving reph over য (`র্য`).

`z` is the letter য and SHALL follow the `r` rules like any other consonant.

#### Scenario: য-ফলা after a single r
- **WHEN** `poryonto` and `ryab` are each typed on a fresh engine
- **THEN** the outputs are `পর‍্যন্ত` and `র‍্যাব`, each with a ZWJ after the র

#### Scenario: য-ফলা after র-ফলা
- **WHEN** `bryanD` is typed on a fresh engine
- **THEN** the output is `ব্র্যান্ড`, with no ZWJ

#### Scenario: Reph over য
- **WHEN** `porryonto`, `porrzonto` and `karryo` are each typed on a fresh engine
- **THEN** the outputs are `পর্যন্ত`, `পর্যন্ত` and `কার্য`

#### Scenario: The letter য
- **WHEN** `porzonto` is typed on a fresh engine
- **THEN** the output is `পরযন্ত`

### Requirement: Nothing joins after a breathy letter
When a consonant key follows a breathy letter (খ ঘ ছ ঝ ঠ ঢ থ ধ ফ ভ), হ, ড় (U+09DC), ঢ় (U+09DD) or
য় (U+09DF) at the end of the current cluster, the engine SHALL write the consonant without a hasant
and start a new cluster with it, except for the ফলা keys: `r` (র-ফলা), `l` (ল-ফলা), `m` (ম-ফলা), `n`
and `N` (ন/ণ-ফলা), `w` (ব-ফলা) and `y` (য-ফলা, unchanged). `b` is not a ফলা key here: it stays
apart. The decision uses the letter as it stands when the key is typed (`kh` is খ).

#### Scenario: Verb forms after a breathy letter
- **WHEN** `dekhte`, `bujhte`, `dekhbe` and `poRte` are each typed on a fresh engine
- **THEN** the outputs are `দেখতে`, `বুঝতে`, `দেখবে` and `পড়তে`

#### Scenario: After য়
- **WHEN** `jayga` is typed on a fresh engine
- **THEN** the output is `জায়গা`

#### Scenario: ফলা keys still join
- **WHEN** `bhromoN`, `flaiT`, `brahmoN`, `cihno`, `dhwoni` and `madhyom` are each typed on a fresh engine
- **THEN** the outputs are `ভ্রমণ`, `ফ্লাইট`, `ব্রাহ্মণ`, `চিহ্ন`, `ধ্বনি` and `মাধ্যম`

#### Scenario: A plain letter before a breathy one still joins
- **WHEN** `buddhi` and `iccha` are each typed on a fresh engine
- **THEN** the outputs are `বুদ্ধি` and `ইচ্ছা`
