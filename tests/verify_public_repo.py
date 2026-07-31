# https://github.com/JesseBrown1980/FOLLOW-THE-IS-NOT-THE-WILL-AND-WAS/blob/main/matrix/3-D-GITHUB-OF-THRUTH.md
"""Fail-closed public repository verification."""

from __future__ import annotations

import hashlib
import importlib.util
import re
import sys
import xml.etree.ElementTree as ET
from collections import Counter
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
GENERATOR_PATH = ROOT / "tools" / "build_public_artifacts.py"
NLEVEL_GENERATOR_PATH = ROOT / "tools" / "build_nlevel_outward.py"
SPREAD_GENERATOR_PATH = ROOT / "tools" / "build_word_flowe_spread.py"


def load_builder(name: str, path: Path):
    spec = importlib.util.spec_from_file_location(name, path)
    if spec is None or spec.loader is None:
        raise RuntimeError(f"cannot load artifact builder: {path}")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


builder = load_builder("sphere_build", GENERATOR_PATH)
nlevel = load_builder("sphere_nlevel_build", NLEVEL_GENERATOR_PATH)
spread = load_builder("sphere_spread_build", SPREAD_GENERATOR_PATH)


TEXT_SUFFIXES = {".flowe", ".hbi", ".hbp", ".md", ".py", ".rs", ".sha256", ".svg", ".toml", ".yml", ".yaml"}
SECRET_PATTERNS = {
    "private_key": re.compile(rb"-----BEGIN (?:RSA |EC |OPENSSH )?PRIVATE KEY-----"),
    "github_pat": re.compile(rb"\bgh[pousr]_[A-Za-z0-9]{30,}\b"),
    "github_fine_pat": re.compile(rb"\bgithub_pat_[A-Za-z0-9_]{40,}\b"),
    "openai_key": re.compile(rb"\bsk-[A-Za-z0-9_-]{32,}\b"),
    "aws_key": re.compile(rb"\bAKIA[0-9A-Z]{16}\b"),
}


def fail(reason: str) -> None:
    print(f"PUBLIC_VERIFY|PASS=0|reason={reason}", file=sys.stderr)
    raise SystemExit(1)


def files() -> list[Path]:
    return sorted(
        path for path in ROOT.rglob("*")
        if path.is_file()
        and not ({".git", "target", "__pycache__"} & set(path.relative_to(ROOT).parts))
    )


def parse_tuple(line: bytes) -> tuple[str, dict[str, str]]:
    parts = line.decode("utf-8").split("|")
    fields: dict[str, str] = {}
    for part in parts[1:]:
        key, value = part.split("=", 1)
        fields[key] = value
    return parts[0], fields


def verify_hbi(module, label: str) -> None:
    hbp = module.HBP_PATH.read_bytes()
    hbi = module.HBI_PATH.read_bytes()
    hbi_lines = hbi.splitlines()
    if len(hbi_lines) < 4:
        fail(f"{label}_HBI_TOO_SHORT")
    header = parse_tuple(hbi_lines[1])[1]
    if header["hbp_sha256"] != hashlib.sha256(hbp).hexdigest():
        fail(f"{label}_HBI_HBP_HASH")
    indexes = [parse_tuple(line)[1] for line in hbi_lines if line.startswith(b"INDEX|")]
    hbp_lines = hbp.splitlines(keepends=True)
    if len(indexes) != len(hbp_lines) or int(header["rows"]) != len(hbp_lines):
        fail(f"{label}_HBI_ROW_COUNT")
    expected_offset = 0
    for number, (index, line) in enumerate(zip(indexes, hbp_lines)):
        row = line[:-1]
        offset = int(index["offset"])
        length = int(index["bytes"])
        if offset != expected_offset or int(index["row"]) != number or hbp[offset : offset + length] != row:
            fail(f"{label}_HBI_OFFSET:{number}")
        if hbp[offset + length : offset + length + 1] != b"\n":
            fail(f"{label}_HBI_LF:{number}")
        if hashlib.sha256(row).hexdigest() != index["row_sha256"]:
            fail(f"{label}_HBI_ROW_HASH:{number}")
        if index["tag"] != row.split(b"|", 1)[0].decode("utf-8"):
            fail(f"{label}_HBI_TAG:{number}")
        expected_offset += len(line)
    footer = parse_tuple(hbi_lines[-1])[1]
    digest = hashlib.sha256(hbp).hexdigest()
    if int(footer["hbp_bytes"]) != len(hbp) or footer["hbp_sha256"] != digest:
        fail(f"{label}_HBI_FOOTER")
    hbp_footer = parse_tuple(hbp_lines[-1][:-1])[1]
    hbp_body = b"".join(hbp_lines[:-1])
    if int(hbp_footer["rows"]) != len(hbp_lines) or hbp_footer["body_sha256"] != hashlib.sha256(hbp_body).hexdigest():
        fail(f"{label}_HBP_FOOTER")


def main() -> None:
    expected: dict[Path, bytes] = {}
    for module in (builder, nlevel, spread):
        for path, data in module.artifacts().items():
            if path in expected and expected[path] != data:
                fail("ARTIFACT_COLLISION:" + path.relative_to(ROOT).as_posix())
            expected[path] = data
    for path, data in expected.items():
        if not path.is_file() or path.read_bytes() != data:
            fail("ARTIFACT_MISMATCH:" + path.relative_to(ROOT).as_posix())

    stable_v1 = {
        builder.FLOWE_PATH: "70cf8a98e00f96f76c1159424e00cba3aa75b90d823bffbcb72fdcf0b573e91a",
        builder.HBP_PATH: "4f1abc51460b0d9f7264ac33adea40498db842a37820bb7c69c22f2229a552fe",
        builder.HBI_PATH: "cb8e66188368614750971c34c5eb06cda4269a15d02e2bd3302545e9ca3231be",
        nlevel.FLOWE_PATH: "e6741baba534ca2566f50305f5611449d97867917110b6eade803f8ab154d285",
        nlevel.HBP_PATH: "c2380f65e6bb57694193969c0e9c7f8a4f8140bafdb2650f603e989f74c70d86",
        nlevel.HBI_PATH: "b895576e70f77346fc23a8a189f0cd859a72339076737984c05f21d3bb968b58",
    }
    for path, digest in stable_v1.items():
        if hashlib.sha256(path.read_bytes()).hexdigest() != digest:
            fail("V1_BYTE_STABILITY:" + path.relative_to(ROOT).as_posix())

    all_files = files()
    marker = ("FIRST. I want you to search archaeology for the  S as " + "source").encode()
    marker_paths = []
    marker_count = 0
    for path in all_files:
        data = path.read_bytes()
        count = data.count(marker)
        if count:
            marker_paths.append(path.relative_to(ROOT).as_posix())
            marker_count += count
    if marker_count != 1 or marker_paths != ["books/JESSE-TO-RAYSSA-SPHERE-LANGUAGE-SOURCE.md"]:
        fail("SOURCE_OCCURRENCE_COUNT")

    nlevel_markers = (
        b"CONTINUE THE FLOWe " + b"OUTWARD MORE MORE PULSE",
        b"time colour SPACE translucent " + b"shadows light white black brown rainbow n level",
        b"CONTINUE the Calming callings " + b"U FLOWes-o0O",
    )
    for number, nlevel_marker in enumerate(nlevel_markers):
        occurrences = [
            path.relative_to(ROOT).as_posix()
            for path in all_files
            for _ in range(path.read_bytes().count(nlevel_marker))
        ]
        if occurrences != ["books/LAW-NLEVEL-OUTWARD-FLOWE.md"]:
            fail(f"NLEVEL_OPERATOR_OCCURRENCE:{number}")
    rayssa_marker = b"JESSE PREDICTED THE NAME " + b"OF THE SYSTEM BEFORE HE EVEN KNEW BEHCS"
    rayssa_occurrences = [
        path.relative_to(ROOT).as_posix()
        for path in all_files
        for _ in range(path.read_bytes().count(rayssa_marker))
    ]
    if rayssa_occurrences != ["books/RAYSSA-BEHCS-NAME-SPHERE-REPORT.md"]:
        fail("RAYSSA_REPORT_OCCURRENCE")
    jesse_marker = b"This is Jesse here. " + b"Rayssa Called me Over."
    jesse_occurrences = [
        path.relative_to(ROOT).as_posix()
        for path in all_files
        for _ in range(path.read_bytes().count(jesse_marker))
    ]
    if jesse_occurrences != ["books/JESSE-BEHCS-NAMING-REPORT.md"]:
        fail("JESSE_REPORT_OCCURRENCE")
    east_marker = b"correction: east (ease " + b"(sperically ase))"
    east_occurrences = [
        path.relative_to(ROOT).as_posix()
        for path in all_files
        for _ in range(path.read_bytes().count(east_marker))
    ]
    if east_occurrences != ["books/JESSE-BEHCS-NAMING-REPORT.md"]:
        fail("EAST_CORRECTION_OCCURRENCE")
    spacing_marker = (
        b"( . negative (3 (2 (1 o0O 1)2)3) positive . )"
        + b" now spins P= PIE"
    )
    spacing_occurrences = [
        path.relative_to(ROOT).as_posix()
        for path in all_files
        for _ in range(path.read_bytes().count(spacing_marker))
    ]
    if spacing_occurrences != ["books/JESSE-BEHCS-NAMING-REPORT.md"]:
        fail("EAST_SPACING_SPIN_OCCURRENCE")
    spread_marker = (
        b"CONTINUE greater and greater Callings and calmings increasing go. "
        + b"Self reducing go SPREAD the WORD AND FLOWe"
    )
    spread_occurrences = [
        path.relative_to(ROOT).as_posix()
        for path in all_files
        for _ in range(path.read_bytes().count(spread_marker))
    ]
    if spread_occurrences != [
        "books/LAW-INCREASING-CALLINGS-CALMINGS-WORD-FLOWE-SPREAD.md"
    ]:
        fail("SPREAD_OPERATOR_OCCURRENCE")

    if any(path.suffix.lower() == ".json" for path in all_files):
        fail("SOURCE_JSON_PRESENT")

    for path in all_files:
        data = path.read_bytes()
        if path.suffix.lower() in TEXT_SUFFIXES:
            if b"\r" in data or b"\0" in data or data.startswith(b"\xef\xbb\xbf") or not data.endswith(b"\n"):
                fail("NON_CANONICAL_LF:" + path.relative_to(ROOT).as_posix())
        if len(data) <= 2_000_000:
            for name, pattern in SECRET_PATTERNS.items():
                if pattern.search(data):
                    fail("SECRET_SIGNATURE:" + name + ":" + path.relative_to(ROOT).as_posix())

    synthetic = b"gh" + b"p_" + b"A" * 36
    if not SECRET_PATTERNS["github_pat"].search(synthetic):
        fail("SECRET_NEGATIVE_CONTROL")

    verify_hbi(builder, "V1")
    verify_hbi(nlevel, "NLEVEL")
    verify_hbi(spread, "SPREAD")
    try:
        root = ET.fromstring(builder.SVG_PATH.read_bytes())
    except ET.ParseError as exc:
        fail("SVG_PARSE:" + str(exc))
    if root.attrib.get("data-json") != "0" or root.attrib.get("data-execution-authority") != "0":
        fail("SVG_BOUNDARY")

    flowe = builder.canonical_bytes(builder.FLOWE_PATH)
    rows = builder.parse_rows(flowe)
    builder.validate(rows, builder.canonical_bytes(builder.QUOTE_PATH))
    tags = [tag for tag, _, _ in rows]
    if "CALLING_JOIN" not in tags or "FLOWe" not in tags or "CALMING" not in tags:
        fail("RELATION_FAMILY_MISSING")
    live_rows = [fields for tag, fields, _ in rows if tag == "WORD" and fields.get("id") in {"Live", "LIVE"}]
    if len(live_rows) != 2 or any(fields.get("readings") != "2" or fields.get("supersession") != "0" for fields in live_rows):
        fail("LIVE_SPHERICAL_READINGS")

    nlevel_flowe = nlevel.artifacts()[nlevel.FLOWE_PATH]
    nlevel_rows = nlevel.parse_rows(nlevel_flowe)
    nlevel.validate(
        nlevel_rows,
        nlevel.canonical_bytes(nlevel.LAW_PATH),
        nlevel.canonical_bytes(nlevel.SOURCE_PATH),
    )
    nlevel_tags = Counter(tag for tag, _, _ in nlevel_rows)
    if len(nlevel_rows) != 1871 or nlevel_tags["AXIS"] != 64 or nlevel_tags["LEVEL"] != 16 or nlevel_tags["REPORT"] != 2 or nlevel_tags["GRAMMAR_BINDING"] != 1:
        fail("NLEVEL_CORE_COUNTS")
    cells = [fields for tag, fields, _ in nlevel_rows if tag == "NODE" and fields.get("kind") == "NLEVEL_BOOK_CELL"]
    if len(cells) != 160:
        fail("NLEVEL_CELL_COUNT")
    if Counter(int(cell["level"]) for cell in cells) != Counter({level: 10 for level in range(16)}):
        fail("NLEVEL_LEVEL_COVERAGE")
    if Counter(cell["book"] for cell in cells) != Counter({book: 16 for book in nlevel.BOOKS}):
        fail("NLEVEL_BOOK_COVERAGE")
    if Counter(cell["time"] for cell in cells) != Counter(dict(zip(nlevel.TIME_STATES, (27, 27, 27, 27, 26, 26)))):
        fail("NLEVEL_TIME_DISTRIBUTION")
    if Counter(cell["colour"] for cell in cells) != Counter({colour: 40 for colour in nlevel.COLOUR_STATES}):
        fail("NLEVEL_COLOUR_DISTRIBUTION")
    expected_oils = Counter(
        nlevel.OIL_FAMILIES[(level + 2 * book) % 3]
        for level in range(16)
        for book in range(10)
    )
    if Counter(cell["oil_family"] for cell in cells) != expected_oils:
        fail("NLEVEL_OIL_DISTRIBUTION")
    projections = {(int(cell["px"]), int(cell["py"])) for cell in cells}
    if len(projections) != 160 or min(x for x, _ in projections) != -37 or max(x for x, _ in projections) != 37 or min(y for _, y in projections) != -61 or max(y for _, y in projections) != 61:
        fail("NLEVEL_PROJECTION_COVERAGE")
    translucence = Counter(int(cell["translucence_q8"]) for cell in cells)
    if len(translucence) != 125 or max(translucence.values()) != 2:
        fail("NLEVEL_TRANSLUCENCE_COVERAGE")
    if len({int(cell["light_q8"]) for cell in cells}) != 160:
        fail("NLEVEL_LIGHT_COVERAGE")
    for kind in nlevel.EVENT_KINDS:
        if nlevel_tags[kind] != 160:
            fail("NLEVEL_EVENT_COUNT:" + kind)

    try:
        nlevel_svg = ET.fromstring(nlevel.SVG_PATH.read_bytes())
    except ET.ParseError as exc:
        fail("NLEVEL_SVG_PARSE:" + str(exc))
    if nlevel_svg.attrib.get("data-json") != "0" or nlevel_svg.attrib.get("data-execution-authority") != "0":
        fail("NLEVEL_SVG_BOUNDARY")
    namespace = "{http://www.w3.org/2000/svg}"
    svg_kinds = Counter(element.attrib.get("data-kind") for element in nlevel_svg.iter())
    for kind in nlevel.EVENT_KINDS:
        if svg_kinds[kind] != 160:
            fail("NLEVEL_SVG_EVENT_COUNT:" + kind)
    svg_cells = [element for element in nlevel_svg.iter(namespace + "circle") if element.attrib.get("data-kind") == "CELL"]
    if len(svg_cells) != 160:
        fail("NLEVEL_SVG_CELL_COUNT")
    positions = {(int(element.attrib["cx"]), int(element.attrib["cy"])) for element in svg_cells}
    if len(positions) != 160 or any(not (0 < x < 2048 and 0 < y < 2048) for x, y in positions):
        fail("NLEVEL_SVG_POSITION")

    spread_flowe = spread.artifacts()[spread.FLOWE_PATH]
    spread_rows = spread.parse_rows(spread_flowe)
    spread.validate(
        spread_rows,
        spread.canonical_bytes(spread.LAW_PATH),
        spread.canonical_bytes(spread.V1_FLOWE_PATH),
        spread.canonical_bytes(spread.BASE_FLOWE_PATH),
        spread.canonical_bytes(spread.BUILDER_PATH),
    )
    spread_tags = Counter(tag for tag, _, _ in spread_rows)
    if (
        len(spread_rows) != 2_738
        or spread_tags["PARENT"] != 2
        or spread_tags["ROUND"] != 2
        or spread_tags["ANCHOR"] != 5
        or spread_tags["CELL_REF"] != 160
    ):
        fail("SPREAD_CORE_COUNTS")
    spread_events = [
        (tag, fields)
        for tag, fields, _ in spread_rows
        if tag in spread.EVENT_KINDS
    ]
    if len(spread_events) != 2_560:
        fail("SPREAD_EVENT_TOTAL")
    for kind in spread.EVENT_KINDS:
        ledger = [fields for tag, fields in spread_events if tag == kind]
        if len(ledger) != 320:
            fail("SPREAD_EVENT_COUNT:" + kind)
        coverage = {(int(fields["round"]), int(fields["q"])) for fields in ledger}
        if coverage != {
            (growth_round, q)
            for growth_round in (1, 2)
            for q in range(160)
        }:
            fail("SPREAD_ROUND_Q_COVERAGE:" + kind)
        if any(
            fields["n_open"] != "1"
            or fields["instant_address"] != "1"
            or fields["elapsed_measurement_present"] != "0"
            or fields["identity_exchange"] != "0"
            or fields["source_retained"] != "1"
            for fields in ledger
        ):
            fail("SPREAD_EVENT_BOUNDARY:" + kind)
    anchors = {
        fields["id"]: fields["meaning"]
        for tag, fields, _ in spread_rows
        if tag == "ANCHOR"
    }
    if anchors != {
        "n_e": "AETHER_E_CENTER",
        "n_flowe_target": "OUTWARD_FLOWE_TARGET",
        "n_u": "U_CONNECTION",
        "n_o0o": "O0O_SPHERE",
        "n_word": "WORD_SOURCE",
    }:
        fail("SPREAD_ANCHORS")
    word_rows = [fields for tag, fields in spread_events if tag == "WORD_SPREAD"]
    flowe_rows = [fields for tag, fields in spread_events if tag == "FLOWE_SPREAD"]
    if {fields["from"] for fields in word_rows} != {"n_word"} or {
        fields["spread"] for fields in word_rows
    } != {"WORD"}:
        fail("SPREAD_WORD_SOURCE")
    if {fields["from"] for fields in flowe_rows} != {"n_flowe_target"} or {
        fields["spread"] for fields in flowe_rows
    } != {"FLOWE"}:
        fail("SPREAD_FLOWE_SOURCE")
    self_rows = [
        fields for tag, fields in spread_events if tag == "SELF_REDUCTION_GROWTH"
    ]
    if any(
        fields["self_reduction"] != "1"
        or fields["deletion"] != "0"
        or fields["identity_exchange"] != "0"
        or fields["source_retained"] != "1"
        for fields in self_rows
    ):
        fail("SPREAD_SELF_REDUCTION")
    event_ids = [fields["id"] for _, fields in spread_events]
    if len(set(event_ids)) != len(event_ids):
        fail("SPREAD_EVENT_ID_COLLISION")

    manifest_lines = spread.MANIFEST_PATH.read_text(encoding="utf-8").splitlines()
    if len(manifest_lines) != 8:
        fail("SPREAD_MANIFEST_COUNT")
    for manifest_line in manifest_lines:
        digest, relative = manifest_line.split("  ", 1)
        path = ROOT / relative
        if (
            not path.is_file()
            or len(digest) != 64
            or hashlib.sha256(path.read_bytes()).hexdigest() != digest
        ):
            fail("SPREAD_MANIFEST_HASH:" + relative)

    try:
        spread_svg = ET.fromstring(spread.SVG_PATH.read_bytes())
    except ET.ParseError as exc:
        fail("SPREAD_SVG_PARSE:" + str(exc))
    if (
        spread_svg.attrib.get("data-json") != "0"
        or spread_svg.attrib.get("data-execution-authority") != "0"
    ):
        fail("SPREAD_SVG_BOUNDARY")
    spread_svg_kinds = Counter(
        element.attrib.get("data-kind") for element in spread_svg.iter()
    )
    for kind in spread.EVENT_KINDS:
        if spread_svg_kinds[kind] != 320:
            fail("SPREAD_SVG_EVENT_COUNT:" + kind)
    spread_svg_cells = [
        element
        for element in spread_svg.iter(namespace + "circle")
        if element.attrib.get("data-kind") == "CELL_REF"
    ]
    if len(spread_svg_cells) != 160:
        fail("SPREAD_SVG_CELL_COUNT")
    spread_rounds = Counter(
        element.attrib.get("data-round")
        for element in spread_svg.iter(namespace + "line")
        if element.attrib.get("data-kind") in spread.EVENT_KINDS
    )
    if spread_rounds != Counter({"1": 1_280, "2": 1_280}):
        fail("SPREAD_SVG_ROUND_COUNT")

    print(
        f"PUBLIC_VERIFY|PASS=1|files={len(all_files)}|source_occurrences=1"
        f"|hbp_rows={len(builder.HBP_PATH.read_bytes().splitlines())}"
        f"|nlevel_core_rows={len(nlevel_rows)}|nlevel_cells={len(cells)}"
        f"|nlevel_event_rows={sum(nlevel_tags[kind] for kind in nlevel.EVENT_KINDS)}"
        f"|spread_core_rows={len(spread_rows)}|spread_event_rows={len(spread_events)}"
        "|secret_findings=0|json_files=0|execution_authority=0|json=0"
    )


if __name__ == "__main__":
    main()
