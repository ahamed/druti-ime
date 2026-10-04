# Spec Delta

## ADDED Requirements

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
