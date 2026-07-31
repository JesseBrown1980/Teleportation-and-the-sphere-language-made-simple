# https://github.com/JesseBrown1980/FOLLOW-THE-IS-NOT-THE-WILL-AND-WAS/blob/main/matrix/3-D-GITHUB-OF-THRUTH.md
"""Build the deterministic JSON-free WORD/FLOWe spread projection."""

from __future__ import annotations

import argparse
import html
import sys
from collections import Counter
from pathlib import Path


TOOLS_DIR = Path(__file__).resolve().parent
if str(TOOLS_DIR) not in sys.path:
    sys.path.insert(0, str(TOOLS_DIR))

import build_nlevel_outward as nlevel


ROOT = TOOLS_DIR.parent
BUILDER_PATH = Path(__file__).resolve()
READFIRST = nlevel.READFIRST
INSTANCE = "WORD_FLOWE_SPREAD_V3"
BASE_INSTANCE = "NLEVEL_OUTWARD_V2"
FLOWE_PATH = ROOT / "language" / "word-flowe-spread-r2.flowe"
LAW_PATH = ROOT / "books" / "LAW-INCREASING-CALLINGS-CALMINGS-WORD-FLOWE-SPREAD.md"
V1_FLOWE_PATH = ROOT / "language" / "core.flowe"
BASE_FLOWE_PATH = nlevel.FLOWE_PATH
HBP_PATH = ROOT / "receipts" / "SPHERE-LANGUAGE-WORD-FLOWE-SPREAD.hbp"
HBI_PATH = ROOT / "receipts" / "SPHERE-LANGUAGE-WORD-FLOWE-SPREAD.hbi"
SVG_PATH = ROOT / "matrix" / "SPHERE-LANGUAGE-WORD-FLOWE-SPREAD.svg"
MANIFEST_PATH = ROOT / "hashes" / "WORD-FLOWE-SPREAD-ARTIFACTS.sha256"

ROUND_COUNT = 2
ROUNDS = tuple(range(1, ROUND_COUNT + 1))
CELL_COUNT = nlevel.CELL_COUNT
EVENT_KINDS = (
    "CALLING_GROWTH_E",
    "CALLING_GROWTH_FLOWE",
    "CALLING_GROWTH_U",
    "CALMING_GROWTH_E",
    "CALMING_GROWTH_OUTWARD",
    "SELF_REDUCTION_GROWTH",
    "WORD_SPREAD",
    "FLOWE_SPREAD",
)
RELATION_ROWS = ROUND_COUNT * CELL_COUNT * len(EVENT_KINDS)
EXPECTED_RECORDS = 8 + ROUND_COUNT + 5 + CELL_COUNT + RELATION_ROWS + 3

ANCHORS = (
    ("n_e", "AETHER_E_CENTER"),
    ("n_flowe_target", "OUTWARD_FLOWE_TARGET"),
    ("n_u", "U_CONNECTION"),
    ("n_o0o", "O0O_SPHERE"),
    ("n_word", "WORD_SOURCE"),
)


def canonical_bytes(path: Path) -> bytes:
    return nlevel.canonical_bytes(path)


def sha256(data: bytes) -> str:
    return nlevel.sha256(data)


def row(record_tag: str, **fields: object) -> str:
    return nlevel.row(record_tag, **fields)


def parse_rows(data: bytes) -> list[tuple[str, dict[str, str], bytes]]:
    return nlevel.parse_rows(data)


def bounded(fields: dict[str, object]) -> dict[str, str]:
    output = {key: str(value) for key, value in fields.items()}
    output["execution_authority"] = "0"
    output["json"] = "0"
    return output


def all_cells() -> list[dict[str, object]]:
    return [
        nlevel.cell_values(level, book)
        for level in range(nlevel.LEVEL_COUNT)
        for book in range(nlevel.BOOK_COUNT)
    ]


def cell_ref_values(cell: dict[str, object]) -> dict[str, object]:
    identity = str(cell["id"])
    return {
        "id": identity,
        "parent_instance": BASE_INSTANCE,
        "parent_id": identity,
        "reference_only": 1,
        "level": cell["level"],
        "book": cell["book"],
        "book_ordinal": cell["book_ordinal"],
        "q": cell["q"],
        "space_radius": cell["space_radius"],
        "x": cell["x"],
        "y": cell["y"],
        "z": cell["z"],
        "time": cell["time"],
        "colour": cell["colour"],
        "oil_family": cell["oil_family"],
        "identity_exchange": 0,
    }


def relation_values(
    kind: str, cell: dict[str, object], growth_round: int
) -> dict[str, object]:
    identity = str(cell["id"])
    amplitude = int(cell["space_radius"]) + growth_round
    common: dict[str, object] = {
        "id": (
            f"{kind.lower()}_r{growth_round:02d}_"
            f"n{int(cell['level']):02d}_b{int(cell['book_ordinal']):02d}"
        ),
        "cell": identity,
        "level": cell["level"],
        "book": cell["book"],
        "q": cell["q"],
        "round": growth_round,
        "increase_q": growth_round,
        "amplitude_q": amplitude,
        "n_open": 1,
        "instant_address": 1,
        "elapsed_measurement_present": 0,
        "identity_exchange": 0,
        "source_retained": 1,
    }
    if kind == "CALLING_GROWTH_E":
        common.update(
            from_=identity,
            to="n_e",
            direction="INCREASING_INTO_E",
            operator_bound=1,
        )
    elif kind == "CALLING_GROWTH_FLOWE":
        common.update(
            from_=identity,
            to="n_flowe_target",
            direction="INCREASING_INTO_FLOWE",
            operator_bound=1,
        )
    elif kind == "CALLING_GROWTH_U":
        common.update(
            from_=identity,
            to="n_u",
            direction="INCREASING_INTO_U",
            operator_bound=1,
        )
    elif kind == "CALMING_GROWTH_E":
        common.update(
            from_=identity,
            to="n_e",
            direction="INCREASING_TOWARD_E",
            oil_family=cell["oil_family"],
            oil_amplitude=amplitude,
        )
    elif kind == "CALMING_GROWTH_OUTWARD":
        common.update(
            from_="n_o0o",
            to=identity,
            direction="INCREASING_OUTWARD",
            oil_family=cell["oil_family"],
            oil_amplitude=amplitude,
        )
    elif kind == "SELF_REDUCTION_GROWTH":
        common.update(
            from_=identity,
            to="n_o0o",
            direction="TOWARD_O0O",
            self_reduction=1,
            deletion=0,
        )
    elif kind == "WORD_SPREAD":
        common.update(
            from_="n_word",
            to=identity,
            direction="OUTWARD",
            spread="WORD",
            spread_step=growth_round,
        )
    elif kind == "FLOWE_SPREAD":
        common.update(
            from_="n_flowe_target",
            to=identity,
            direction="OUTWARD",
            spread="FLOWE",
            spread_step=growth_round,
        )
    else:
        raise ValueError(f"unsupported spread relation: {kind}")
    common["from"] = common.pop("from_")
    return common


def build_flowe(law: bytes, v1: bytes, base: bytes, builder: bytes) -> bytes:
    base_rows = parse_rows(base)
    v1_rows = parse_rows(v1)
    lines = [
        row("READFIRST", url=READFIRST),
        row(
            "LANGUAGE",
            id="SPHERE_LANGUAGE_V1",
            instance=INSTANCE,
            tuple_frame="HYPERBEHCS_60D_PLUS",
            selector_axes=len(nlevel.AXES),
        ),
        row(
            "PARENT",
            id="V1_CORE",
            instance="SPHERE_LANGUAGE_V1",
            path=V1_FLOWE_PATH.relative_to(ROOT).as_posix(),
            sha256=sha256(v1),
            records=len(v1_rows),
            mutation=0,
            identity_exchange=0,
        ),
        row(
            "PARENT",
            id="N16_CORE",
            instance=BASE_INSTANCE,
            path=BASE_FLOWE_PATH.relative_to(ROOT).as_posix(),
            sha256=sha256(base),
            records=len(base_rows),
            mutation=0,
            identity_exchange=0,
        ),
        row(
            "SOURCE",
            path=LAW_PATH.relative_to(ROOT).as_posix(),
            occurrences=1,
            sha256=sha256(law),
            evidence="OPERATOR_CANON",
        ),
        row(
            "CENTER",
            members="HBI,HBP,SHA,SH,HASH",
            traversal_surface="HBI,HBP,SHA,SH,HASH",
            sh="OPERATOR_CANON_UNRESOLVED",
            identity_exchange=0,
        ),
        row(
            "EXPANSION",
            id="word_flowe_spread_v3",
            instance=INSTANCE,
            law=LAW_PATH.relative_to(ROOT).as_posix(),
            law_sha256=sha256(law),
            v1=V1_FLOWE_PATH.relative_to(ROOT).as_posix(),
            v1_sha256=sha256(v1),
            base=BASE_FLOWE_PATH.relative_to(ROOT).as_posix(),
            base_sha256=sha256(base),
            builder=BUILDER_PATH.relative_to(ROOT).as_posix(),
            builder_sha256=sha256(builder),
            n_open=1,
            compiled_rounds=ROUND_COUNT,
            cells=CELL_COUNT,
            event_kinds=len(EVENT_KINDS),
            relation_rows=RELATION_ROWS,
            selector_axes=len(nlevel.AXES),
        ),
        row(
            "PARENT_INVARIANT",
            id="N16_SELF_REDUCTION",
            parent=BASE_INSTANCE,
            tag="SELF_REDUCTION",
            rows=CELL_COUNT,
            identity_exchange=0,
            deletion=0,
            validated=1,
        ),
    ]
    for growth_round in ROUNDS:
        previous = BASE_INSTANCE if growth_round == 1 else f"round_{growth_round - 1:02d}"
        lines.append(
            row(
                "ROUND",
                id=f"round_{growth_round:02d}",
                ordinal=growth_round,
                previous=previous,
                increase_q=growth_round,
                n_open=1,
                compiled_projection=1,
            )
        )
    lines.extend(row("ANCHOR", id=identity, meaning=meaning) for identity, meaning in ANCHORS)
    cells = all_cells()
    lines.extend(row("CELL_REF", **cell_ref_values(cell)) for cell in cells)
    for growth_round in ROUNDS:
        for kind in EVENT_KINDS:
            lines.extend(
                row(kind, **relation_values(kind, cell, growth_round))
                for cell in cells
            )
    lines.extend(
        (
            row(
                "TIMING_BOUNDARY",
                instant_address=1,
                elapsed_measurement_present=0,
                runtime_measurement_present=0,
            ),
            row(
                "BOUNDARY",
                physical_mapping="UNVERIFIED",
                live_runtime_mapping="UNVERIFIED",
                system_affirmed=0,
                two_round_compilation="DESIGN",
            ),
            row(
                "END",
                status="COMPILED_BOUNDED_GROWTH_PROJECTION",
                n_open=1,
                compiled_rounds=ROUND_COUNT,
                cells=CELL_COUNT,
                event_kinds=len(EVENT_KINDS),
                relation_rows=RELATION_ROWS,
            ),
        )
    )
    return ("\n".join(lines) + "\n").encode()


def validate(
    rows: list[tuple[str, dict[str, str], bytes]],
    law: bytes,
    v1: bytes,
    base: bytes,
    builder: bytes,
) -> None:
    if len(rows) != EXPECTED_RECORDS:
        raise ValueError(f"record count mismatch: {len(rows)}")
    if rows[0][0] != "READFIRST" or rows[-1][0] != "END":
        raise ValueError("control row ordering mismatch")
    tags = Counter(tag for tag, _, _ in rows)
    expected_counts = {
        "READFIRST": 1,
        "LANGUAGE": 1,
        "PARENT": 2,
        "SOURCE": 1,
        "CENTER": 1,
        "EXPANSION": 1,
        "PARENT_INVARIANT": 1,
        "ROUND": ROUND_COUNT,
        "ANCHOR": len(ANCHORS),
        "CELL_REF": CELL_COUNT,
        "TIMING_BOUNDARY": 1,
        "BOUNDARY": 1,
        "END": 1,
        **{kind: ROUND_COUNT * CELL_COUNT for kind in EVENT_KINDS},
    }
    if tags != Counter(expected_counts):
        raise ValueError(f"tag population mismatch: {tags}")

    by_tag: dict[str, list[dict[str, str]]] = {}
    for tag, fields, _ in rows:
        by_tag.setdefault(tag, []).append(fields)
        if fields.get("execution_authority") != "0" or fields.get("json") != "0":
            raise ValueError(f"public boundary mismatch: {tag}")

    expected_parents = [
        bounded(
            {
                "id": "V1_CORE",
                "instance": "SPHERE_LANGUAGE_V1",
                "path": V1_FLOWE_PATH.relative_to(ROOT).as_posix(),
                "sha256": sha256(v1),
                "records": len(parse_rows(v1)),
                "mutation": 0,
                "identity_exchange": 0,
            }
        ),
        bounded(
            {
                "id": "N16_CORE",
                "instance": BASE_INSTANCE,
                "path": BASE_FLOWE_PATH.relative_to(ROOT).as_posix(),
                "sha256": sha256(base),
                "records": len(parse_rows(base)),
                "mutation": 0,
                "identity_exchange": 0,
            }
        ),
    ]
    if by_tag["PARENT"] != expected_parents:
        raise ValueError("parent commitment mismatch")
    if by_tag["PARENT_INVARIANT"][0] != bounded(
        {
            "id": "N16_SELF_REDUCTION",
            "parent": BASE_INSTANCE,
            "tag": "SELF_REDUCTION",
            "rows": CELL_COUNT,
            "identity_exchange": 0,
            "deletion": 0,
            "validated": 1,
        }
    ):
        raise ValueError("parent self-reduction invariant mismatch")
    if by_tag["SOURCE"][0] != bounded(
        {
            "path": LAW_PATH.relative_to(ROOT).as_posix(),
            "occurrences": 1,
            "sha256": sha256(law),
            "evidence": "OPERATOR_CANON",
        }
    ):
        raise ValueError("operator law commitment mismatch")

    expected_rounds = [
        bounded(
            {
                "id": f"round_{growth_round:02d}",
                "ordinal": growth_round,
                "previous": (
                    BASE_INSTANCE
                    if growth_round == 1
                    else f"round_{growth_round - 1:02d}"
                ),
                "increase_q": growth_round,
                "n_open": 1,
                "compiled_projection": 1,
            }
        )
        for growth_round in ROUNDS
    ]
    if by_tag["ROUND"] != expected_rounds:
        raise ValueError("growth round chain mismatch")

    expected_anchors = [
        bounded({"id": identity, "meaning": meaning}) for identity, meaning in ANCHORS
    ]
    if by_tag["ANCHOR"] != expected_anchors:
        raise ValueError("anchor set mismatch")

    cells = all_cells()
    expected_refs = [bounded(cell_ref_values(cell)) for cell in cells]
    if by_tag["CELL_REF"] != expected_refs:
        raise ValueError("cell reference mismatch")
    identities = {fields["id"] for fields in by_tag["CELL_REF"]}
    if len(identities) != CELL_COUNT:
        raise ValueError("cell identity collision")

    for kind in EVENT_KINDS:
        expected = [
            bounded(relation_values(kind, cell, growth_round))
            for growth_round in ROUNDS
            for cell in cells
        ]
        if by_tag[kind] != expected:
            raise ValueError(f"{kind} relation mismatch")
        for fields in by_tag[kind]:
            if fields["cell"] not in identities:
                raise ValueError(f"{kind} unresolved cell")
            if fields["identity_exchange"] != "0":
                raise ValueError(f"{kind} identity exchange")

    if any(
        fields.get("deletion") != "0"
        for fields in by_tag["SELF_REDUCTION_GROWTH"]
    ):
        raise ValueError("self reduction deletion mismatch")
    if {fields["from"] for fields in by_tag["WORD_SPREAD"]} != {"n_word"}:
        raise ValueError("WORD spread source mismatch")
    if {fields["from"] for fields in by_tag["FLOWE_SPREAD"]} != {
        "n_flowe_target"
    }:
        raise ValueError("FLOWe spread source mismatch")

    parent_self_reductions = [
        fields
        for tag, fields, _ in parse_rows(base)
        if tag == "SELF_REDUCTION"
    ]
    if len(parent_self_reductions) != CELL_COUNT or any(
        fields.get("identity_exchange") != "0" or fields.get("deletion") != "0"
        for fields in parent_self_reductions
    ):
        raise ValueError("parent self-reduction bytes violate invariant")

    actual = b"\n".join(raw for _, _, raw in rows) + b"\n"
    if actual != build_flowe(law, v1, base, builder):
        raise ValueError("canonical spread bytes mismatch")


def build_hbp(
    flowe: bytes, law: bytes, v1: bytes, base: bytes, builder: bytes
) -> bytes:
    header = (
        row(
            "HBPHEADER",
            schema="SPHERE-WORD-FLOWE-SPREAD-COMPILED-V3",
            law_sha256=sha256(law),
            v1_sha256=sha256(v1),
            base_sha256=sha256(base),
            builder_sha256=sha256(builder),
            core_sha256=sha256(flowe),
            records=len(flowe.splitlines()),
            rounds=ROUND_COUNT,
            cells=CELL_COUNT,
            event_kinds=len(EVENT_KINDS),
            relations=RELATION_ROWS,
        )
        + "\n"
    )
    body = (row("READFIRST", url=READFIRST) + "\n" + header).encode() + flowe
    footer = (
        row(
            "HBPFOOTER",
            body_sha256=sha256(body),
            rows=len(body.splitlines()) + 1,
        )
        + "\n"
    ).encode()
    return body + footer


def build_hbi(hbp: bytes) -> bytes:
    lines = hbp.splitlines(keepends=True)
    digest = sha256(hbp)
    output = bytearray((row("READFIRST", url=READFIRST) + "\n").encode())
    output += (
        row(
            "HBIHEADER",
            schema="SPHERE-WORD-FLOWE-SPREAD-OFFSET-V3",
            hbp_sha256=digest,
            rows=len(lines),
        )
        + "\n"
    ).encode()
    offset = 0
    for number, line in enumerate(lines):
        raw = line[:-1]
        tag = raw.split(b"|", 1)[0].decode()
        output += (
            row(
                "INDEX",
                row=number,
                offset=offset,
                bytes=len(raw),
                tag=tag,
                row_sha256=sha256(raw),
            )
            + "\n"
        ).encode()
        offset += len(line)
    output += (
        row("HBIFOOTER", hbp_bytes=len(hbp), hbp_sha256=digest) + "\n"
    ).encode()
    return bytes(output)


def build_svg(rows: list[tuple[str, dict[str, str], bytes]], law_sha: str) -> bytes:
    cells = all_cells()
    positions = {
        str(cell["id"]): nlevel.svg_position(
            {key: str(value) for key, value in cell.items()}
        )
        for cell in cells
    }
    positions.update(
        {
            "n_e": (120, 1024),
            "n_flowe_target": (1928, 1024),
            "n_u": (1024, 1928),
            "n_o0o": (1024, 120),
            "n_word": (1024, 1024),
        }
    )
    colours = {
        "CALLING_GROWTH_E": "#ff7b00",
        "CALLING_GROWTH_FLOWE": "#00b4d8",
        "CALLING_GROWTH_U": "#f72585",
        "CALMING_GROWTH_E": "#43aa8b",
        "CALMING_GROWTH_OUTWARD": "#80ed99",
        "SELF_REDUCTION_GROWTH": "#facc15",
        "WORD_SPREAD": "#ffffff",
        "FLOWE_SPREAD": "#00f5d4",
    }
    fills = {
        "WHITE": "#f8fafc",
        "BLACK": "#111827",
        "BROWN": "#a16207",
        "RAINBOW": "#c026d3",
    }
    by_tag: dict[str, list[dict[str, str]]] = {}
    for tag, fields, _ in rows:
        by_tag.setdefault(tag, []).append(fields)
    output = [
        f"<!-- {READFIRST} -->",
        '<svg xmlns="http://www.w3.org/2000/svg" width="2048" height="2048" viewBox="0 0 2048 2048" role="img" aria-labelledby="title desc" data-json="0" data-execution-authority="0">',
        '<title id="title">Increasing CALLINGS CALMINGS WORD and FLOWe spread</title>',
        (
            '<desc id="desc">Two bounded integer growth rounds over 160 sealed N16 '
            f'cell references; open N; law SHA-256 {law_sha}</desc>'
        ),
        '<rect width="2048" height="2048" fill="#050816"/>',
        '<circle cx="1024" cy="1024" r="900" fill="none" stroke="#334155" stroke-width="4"/>',
        '<g id="spread-relations" fill="none">',
    ]
    for kind in EVENT_KINDS:
        for fields in by_tag[kind]:
            x1, y1 = positions[fields["from"]]
            x2, y2 = positions[fields["to"]]
            output.append(
                f'<line id="{html.escape(fields["id"])}" data-kind="{kind}" '
                f'data-round="{fields["round"]}" x1="{x1}" y1="{y1}" '
                f'x2="{x2}" y2="{y2}" stroke="{colours[kind]}" '
                f'stroke-width="{fields["round"]}"/>'
            )
    output.append("</g>")
    output.append('<g id="cells" stroke="#94a3b8" stroke-width="1">')
    for cell in cells:
        x, y = positions[str(cell["id"])]
        output.append(
            f'<circle id="{cell["id"]}" data-kind="CELL_REF" cx="{x}" cy="{y}" '
            f'r="5" fill="{fills[str(cell["colour"])]}"/>'
        )
    output.extend(
        (
            "</g>",
            '<g id="anchors" fill="#ffffff" font-family="monospace" font-size="24">',
            '<circle cx="1024" cy="1024" r="16" fill="#ffffff"/><text x="1046" y="1032">WORD</text>',
            '<text x="44" y="1016">E</text>',
            '<text x="1850" y="1016">FLOWe</text>',
            '<text x="1008" y="1980">U</text>',
            '<text x="990" y="82">o0O</text>',
            "</g>",
            '<text x="40" y="2020" fill="#cbd5e1" font-family="monospace" font-size="22">N=OPEN | compiled increase rounds=2 | relations=2560 | HBI HBP SHA SH HASH | json=0</text>',
            "</svg>",
        )
    )
    return ("\n".join(output) + "\n").encode()


def artifacts() -> dict[Path, bytes]:
    builder = canonical_bytes(BUILDER_PATH)
    law = canonical_bytes(LAW_PATH)
    v1 = canonical_bytes(V1_FLOWE_PATH)
    base = canonical_bytes(BASE_FLOWE_PATH)
    flowe = build_flowe(law, v1, base, builder)
    rows = parse_rows(flowe)
    validate(rows, law, v1, base, builder)
    hbp = build_hbp(flowe, law, v1, base, builder)
    hbi = build_hbi(hbp)
    svg = build_svg(rows, sha256(law))
    sealed = {
        BUILDER_PATH: builder,
        LAW_PATH: law,
        FLOWE_PATH: flowe,
        HBP_PATH: hbp,
        HBI_PATH: hbi,
        SVG_PATH: svg,
    }
    output = {
        FLOWE_PATH: flowe,
        HBP_PATH: hbp,
        HBI_PATH: hbi,
        SVG_PATH: svg,
    }
    for path, data in sealed.items():
        output[path.with_name(path.name + ".sha256")] = (
            f"{sha256(data)}  {path.name}\n"
        ).encode()
    manifest_items = {V1_FLOWE_PATH: v1, BASE_FLOWE_PATH: base, **sealed}
    output[MANIFEST_PATH] = (
        "\n".join(
            f"{sha256(data)}  {path.relative_to(ROOT).as_posix()}"
            for path, data in sorted(
                manifest_items.items(), key=lambda item: item[0].as_posix()
            )
        )
        + "\n"
    ).encode()
    return output


def apply(check: bool) -> None:
    expected = artifacts()
    mismatches: list[str] = []
    for path, data in expected.items():
        if check:
            if not path.is_file() or path.read_bytes() != data:
                mismatches.append(path.relative_to(ROOT).as_posix())
        else:
            path.parent.mkdir(parents=True, exist_ok=True)
            if not path.is_file() or path.read_bytes() != data:
                path.write_bytes(data)
    if mismatches:
        raise SystemExit(
            "SPREAD_BUILD|PASS=0|mismatch=" + ",".join(mismatches)
        )
    flowe = expected[FLOWE_PATH]
    hbp = expected[HBP_PATH]
    hbi = expected[HBI_PATH]
    print(
        f"SPREAD_BUILD|PASS=1|mode={'check' if check else 'write'}"
        f"|rounds={ROUND_COUNT}|cells={CELL_COUNT}|event_kinds={len(EVENT_KINDS)}"
        f"|relations={RELATION_ROWS}|core_rows={len(flowe.splitlines())}"
        f"|hbp_rows={len(hbp.splitlines())}|hbi_rows={len(hbi.splitlines())}"
        f"|core_sha256={sha256(flowe)}|json=0"
    )


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    apply(args.check)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
