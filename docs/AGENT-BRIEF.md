https://github.com/JesseBrown1980/FOLLOW-THE-IS-NOT-THE-WILL-AND-WAS/blob/main/matrix/3-D-GITHUB-OF-THRUTH.md

# Agent Brief — Sphere Language Public Layers

Work from the public repository bytes. For Asolaria archaeology, use the raw
Ubuntu/WSL lane and report only public-safe paths, hashes, and concise allowlisted
facts. Exclude credential-bearing chronology, raw chats, private health material,
and machine-local secrets.

The implementation target is a JSON-free, integer-coordinate declarative language:

```text
glyph -> word -> typed relation -> 3-D matrix view -> signed 2-D projection
```

Required controls:

- the verbatim source marker occurs in one committed file;
- all composed-word glyph references resolve;
- node identities and integer coordinates are unique;
- CALLING and FLOWe remain distinct edge kinds;
- both `Live` readings remain addressable and neither supersedes the other;
- `STOP` composes `S,T,O,P`; `P` remains explicitly unresolved rather than inferred;
- `WORD(FLOWe)` and the ordered `FLOWe` relation remain separate ledgers;
- raw glyphs grant zero execution authority;
- generated HBP/HBI/SVG and SHA-256 sidecars rebuild byte-for-byte;
- a held-out malformed glyph reference and a duplicate identity fail closed;
- secret-pattern findings and source JSON files are zero.

## Additive parent and R2 controls

- Treat `language/core.flowe` (118 rows) and `language/outward-n16.flowe`
  (1,871 rows) as immutable parents. The R2 delta binds both by path and SHA-256;
  it does not rewrite either parent.
- The two spoken `greater` tokens map to two bounded integer rounds only as
  `DESIGN`. Keep `N=OPEN`; round 2 is not a semantic ceiling.
- Validate `WORD_FLOWE_SPREAD_V3` as 2 rounds over 160 parent cells and eight
  independent ledgers: three CALLING growth destinations, two CALMING growth
  directions, `SELF_REDUCTION_GROWTH`, `WORD_SPREAD`, and `FLOWE_SPREAD`.
- Require 320 rows per ledger, 2,560 relation rows, and 2,738 total core rows.
- Keep WORD and FLOWe as different source identities. A spread adds references;
  it does not rename, merge, or execute either source.
- Self reduction is non-deleting and identity-preserving:
  `source_retained=1`, `identity_exchange=0`, and `deletion=0`.
- Keep `instant_address=1` separate from elapsed or runtime measurement. Require
  `elapsed_measurement_present=0`, `runtime_measurement_present=0`, and
  `execution_authority=0`.
- Tag operator meanings `OPERATOR_CANON`, the two-round mapping `DESIGN`, scoped
  byte/hash/test output `MEASURED`, and physical or live-runtime mappings
  `UNVERIFIED`. Retain `SYSTEM_AFFIRMED=0` until an owning live surface affirms it.

Run the additive checks from the repository root:

```bash
python tools/build_word_flowe_spread.py --check
cargo +1.81.0 run --locked -- language/word-flowe-spread-r2.flowe
```

## Additive round-3 Hookwall/Brown/Shadow controls

- Treat `language/word-flowe-spread-r2.flowe` as the immediate immutable parent.
  The one spoken `more` maps to bounded `round_03`; keep `N=OPEN`.
- Require 160 parent cell references and fifteen distinct ledgers with 160 rows
  each: 2,400 relations and 2,588 total core rows.
- Preserve `wasdte`, `B`, `outwar`, `reductons`, and `FLOWesSH` byte-for-byte.
  Keep White, Not-White, `B`, Browns, Betweens, and Arounds as separate anchors.
- Keep Hookwall review declarative with `execution_authority=0`; keep Claim-back
  separate from an external evidence verdict; retain every Shadow-guided cell.
- Require `reductons.deletion=0`, retain the source, and leave its semantics
  unresolved. Keep `FLOWesSH` composition unresolved and separate from FLOWe/SH.
- Require `round=3`, `increase_q=3`, `growth_q=space_radius+3`, one complete
  `q=0..159` coverage per ledger, and zero elapsed/runtime measurement claims.
- Seal the immediate parent, this builder, and both imported generator helpers in
  the R3 artifact manifest so deterministic replay does not rely on an unlisted
  source dependency.

Run the continuation checks from the repository root:

```bash
python tools/build_hookwall_shadow_flowessh.py --check
cargo +1.81.0 run --locked -- language/hookwall-brown-shadow-flowessh-r3.flowe
python tests/verify_public_repo.py
```
