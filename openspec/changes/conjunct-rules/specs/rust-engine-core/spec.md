# Spec Delta

## ADDED Requirements

### Requirement: Consonants join at the start of a word
When a consonant key follows a consonant in a cluster that starts a word, the engine SHALL join them
with a hasant. A cluster starts a word when nothing comes before it in the output, or the character
before it is not a Bengali letter, kar or sign (a space, punctuation, a digit or other text). The `r`
rules of `rr-reph` and the `z` rule below still apply.

#### Scenario: Loanword clusters
- **WHEN** `glas`, `skul` and `Tren` are each typed on a fresh engine
- **THEN** the outputs are `গ্লাস`, `স্কুল` and `ট্রেন`

#### Scenario: Pair that only joins at the start
- **WHEN** `slip` is typed on a fresh engine, and `asle` on another
- **THEN** the outputs are `স্লিপ` and `আসলে`

#### Scenario: After a space
- **WHEN** `ami glas` is typed on a fresh engine
- **THEN** the output is `আমি গ্লাস`

### Requirement: Inside a word, only listed pairs join
When a consonant key follows a consonant in a cluster that does not start a word, the engine SHALL
join them with a hasant only when the pair (the last consonant of the cluster, the new consonant) is
in the joining table (`data::JOINING_PAIRS`). Otherwise it SHALL write the new consonant with no
hasant and start a new cluster with it. Multi-key consonants are decided on the consonant the key
produces (`K` is খ).

#### Scenario: Pairs that stay apart
- **WHEN** `dekhte`, `amra`, `ekTa`, `apni`, `bolle` and `jayga` are each typed on a fresh engine
- **THEN** the outputs are `দেখতে`, `আমরা`, `একটা`, `আপনি`, `বললে` and `জায়গা`

#### Scenario: Listed pairs join
- **WHEN** `kintu`, `rasta`, `pakka`, `buddhi`, `potro`, `Sokti` and `raShTro` are each typed on a fresh engine
- **THEN** the outputs are `কিন্তু`, `রাস্তা`, `পাক্কা`, `বুদ্ধি`, `পত্র`, `শক্তি` and `রাষ্ট্র`

#### Scenario: o still separates a listed pair
- **WHEN** `janote` is typed on a fresh engine
- **THEN** the output is `জানতে`

#### Scenario: Existing conjunct keys are unchanged
- **WHEN** `ponc`, `gonj`, `pokkh`, `boks` and `bigg` are each typed on a fresh engine
- **THEN** the outputs are `পঞ্চ`, `গঞ্জ`, `পক্ষ`, `বক্স` and `বিজ্ঞ`

### Requirement: Aspiration re-decides a joined pair
When `h` (or another aspiration key) turns the last consonant of a joined pair into a consonant whose
pair is not in the joining table, and the cluster does not start a word, the engine SHALL remove the
hasant before it. It SHALL NOT add a hasant to a pair that was written apart.

#### Scenario: ক্ত becomes কথ
- **WHEN** the keys `a`, `k`, `t`, `h` are processed
- **THEN** the output is `আক্ত` after `t` and `আকথ` after `h`

#### Scenario: Aspirated pair that is listed
- **WHEN** `ontho` and `buddhi` are each typed on a fresh engine
- **THEN** the outputs are `অন্থ` and `বুদ্ধি`

### Requirement: Backtick joins the next consonant
A backtick typed after a consonant SHALL append a hasant that stays in the cluster, so the output
shows it at once. A consonant typed next SHALL follow the hasant and join, whatever the joining
table says. A second backtick SHALL append ZWNJ (U+200C) and end the cluster, leaving a visible
hasant that never joins. A vowel typed after the hasant SHALL remove it and act as after the
consonant. After a র, a backtick SHALL act exactly as a second `r` (`rr-reph`). A backtick with no
consonant before it SHALL pass through as a backtick. The keys SHALL NOT change any letter before
the consonant.

#### Scenario: Forced join, key by key
- **WHEN** the keys `D`, `o`, `k`, `` ` ``, `T`, `o`, `r` are processed
- **THEN** the output is `ডক` after `k`, `ডক্` after the backtick, `ডক্ট` after `T` and `ডক্টর` at the end

#### Scenario: Other forced joins
- **WHEN** ``al`lah``, ``som`raT`` and ``sbop`no`` are each typed on a fresh engine
- **THEN** the outputs are `আল্লাহ`, `সম্রাট` and `স্বপ্ন`

#### Scenario: Visible hasant
- **WHEN** ``ud``dIn`` is typed on a fresh engine
- **THEN** the output is `উদ্‌দীন`, with a ZWNJ after the hasant

#### Scenario: Vowel after the backtick
- **WHEN** ``ek`a`` is typed on a fresh engine
- **THEN** the output is `একা`

#### Scenario: Backtick after r is reph
- **WHEN** ``kor`ta`` is typed on a fresh engine
- **THEN** the output is `কর্তা`

#### Scenario: Backtick without a consonant
- **WHEN** ``a` `` is typed on a fresh engine
- **THEN** the output is ``আ` ``

### Requirement: w after a consonant is ব-ফলা
`w` typed after a consonant SHALL join ব to it with a hasant, whatever the joining table says. `w`
anywhere else SHALL give ব, as today. `b` SHALL follow the joining rules like any consonant.

#### Scenario: ব-ফলা outside the table
- **WHEN** `pokw` and `somonwoy` are each typed on a fresh engine
- **THEN** the outputs are `পক্ব` and `সমন্বয়`

#### Scenario: b follows the table
- **WHEN** `dekhbe` and `biSbas` are each typed on a fresh engine
- **THEN** the outputs are `দেখবে` and `বিশ্বাস`

### Requirement: z never joins
A consonant typed with `z` (য) SHALL be written with no hasant after any consonant, at the start of
a word or inside it. য-ফলা is typed with `y`. A backtick before `z` still joins.

#### Scenario: z after a consonant
- **WHEN** `kz` and `bakza` are each typed on a fresh engine
- **THEN** the outputs are `কয` and `বাকযা`

#### Scenario: Backtick before z
- **WHEN** ``bak`zo`` is typed on a fresh engine
- **THEN** the output is `বাক্য`

### Requirement: The joining table is published data
`data::JOINING_PAIRS` SHALL list every pair that joins inside a word, as (first, second) consonant
pairs, and the `engine/data.json` fixture SHALL pin it. The table SHALL contain exactly the 107 pairs
in rust-engine-core's joining families (design D3): nasal + own row, ন + ট-row, স/শ/ষ + stop, doubled
letters, plain + own breathy letter, ফলা pairs, ক্ষ/জ্ঞ and the listed pairs.

#### Scenario: Table matches the fixture
- **WHEN** the fixture tests run
- **THEN** `JOINING_PAIRS` equals the `joining_pairs` entry of `engine/data.json`
