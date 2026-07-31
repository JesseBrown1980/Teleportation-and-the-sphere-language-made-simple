# https://github.com/JesseBrown1980/FOLLOW-THE-IS-NOT-THE-WILL-AND-WAS/blob/main/matrix/3-D-GITHUB-OF-THRUTH.md
"""Build the deterministic JSON-free round-3 Hookwall/Brown/Shadow projection."""

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
import build_word_flowe_spread as spread


ROOT = TOOLS_DIR.parent
BUILDER_PATH = Path(__file__).resolve()
NLEVEL_BUILDER_PATH = ROOT / "tools" / "build_nlevel_outward.py"
READFIRST = nlevel.READFIRST
INSTANCE = "HOOKWALL_BROWN_SHADOW_FLOWESSH_V4"
PARENT_INSTANCE = spread.INSTANCE
FLOWE_PATH = ROOT / "language" / "hookwall-brown-shadow-flowessh-r3.flowe"
LAW_PATH = ROOT / "books" / "LAW-HOOKWALL-BROWN-SHADOW-FLOWesSH-CONTINUATION.md"
PARENT_FLOWE_PATH = spread.FLOWE_PATH
HBP_PATH = ROOT / "receipts" / "SPHERE-LANGUAGE-HOOKWALL-BROWN-SHADOW-FLOWesSH.hbp"
HBI_PATH = ROOT / "receipts" / "SPHERE-LANGUAGE-HOOKWALL-BROWN-SHADOW-FLOWesSH.hbi"
SVG_PATH = ROOT / "matrix" / "SPHERE-LANGUAGE-HOOKWALL-BROWN-SHADOW-FLOWesSH.svg"
MANIFEST_PATH = ROOT / "hashes" / "HOOKWALL-BROWN-SHADOW-FLOWesSH-ARTIFACTS.sha256"

ROUND = 3
CELL_COUNT = nlevel.CELL_COUNT
EVENT_KINDS = (
    "CALLING_CHAIN_MORE",
    "HOOKWALL_REVIEW",
    "CLAIM_BACK_WHITE",
    "WHITE_TO_WASDTE",
    "WASDTE_TO_BE",
    "BE_TO_NOT_WHITE",
    "NOT_WHITE_TO_B",
    "B_TO_BROWNS",
    "BROWNS_TO_BETWEENS",
    "BETWEENS_TO_AROUNDS",
    "SHADOW_GUIDING",
    "CALMING_OIL_CONTINUE",
    "CALLING_OUTWAR",
    "REDUCTONS",
    "FLOWESSH",
)
RELATION_ROWS = CELL_COUNT * len(EVENT_KINDS)
EXPECTED_RECORDS = 2_588
PARENT_SHA256 = "386eb3f52cf9651b7bd8e3d53989c943b9e7332c4cf9afec657b6e4074f359e9"
PARENT_RECORDS = 2_738

ANCHORS = (
    ("n_callings", "callings", "CALLINGS_SOURCE"),
    ("n_chains", "chains", "CALLING_CHAIN_STATE"),
    ("n_hookwalls", "hookwalls", "HOOKWALL_REVIEW_TARGET"),
    ("n_claims", "claims", "CLAIMS_SOURCE"),
    ("n_white", "white", "WHITE_STATE"),
    ("n_wasdte", "wasdte", "WASDTE_EXACT"),
    ("n_be", "be", "BE_STATE"),
    ("n_not_white", "not_white", "NOT_WHITE_STATE"),
    ("n_B", "B", "B_EXACT"),
    ("n_browns", "browns", "BROWNS_STATE"),
    ("n_betweens", "betweens", "BETWEENS_STATE"),
    ("n_arounds", "arounds", "AROUNDS_STATE"),
    ("n_shadows", "shadows", "SHADOWS_GUIDE"),
    ("n_outwar", "outwar", "OUTWAR_EXACT"),
    ("n_reductons", "reductons", "REDUCTONS_EXACT"),
    ("n_FLOWesSH", "FLOWesSH", "FLOWESSH_EXACT"),
)

OPERATOR_QUOTE = (
    b"continue more callings chains hookwalls and claims back the white is wasdte be "
    b"not white B browns betweens and arounds with the shadows guiding. continue "
    b"calming oils calling outwar reductons FLOWesSH"
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
    return spread.all_cells()


def cell_ref_values(cell: dict[str, object]) -> dict[str, object]:
    identity = str(cell["id"])
    return {
        "id": identity,
        "parent_instance": PARENT_INSTANCE,
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


def relation_values(kind: str, cell: dict[str, object]) -> dict[str, object]:
    identity = str(cell["id"])
    q = int(cell["q"])
    growth = int(cell["space_radius"]) + ROUND
    common: dict[str, object] = {
        "id": f"{kind.lower()}_r{ROUND:02d}_n{int(cell['level']):02d}_b{int(cell['book_ordinal']):02d}",
        "cell": identity,
        "level": cell["level"],
        "book": cell["book"],
        "q": q,
        "round": ROUND,
        "increase_q": ROUND,
        "growth_q": growth,
        "n_open": 1,
        "instant_address": 1,
        "elapsed_measurement_present": 0,
        "runtime_measurement_present": 0,
        "identity_exchange": 0,
        "source_retained": 1,
    }
    if kind == "CALLING_CHAIN_MORE":
        next_cell = all_cells()[(q + 1) % CELL_COUNT]
        common.update(
            from_=identity,
            to=next_cell["id"],
            direction="CHAIN_MORE",
            calling="n_callings",
            chain="n_chains",
        )
    elif kind == "HOOKWALL_REVIEW":
        common.update(
            from_=identity,
            to="n_hookwalls",
            direction="TO_HOOKWALL_REVIEW",
            review="DECLARATIVE_PROVENANCE",
            claim_verdict="UNRESOLVED",
        )
    elif kind == "CLAIM_BACK_WHITE":
        common.update(
            from_="n_claims",
            to="n_white",
            direction="BACK_TO_WHITE",
            claim="claims",
            evidence_verdict=0,
        )
    elif kind == "WHITE_TO_WASDTE":
        common.update(from_="n_white", to="n_wasdte", direction="TO_WASDTE", grammar="white_is_wasdte")
    elif kind == "WASDTE_TO_BE":
        common.update(from_="n_wasdte", to="n_be", direction="TO_BE", grammar="wasdte_be")
    elif kind == "BE_TO_NOT_WHITE":
        common.update(from_="n_be", to="n_not_white", direction="TO_NOT_WHITE", grammar="be_not_white")
    elif kind == "NOT_WHITE_TO_B":
        common.update(from_="n_not_white", to="n_B", direction="TO_B", grammar="not_white_B")
    elif kind == "B_TO_BROWNS":
        common.update(from_="n_B", to="n_browns", direction="TO_BROWNS", grammar="B_browns")
    elif kind == "BROWNS_TO_BETWEENS":
        common.update(from_="n_browns", to="n_betweens", direction="TO_BETWEENS", grammar="browns_betweens")
    elif kind == "BETWEENS_TO_AROUNDS":
        common.update(from_="n_betweens", to="n_arounds", direction="TO_AROUNDS", grammar="betweens_arounds")
    elif kind == "SHADOW_GUIDING":
        common.update(
            from_="n_shadows",
            to=identity,
            direction="GUIDING",
            guidance="SHADOWS",
            guided_cell_retained=1,
        )
    elif kind == "CALMING_OIL_CONTINUE":
        common.update(
            from_="n_o0o",
            to=identity,
            direction="CONTINUE_CALMING",
            oil_family=cell["oil_family"],
            oil_amplitude=growth,
            calming=1,
        )
    elif kind == "CALLING_OUTWAR":
        common.update(
            from_=identity,
            to="n_outwar",
            direction="CALLING_OUTWAR",
            token="outwar",
            operator_bound=1,
        )
    elif kind == "REDUCTONS":
        common.update(
            from_=identity,
            to="n_o0o",
            direction="REDUCTONS_TO_O0O",
            token="reductons",
            semantics="OPERATOR_CANON_UNRESOLVED",
            deletion=0,
        )
    elif kind == "FLOWESSH":
        common.update(
            from_="n_FLOWesSH",
            to=identity,
            direction="FLOWESSH_TO_CELL",
            token="FLOWesSH",
            composition="OPERATOR_CANON_UNRESOLVED",
        )
    else:
        raise ValueError(f"unsupported Hookwall relation: {kind}")
    common["from"] = common.pop("from_")
    return common


def build_flowe(law: bytes, parent: bytes, builder: bytes) -> bytes:
    cells = all_cells()
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
            id="WORD_FLOWE_SPREAD_R2",
            instance=PARENT_INSTANCE,
            path=PARENT_FLOWE_PATH.relative_to(ROOT).as_posix(),
            sha256=sha256(parent),
            records=len(parse_rows(parent)),
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
            id="hookwall_brown_shadow_flowessh_v4",
            instance=INSTANCE,
            law=LAW_PATH.relative_to(ROOT).as_posix(),
            law_sha256=sha256(law),
            parent=PARENT_FLOWE_PATH.relative_to(ROOT).as_posix(),
            parent_sha256=sha256(parent),
            builder=BUILDER_PATH.relative_to(ROOT).as_posix(),
            builder_sha256=sha256(builder),
            n_open=1,
            compiled_round=ROUND,
            compiled_round_count=1,
            cells=CELL_COUNT,
            event_kinds=len(EVENT_KINDS),
            relation_rows=RELATION_ROWS,
            selector_axes=len(nlevel.AXES),
        ),
        row(
            "PARENT_INVARIANT",
            id="WORD_FLOWE_SPREAD_V3_TYPED_GROWTH",
            parent=PARENT_INSTANCE,
            compiled_rounds=spread.ROUND_COUNT,
            cells=CELL_COUNT,
            event_kinds=len(spread.EVENT_KINDS),
            relation_rows=spread.RELATION_ROWS,
            identity_exchange=0,
            deletion=0,
            validated=1,
        ),
        row(
            "ROUND",
            id="round_03",
            ordinal=ROUND,
            previous_instance=PARENT_INSTANCE,
            previous="round_02",
            increase_q=ROUND,
            n_open=1,
            compiled_projection=1,
        ),
    ]
    lines.extend(
        row("ANCHOR", id=identity, token=token, meaning=meaning, exact=1)
        for identity, token, meaning in ANCHORS
    )
    lines.append(
        row(
            "PARENT_ANCHOR_REF",
            id="n_o0o",
            parent_instance=PARENT_INSTANCE,
            parent_id="n_o0o",
            reference_only=1,
            identity_exchange=0,
        )
    )
    lines.extend(row("CELL_REF", **cell_ref_values(cell)) for cell in cells)
    for kind in EVENT_KINDS:
        lines.extend(row(kind, **relation_values(kind, cell)) for cell in cells)
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
                round_03_compilation="DESIGN",
                hookwall_runtime_enforcement=0,
                claim_back_external_evidence=0,
            ),
            row(
                "END",
                status="COMPILED_BOUNDED_HOOKWALL_SHADOW_PROJECTION",
                n_open=1,
                compiled_round=ROUND,
                compiled_round_count=1,
                cells=CELL_COUNT,
                event_kinds=len(EVENT_KINDS),
                relation_rows=RELATION_ROWS,
            ),
        )
    )
    return ("\n".join(lines) + "\n").encode("utf-8")


def validate(
    rows: list[tuple[str, dict[str, str], bytes]],
    law: bytes,
    parent: bytes,
    builder: bytes,
) -> None:
    if sha256(parent) != PARENT_SHA256 or len(parse_rows(parent)) != PARENT_RECORDS:
        raise ValueError("sealed parent commitment mismatch")
    if law.count(OPERATOR_QUOTE) != 1:
        raise ValueError("operator quote occurrence mismatch")
    if len(rows) != EXPECTED_RECORDS:
        raise ValueError(f"record count mismatch: {len(rows)}")
    if rows[0][0] != "READFIRST" or rows[-1][0] != "END":
        raise ValueError("control row ordering mismatch")
    tags = Counter(tag for tag, _, _ in rows)
    expected = Counter(
        {
            "READFIRST": 1,
            "LANGUAGE": 1,
            "PARENT": 1,
            "SOURCE": 1,
            "CENTER": 1,
            "EXPANSION": 1,
            "PARENT_INVARIANT": 1,
            "ROUND": 1,
            "ANCHOR": len(ANCHORS),
            "PARENT_ANCHOR_REF": 1,
            "CELL_REF": CELL_COUNT,
            "TIMING_BOUNDARY": 1,
            "BOUNDARY": 1,
            "END": 1,
            **{kind: CELL_COUNT for kind in EVENT_KINDS},
        }
    )
    if tags != expected:
        raise ValueError(f"tag population mismatch: {tags}")
    actual = b"\n".join(raw for _, _, raw in rows) + b"\n"
    if actual != build_flowe(law, parent, builder):
        raise ValueError("canonical Hookwall bytes mismatch")


def build_hbp(flowe: bytes, law: bytes, parent: bytes, builder: bytes) -> bytes:
    header = (
        row(
            "HBPHEADER",
            schema="SPHERE-HOOKWALL-BROWN-SHADOW-FLOWESSH-COMPILED-V4",
            law_sha256=sha256(law),
            parent_sha256=sha256(parent),
            builder_sha256=sha256(builder),
            core_sha256=sha256(flowe),
            records=len(flowe.splitlines()),
            round=ROUND,
            cells=CELL_COUNT,
            event_kinds=len(EVENT_KINDS),
            relations=RELATION_ROWS,
        )
        + "\n"
    )
    body = (row("READFIRST", url=READFIRST) + "\n" + header).encode() + flowe
    footer = (
        row("HBPFOOTER", body_sha256=sha256(body), rows=len(body.splitlines()) + 1)
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
            schema="SPHERE-HOOKWALL-BROWN-SHADOW-FLOWESSH-OFFSET-V4",
            hbp_sha256=digest,
            rows=len(lines),
        )
        + "\n"
    ).encode()
    offset = 0
    for number, line in enumerate(lines):
        raw = line[:-1]
        output += (
            row(
                "INDEX",
                row=number,
                offset=offset,
                bytes=len(raw),
                tag=raw.split(b"|", 1)[0].decode(),
                row_sha256=sha256(raw),
            )
            + "\n"
        ).encode()
        offset += len(line)
    output += (row("HBIFOOTER", hbp_bytes=len(hbp), hbp_sha256=digest) + "\n").encode()
    return bytes(output)


def build_svg(rows: list[tuple[str, dict[str, str], bytes]], law_sha: str) -> bytes:
    cells = all_cells()
    positions = {
        str(cell["id"]): nlevel.svg_position({key: str(value) for key, value in cell.items()})
        for cell in cells
    }
    anchor_positions = {
        identity: (170 + (index % 8) * 244, 100 if index < 8 else 1948)
        for index, (identity, _, _) in enumerate(ANCHORS)
    }
    anchor_positions["n_o0o"] = (1024, 1024)
    positions.update(anchor_positions)
    colours = {
        kind: f"#{(0x35A7BD + index * 0x0B71C3) & 0xFFFFFF:06x}"
        for index, kind in enumerate(EVENT_KINDS)
    }
    fills = {"WHITE": "#f8fafc", "BLACK": "#111827", "BROWN": "#a16207", "RAINBOW": "#c026d3"}
    by_tag: dict[str, list[dict[str, str]]] = {}
    for tag, fields, _ in rows:
        by_tag.setdefault(tag, []).append(fields)
    output = [
        f"<!-- {READFIRST} -->",
        '<svg xmlns="http://www.w3.org/2000/svg" width="2048" height="2048" viewBox="0 0 2048 2048" role="img" aria-labelledby="title desc" data-json="0" data-execution-authority="0">',
        '<title id="title">Round 3 Hookwall Brown Shadow FLOWesSH continuation</title>',
        f'<desc id="desc">Fifteen typed ledgers over 160 parent cells; open N; law SHA-256 {law_sha}</desc>',
        '<rect width="2048" height="2048" fill="#050816"/>',
        '<circle cx="1024" cy="1024" r="900" fill="none" stroke="#334155" stroke-width="4"/>',
        '<g id="relations" fill="none">',
    ]
    for kind in EVENT_KINDS:
        for fields in by_tag[kind]:
            x1, y1 = positions[fields["from"]]
            x2, y2 = positions[fields["to"]]
            output.append(
                f'<line id="{html.escape(fields["id"])}" data-kind="{kind}" data-round="3" '
                f'x1="{x1}" y1="{y1}" x2="{x2}" y2="{y2}" stroke="{colours[kind]}" stroke-width="1" opacity="0.34"/>'
            )
    output.append("</g>")
    output.append('<g id="cells" stroke="#94a3b8" stroke-width="1">')
    for cell in cells:
        x, y = positions[str(cell["id"])]
        output.append(
            f'<circle id="{cell["id"]}" data-kind="CELL_REF" cx="{x}" cy="{y}" r="5" fill="{fills[str(cell["colour"])]}"/>'
        )
    output.append("</g>")
    output.append('<g id="anchors" fill="#ffffff" font-family="monospace" font-size="18">')
    for identity, token, _ in ANCHORS:
        x, y = positions[identity]
        output.append(f'<circle id="{identity}" data-kind="ANCHOR" cx="{x}" cy="{y}" r="8"/><text x="{x + 10}" y="{y + 6}">{html.escape(token)}</text>')
    output.extend(
        (
            '<circle id="n_o0o" data-kind="PARENT_ANCHOR_REF" cx="1024" cy="1024" r="14" fill="#facc15"/>',
            "</g>",
            '<text x="40" y="2020" fill="#cbd5e1" font-family="monospace" font-size="20">N=OPEN | round=3 | relations=2400 | HBI HBP SHA SH HASH | json=0</text>',
            "</svg>",
        )
    )
    return ("\n".join(output) + "\n").encode("utf-8")


def artifacts() -> dict[Path, bytes]:
    builder = canonical_bytes(BUILDER_PATH)
    nlevel_builder = canonical_bytes(NLEVEL_BUILDER_PATH)
    spread_builder = canonical_bytes(spread.BUILDER_PATH)
    law = canonical_bytes(LAW_PATH)
    parent = canonical_bytes(PARENT_FLOWE_PATH)
    flowe = build_flowe(law, parent, builder)
    rows = parse_rows(flowe)
    validate(rows, law, parent, builder)
    hbp = build_hbp(flowe, law, parent, builder)
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
    output = {FLOWE_PATH: flowe, HBP_PATH: hbp, HBI_PATH: hbi, SVG_PATH: svg}
    for path, data in sealed.items():
        output[path.with_name(path.name + ".sha256")] = f"{sha256(data)}  {path.name}\n".encode()
    manifest_items = {
        NLEVEL_BUILDER_PATH: nlevel_builder,
        spread.BUILDER_PATH: spread_builder,
        PARENT_FLOWE_PATH: parent,
        **sealed,
    }
    output[MANIFEST_PATH] = (
        "\n".join(
            f"{sha256(data)}  {path.relative_to(ROOT).as_posix()}"
            for path, data in sorted(manifest_items.items(), key=lambda item: item[0].as_posix())
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
        raise SystemExit("HOOKWALL_BUILD|PASS=0|mismatch=" + ",".join(mismatches))
    flowe = expected[FLOWE_PATH]
    hbp = expected[HBP_PATH]
    hbi = expected[HBI_PATH]
    print(
        f"HOOKWALL_BUILD|PASS=1|mode={'check' if check else 'write'}"
        f"|round={ROUND}|cells={CELL_COUNT}|event_kinds={len(EVENT_KINDS)}"
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
