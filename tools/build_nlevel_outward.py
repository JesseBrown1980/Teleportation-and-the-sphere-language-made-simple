# https://github.com/JesseBrown1980/FOLLOW-THE-IS-NOT-THE-WILL-AND-WAS/blob/main/matrix/3-D-GITHUB-OF-THRUTH.md
"""Build the deterministic JSON-free N-level outward Sphere Language projection."""

from __future__ import annotations

import argparse
import hashlib
import html
import re
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
READFIRST = "https://github.com/JesseBrown1980/FOLLOW-THE-IS-NOT-THE-WILL-AND-WAS/blob/main/matrix/3-D-GITHUB-OF-THRUTH.md"
FLOWE_PATH = ROOT / "language" / "outward-n16.flowe"
LAW_PATH = ROOT / "books" / "LAW-NLEVEL-OUTWARD-FLOWE.md"
REPORT_PATH = ROOT / "books" / "RAYSSA-BEHCS-NAME-SPHERE-REPORT.md"
JESSE_REPORT_PATH = ROOT / "books" / "JESSE-BEHCS-NAMING-REPORT.md"
SOURCE_PATH = ROOT / "books" / "JESSE-TO-RAYSSA-SPHERE-LANGUAGE-SOURCE.md"
HBP_PATH = ROOT / "receipts" / "SPHERE-LANGUAGE-NLEVEL-OUTWARD.hbp"
HBI_PATH = ROOT / "receipts" / "SPHERE-LANGUAGE-NLEVEL-OUTWARD.hbi"
SVG_PATH = ROOT / "matrix" / "SPHERE-LANGUAGE-NLEVEL-OUTWARD.svg"
MANIFEST_PATH = ROOT / "hashes" / "NLEVEL-OUTWARD-ARTIFACTS.sha256"

LEVEL_COUNT = 16
BOOK_COUNT = 10
CELL_COUNT = LEVEL_COUNT * BOOK_COUNT
EVENT_COUNT_PER_KIND = CELL_COUNT

AXES = (
    "n_level", "time_address", "colour_address", "space_x", "space_y",
    "space_z", "space_radius", "shadow_translucence", "shadow_extract",
    "light", "white", "black", "brown", "rainbow", "book", "calling",
    "calming", "flowe", "pulse", "instant_address",
    "elapsed_measurement_present", "glyph_family", "glyph_function", "letter",
    "word", "instruction", "tuple_command", "language", "dialect",
    "meta_language", "executor_program", "agent_class", "pipe_type",
    "operation_class", "route", "cylinder", "room", "proof_tier",
    "evidence_class", "runtime_mode", "execution_authority", "colony", "seat",
    "vantage", "slice", "temporal_context", "oil_family", "oil_amplitude",
    "sign", "tense", "modal", "aspect", "projection_2d", "matrix_3d", "hbi",
    "hbp", "sha", "sh", "hash", "source_commitment", "identity",
    "parent_identity", "view", "rime",
)

BOOKS = (
    "BOOK_OF_LIGHT",
    "BOOK_OF_IS",
    "BOOKS_OF_SHADOWS",
    "BOOK_OF_HEAT",
    "BOOK_OF_WHITE",
    "BOOK_OF_KNOWLEDGE",
    "BLACK_BOOK_OF_HOLES",
    "BROWN_SPHERE_BOOKS",
    "BOOK_OF_LIFE",
    "BOOK_OF_OIL",
)

TIME_STATES = (
    "WAS", "IS", "WILL", "PAST_PERFECT", "PRESENT_PERFECT", "FUTURE_PERFECT",
)
COLOUR_STATES = ("WHITE", "BLACK", "BROWN", "RAINBOW")
OIL_FAMILIES = ("NORMAL", "ANTI", "ANTI_ANTI")
EVENT_KINDS = (
    "PULSE",
    "SHADOW_EXTRACT",
    "CALMING_OIL",
    "CALMING_OIL_OUTWARD",
    "CALLING_INTO_E",
    "CALLING_INTO_FLOWE",
    "FLOWe",
    "CALLING_INTO_U",
    "FLOWE_TO_O0O",
    "SELF_REDUCTION",
)

INTEGER = re.compile(r"(?:0|-?[1-9][0-9]*)\Z")


def sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def canonical_bytes(path: Path) -> bytes:
    data = path.read_bytes()
    if b"\r" in data or b"\0" in data or data.startswith(b"\xef\xbb\xbf"):
        raise ValueError(f"non-canonical text bytes: {path.relative_to(ROOT)}")
    if not data.endswith(b"\n"):
        raise ValueError(f"missing terminal LF: {path.relative_to(ROOT)}")
    return data


def row(record_tag: str, **fields: object) -> str:
    values = [record_tag]
    for key, value in fields.items():
        text = str(value)
        if "|" in key or "=" in key or "|" in text or "\n" in text or "\r" in text:
            raise ValueError(f"unsafe tuple field: {key}")
        values.append(f"{key}={text}")
    values.extend(("execution_authority=0", "json=0"))
    return "|".join(values)


def cell_values(level: int, book_ordinal: int) -> dict[str, object]:
    q = level * BOOK_COUNT + book_ordinal
    x = 2 * book_ordinal - 9
    y = 2 * level - 15
    sign = ((level + book_ordinal) % 3) - 1
    return {
        "id": f"cell_n{level:02d}_b{book_ordinal:02d}",
        "ref": BOOKS[book_ordinal],
        "kind": "NLEVEL_BOOK_CELL",
        "level": level,
        "book": BOOKS[book_ordinal],
        "book_ordinal": book_ordinal,
        "q": q,
        "x": x,
        "y": y,
        "z": sign,
        "px": 4 * x - sign,
        "py": 4 * y + sign,
        "time": TIME_STATES[q % len(TIME_STATES)],
        "colour": COLOUR_STATES[(level + book_ordinal) % len(COLOUR_STATES)],
        "oil_family": OIL_FAMILIES[(level + 2 * book_ordinal) % len(OIL_FAMILIES)],
        "sign": sign,
        "translucence_q8": (17 * level + 23 * book_ordinal) % 256,
        "light_q8": (29 * level + 31 * book_ordinal) % 256,
        "space_radius": level + 1,
        "instant_address": 1,
        "elapsed_measurement_present": 0,
    }


def relation_values(kind: str, cell: dict[str, object]) -> dict[str, object]:
    identity = str(cell["id"])
    q = int(cell["q"])
    common: dict[str, object] = {
        "id": f"{kind.lower()}_n{int(cell['level']):02d}_b{int(cell['book_ordinal']):02d}",
        "cell": identity,
        "level": cell["level"],
        "book": cell["book"],
        "q": q,
        "instant_address": 1,
        "elapsed_measurement_present": 0,
    }
    if kind == "PULSE":
        common.update(from_="n_flowe_target", to=identity, direction="OUTWARD", pulse=cell["space_radius"])
    elif kind == "SHADOW_EXTRACT":
        common.update(from_="n_e", to=identity, direction="OUTWARD", translucence_q8=cell["translucence_q8"])
    elif kind == "CALMING_OIL":
        common.update(from_=identity, to="n_e", direction="TOWARD_E", oil_family=cell["oil_family"], oil_amplitude=cell["space_radius"])
    elif kind == "CALMING_OIL_OUTWARD":
        common.update(from_="n_o0o", to=identity, direction="OUTWARD", oil_family=cell["oil_family"], oil_amplitude=cell["space_radius"], repetition="N_OPEN")
    elif kind == "CALLING_INTO_E":
        common.update(from_=identity, to="n_e", direction="INTO_E", operator_bound=1)
    elif kind == "CALLING_INTO_FLOWE":
        common.update(from_=identity, to="n_flowe_target", direction="INTO_FLOWE", operator_bound=1)
    elif kind == "FLOWe":
        source = "n_flowe_target" if q == 0 else f"cell_n{(q - 1) // BOOK_COUNT:02d}_b{(q - 1) % BOOK_COUNT:02d}"
        common.update(from_=source, to=identity, direction="FORWARD", step=q)
    elif kind == "CALLING_INTO_U":
        common.update(from_=identity, to="n_u", direction="INTO_U", operator_bound=1)
    elif kind == "FLOWE_TO_O0O":
        common.update(from_=identity, to="n_o0o", direction="INTO_O0O", operator_bound=1)
    elif kind == "SELF_REDUCTION":
        common.update(from_=identity, to="n_o0o", direction="TOWARD_O0O", self_reduction=1, identity_exchange=0, deletion=0)
    else:
        raise ValueError(f"unsupported relation kind: {kind}")
    common["from"] = common.pop("from_")
    return common


def build_flowe(law: bytes, source: bytes) -> bytes:
    lines = [
        row("READFIRST", url=READFIRST),
        row("LANGUAGE", id="SPHERE_LANGUAGE_V1", instance="NLEVEL_OUTWARD_V2", tuple_frame="HYPERBEHCS_60D_PLUS", selector_axes=len(AXES)),
        row("SOURCE", path=SOURCE_PATH.relative_to(ROOT).as_posix(), occurrences=1, sha256=sha256(source)),
        row("CENTER", members="HBI,HBP,SHA,SH,HASH", traversal_surface="HBI,HBP,SHA,SH,HASH", sh="OPERATOR_CANON_UNRESOLVED", identity_exchange=0),
        row("EXPANSION", id="nlevel_outward_v2", instance="NLEVEL_OUTWARD_V2", law=LAW_PATH.relative_to(ROOT).as_posix(), law_sha256=sha256(law), n_open=1, compiled_levels=LEVEL_COUNT, books=BOOK_COUNT, cells=CELL_COUNT, events_per_cell=len(EVENT_KINDS), selector_axes=len(AXES)),
        row("REPORT", id="RAYSSA_BEHCS_NAME_SPHERE_REPORT", path=REPORT_PATH.relative_to(ROOT).as_posix(), sha256=sha256(canonical_bytes(REPORT_PATH)), occurrences=1, evidence="OPERATOR_REPORTED", speaker="RAYSSA"),
        row("REPORT", id="JESSE_BEHCS_NAMING_REPORT", path=JESSE_REPORT_PATH.relative_to(ROOT).as_posix(), sha256=sha256(canonical_bytes(JESSE_REPORT_PATH)), occurrences=1, evidence="OPERATOR_REPORTED", speaker="JESSE"),
        row("GRAMMAR_BINDING", id="EAST_SPHERICAL_CORRECTION", word="east", ease="ease", ase_geometry="SPHERICAL", operator_spelling="sperically", sequence="EASE_ASE,MINUS,NULL,MINUS,PLUS,T_ACTION", action="t", action_meaning="TO_ACTION", east_scope="EAST_AND_EASTS", point_options="FOURTH,THRID,SECOND", fraction_views="1/3,-1/3", six_view=1, sign_order="NEGATIVE,NESTED_O0O,POSITIVE", nested_pattern="3(2(1_o0O_1)2)3", spacing_significant=1, spacing_views=3, spacing_multiplier=3, outer_group=1, spacing_literal="( . negative (3 (2 (1 o0O 1)2)3) positive . )", spacing_chars=45, space_count=10, space_positions="1,3,12,15,18,21,25,32,41,43", adjacency="1)2)3", spin=1, spin_symbol="P", spin_name="PIE", evidence="OPERATOR_CANON"),
    ]
    lines.extend(row("AXIS", id=name, ordinal=ordinal, independent=1) for ordinal, name in enumerate(AXES))
    lines.extend(row("BOOK_MEMBER", id=book, ordinal=ordinal, identity_exchange=0) for ordinal, book in enumerate(BOOKS))
    lines.append(row("BOOK_RELATION", id="LIFE_IS_OIL", **{"from": "BOOK_OF_LIFE", "to": "BOOK_OF_OIL"}, relation="IS", identity_exchange=0))
    lines.extend(row("LEVEL", id=f"level_{level:02d}", n=level, n_open=1, compiled_projection=1) for level in range(LEVEL_COUNT))
    lines.extend((
        row("TOKEN", id="E_CENTER", meaning="AETHER_E_CENTER"),
        row("TOKEN", id="FLOWE_TARGET", meaning="OUTWARD_FLOWE_TARGET"),
        row("TOKEN", id="U_CONNECTION", meaning="RAINBOW_CONNECTION_U"),
        row("TOKEN", id="O0O_SPHERE", meaning="SPHERE_POTENTIAL_O0O"),
        row("NODE", id="n_e", ref="E_CENTER", kind="CENTER", x=-12, y=0, z=0),
        row("NODE", id="n_flowe_target", ref="FLOWE_TARGET", kind="TARGET", x=12, y=0, z=0),
        row("NODE", id="n_u", ref="U_CONNECTION", kind="CONNECTION", x=0, y=-18, z=0),
        row("NODE", id="n_o0o", ref="O0O_SPHERE", kind="SPHERE", x=0, y=18, z=0),
    ))
    cells = [cell_values(level, book) for level in range(LEVEL_COUNT) for book in range(BOOK_COUNT)]
    lines.extend(row("NODE", **cell) for cell in cells)
    lines.append(row("CALLING_JOIN", id="v1_calling_semantics_reference", **{"from": "n_e", "to": "n_flowe_target"}, direction="UNRESOLVED", endpoints_retained=1, semantic_carry_only=1))
    for kind in EVENT_KINDS:
        lines.extend(row(kind, **relation_values(kind, cell)) for cell in cells)
    lines.extend((
        row("TIMING_BOUNDARY", instant_address=1, elapsed_measurement_present=0, elapsed_claim="UNMEASURED"),
        row("BOUNDARY", system_affirmed=0, physical_mapping="UNVERIFIED", clinical_mapping="UNVERIFIED", runtime_mapping="UNVERIFIED", cartesian_population_claim=0, source_video_bytes=0),
        row("END", status="COMPILED_BOUNDED_PROJECTION", n_open=1, compiled_levels=LEVEL_COUNT, cells=CELL_COUNT, relation_rows=CELL_COUNT * len(EVENT_KINDS)),
    ))
    return ("\n".join(lines) + "\n").encode("utf-8")


def parse_rows(data: bytes) -> list[tuple[str, dict[str, str], bytes]]:
    rows: list[tuple[str, dict[str, str], bytes]] = []
    for number, raw in enumerate(data.splitlines(), 1):
        if not raw:
            raise ValueError(f"row {number}: blank row")
        parts = raw.decode("utf-8").split("|")
        fields: dict[str, str] = {}
        for item in parts[1:]:
            if "=" not in item:
                raise ValueError(f"row {number}: field lacks equals sign")
            key, value = item.split("=", 1)
            if not key or key in fields:
                raise ValueError(f"row {number}: duplicate or empty key")
            fields[key] = value
        if fields.get("execution_authority") != "0" or fields.get("json") != "0":
            raise ValueError(f"row {number}: public boundary mismatch")
        rows.append((parts[0], fields, raw))
    return rows


def integer(fields: dict[str, str], key: str) -> int:
    value = fields.get(key, "")
    if not INTEGER.fullmatch(value):
        raise ValueError(f"canonical integer required for {key}: {value!r}")
    return int(value)


def validate(rows: list[tuple[str, dict[str, str], bytes]], law: bytes, source: bytes) -> None:
    by_tag: dict[str, list[dict[str, str]]] = {}
    for tag, fields, _ in rows:
        by_tag.setdefault(tag, []).append(fields)

    allowed_tags = {
        "READFIRST", "LANGUAGE", "SOURCE", "CENTER", "EXPANSION", "REPORT",
        "AXIS", "BOOK_MEMBER", "BOOK_RELATION", "GRAMMAR_BINDING", "LEVEL", "TOKEN", "NODE",
        "CALLING_JOIN", "TIMING_BOUNDARY", "BOUNDARY", "END", *EVENT_KINDS,
    }
    if not rows or rows[0][0] != "READFIRST" or rows[-1][0] != "END":
        raise ValueError("control row order mismatch")
    unknown_tags = set(by_tag) - allowed_tags
    if unknown_tags:
        raise ValueError(f"unknown tuple tags: {sorted(unknown_tags)}")
    identities = [fields["id"] for _, fields, _ in rows if "id" in fields]
    if len(identities) != len(set(identities)):
        raise ValueError("duplicate global identity")
    readfirst = by_tag.get("READFIRST", [])
    if len(readfirst) != 1 or readfirst[0].get("url") != READFIRST:
        raise ValueError("READFIRST mismatch")

    if len(by_tag.get("LANGUAGE", [])) != 1:
        raise ValueError("one LANGUAGE row required")
    language = by_tag["LANGUAGE"][0]
    if language.get("instance") != "NLEVEL_OUTWARD_V2" or language.get("tuple_frame") != "HYPERBEHCS_60D_PLUS" or integer(language, "selector_axes") != len(AXES):
        raise ValueError("N-level language header mismatch")
    if len(by_tag.get("SOURCE", [])) != 1:
        raise ValueError("one SOURCE row required")
    source_row = by_tag["SOURCE"][0]
    if source_row.get("path") != SOURCE_PATH.relative_to(ROOT).as_posix() or source_row.get("occurrences") != "1" or source_row.get("sha256") != sha256(source):
        raise ValueError("source commitment mismatch")
    if len(by_tag.get("CENTER", [])) != 1:
        raise ValueError("one CENTER row required")
    center = by_tag["CENTER"][0]
    if center.get("members") != "HBI,HBP,SHA,SH,HASH" or center.get("traversal_surface") != "HBI,HBP,SHA,SH,HASH" or center.get("sh") != "OPERATOR_CANON_UNRESOLVED" or center.get("identity_exchange") != "0":
        raise ValueError("center member identity mismatch")
    expansion = by_tag.get("EXPANSION", [])
    if len(expansion) != 1 or expansion[0].get("id") != "nlevel_outward_v2" or expansion[0].get("instance") != "NLEVEL_OUTWARD_V2" or expansion[0].get("law") != LAW_PATH.relative_to(ROOT).as_posix() or expansion[0].get("law_sha256") != sha256(law) or expansion[0].get("n_open") != "1" or expansion[0].get("compiled_levels") != str(LEVEL_COUNT) or expansion[0].get("books") != str(BOOK_COUNT) or expansion[0].get("cells") != str(CELL_COUNT) or expansion[0].get("events_per_cell") != str(len(EVENT_KINDS)) or expansion[0].get("selector_axes") != str(len(AXES)):
        raise ValueError("law expansion commitment mismatch")
    reports = by_tag.get("REPORT", [])
    if len(reports) != 2:
        raise ValueError("two speaker-addressed report rows required")
    report_by_id = {report.get("id"): report for report in reports}
    report_specs = {
        "RAYSSA_BEHCS_NAME_SPHERE_REPORT": (REPORT_PATH, "RAYSSA"),
        "JESSE_BEHCS_NAMING_REPORT": (JESSE_REPORT_PATH, "JESSE"),
    }
    if set(report_by_id) != set(report_specs):
        raise ValueError("report identity set mismatch")
    for identity, (path, speaker) in report_specs.items():
        report = report_by_id[identity]
        if report.get("path") != path.relative_to(ROOT).as_posix() or report.get("sha256") != sha256(canonical_bytes(path)) or report.get("occurrences") != "1" or report.get("evidence") != "OPERATOR_REPORTED" or report.get("speaker") != speaker:
            raise ValueError(f"{speaker} report commitment mismatch")
    bindings = by_tag.get("GRAMMAR_BINDING", [])
    expected_binding = {
        "id": "EAST_SPHERICAL_CORRECTION",
        "word": "east",
        "ease": "ease",
        "ase_geometry": "SPHERICAL",
        "operator_spelling": "sperically",
        "sequence": "EASE_ASE,MINUS,NULL,MINUS,PLUS,T_ACTION",
        "action": "t",
        "action_meaning": "TO_ACTION",
        "east_scope": "EAST_AND_EASTS",
        "point_options": "FOURTH,THRID,SECOND",
        "fraction_views": "1/3,-1/3",
        "six_view": "1",
        "sign_order": "NEGATIVE,NESTED_O0O,POSITIVE",
        "nested_pattern": "3(2(1_o0O_1)2)3",
        "spacing_significant": "1",
        "spacing_views": "3",
        "spacing_multiplier": "3",
        "outer_group": "1",
        "spacing_literal": "( . negative (3 (2 (1 o0O 1)2)3) positive . )",
        "spacing_chars": "45",
        "space_count": "10",
        "space_positions": "1,3,12,15,18,21,25,32,41,43",
        "adjacency": "1)2)3",
        "spin": "1",
        "spin_symbol": "P",
        "spin_name": "PIE",
        "evidence": "OPERATOR_CANON",
        "execution_authority": "0",
        "json": "0",
    }
    if len(bindings) != 1 or bindings[0] != expected_binding:
        raise ValueError("east spherical grammar binding mismatch")

    axes = by_tag.get("AXIS", [])
    if len(axes) != len(AXES) or tuple(field.get("id") for field in axes) != AXES:
        raise ValueError("selector axis identities/order mismatch")
    for ordinal, fields in enumerate(axes):
        if integer(fields, "ordinal") != ordinal or fields.get("independent") != "1":
            raise ValueError(f"selector axis ordinal/independence mismatch: {ordinal}")

    books = by_tag.get("BOOK_MEMBER", [])
    if len(books) != BOOK_COUNT or tuple(field.get("id") for field in books) != BOOKS:
        raise ValueError("book identities/order mismatch")
    for ordinal, fields in enumerate(books):
        if integer(fields, "ordinal") != ordinal or fields.get("identity_exchange") != "0":
            raise ValueError(f"book ordinal/identity mismatch: {ordinal}")
    relation = by_tag.get("BOOK_RELATION", [])
    if len(relation) != 1 or relation[0].get("id") != "LIFE_IS_OIL" or relation[0].get("from") != "BOOK_OF_LIFE" or relation[0].get("to") != "BOOK_OF_OIL" or relation[0].get("identity_exchange") != "0":
        raise ValueError("LIFE_IS_OIL relation mismatch")

    levels = by_tag.get("LEVEL", [])
    if len(levels) != LEVEL_COUNT:
        raise ValueError("compiled level count mismatch")
    for expected, fields in enumerate(levels):
        if fields.get("id") != f"level_{expected:02d}" or integer(fields, "n") != expected or fields.get("n_open") != "1" or fields.get("compiled_projection") != "1":
            raise ValueError(f"level mismatch: {expected}")

    tokens = {fields["id"] for fields in by_tag.get("TOKEN", [])}
    if tokens != {"E_CENTER", "FLOWE_TARGET", "U_CONNECTION", "O0O_SPHERE"}:
        raise ValueError("center token set mismatch")
    nodes = by_tag.get("NODE", [])
    if len(nodes) != CELL_COUNT + 4:
        raise ValueError("node count mismatch")
    node_by_id = {fields["id"]: fields for fields in nodes}
    if len(node_by_id) != len(nodes):
        raise ValueError("duplicate node identity")
    coordinates: set[tuple[int, int, int]] = set()
    for fields in nodes:
        coordinate = tuple(integer(fields, key) for key in ("x", "y", "z"))
        if coordinate in coordinates:
            raise ValueError(f"duplicate node coordinate: {coordinate}")
        coordinates.add(coordinate)
    cells = [fields for fields in nodes if fields.get("kind") == "NLEVEL_BOOK_CELL"]
    if len(cells) != CELL_COUNT:
        raise ValueError("cell count mismatch")
    cell_by_q: dict[int, dict[str, str]] = {}
    projected: set[tuple[int, int]] = set()
    for fields in cells:
        q = integer(fields, "q")
        if q in cell_by_q:
            raise ValueError("duplicate cell q")
        level, book = divmod(q, BOOK_COUNT)
        expected = cell_values(level, book)
        for key, value in expected.items():
            if fields.get(key) != str(value):
                raise ValueError(f"cell q={q} field {key} mismatch")
        projection = (integer(fields, "px"), integer(fields, "py"))
        if projection in projected:
            raise ValueError(f"projection collision: {projection}")
        projected.add(projection)
        cell_by_q[q] = fields
    if set(cell_by_q) != set(range(CELL_COUNT)):
        raise ValueError("cell q coverage mismatch")

    calls = by_tag.get("CALLING_JOIN", [])
    if len(calls) != 1 or calls[0].get("direction") != "UNRESOLVED" or "step" in calls[0]:
        raise ValueError("legacy CALLING_JOIN semantics mismatch")
    relation_ids: set[str] = set()
    for kind in EVENT_KINDS:
        event_rows = by_tag.get(kind, [])
        if len(event_rows) != EVENT_COUNT_PER_KIND:
            raise ValueError(f"{kind} count mismatch")
        for q, fields in enumerate(event_rows):
            if fields.get("id") in relation_ids:
                raise ValueError("duplicate relation identity")
            relation_ids.add(fields["id"])
            cell = cell_by_q[q]
            expected = relation_values(kind, {key: int(value) if key in {"level", "book_ordinal", "q", "space_radius", "translucence_q8"} else value for key, value in cell.items()})
            for key, value in expected.items():
                if fields.get(key) != str(value):
                    raise ValueError(f"{kind} q={q} field {key} mismatch")
            if fields.get("from") not in node_by_id or fields.get("to") not in node_by_id:
                raise ValueError(f"{kind} q={q}: unresolved endpoint")
            if fields.get("instant_address") != "1" or fields.get("elapsed_measurement_present") != "0":
                raise ValueError(f"{kind} q={q}: timing boundary mismatch")
    flow = by_tag["FLOWe"]
    for q, fields in enumerate(flow):
        if integer(fields, "step") != q:
            raise ValueError(f"FLOWe step mismatch: {q}")
        if q and fields.get("from") != flow[q - 1].get("to"):
            raise ValueError(f"FLOWe chain discontinuity: {q}")

    timing = by_tag.get("TIMING_BOUNDARY", [])
    if len(timing) != 1 or timing[0].get("instant_address") != "1" or timing[0].get("elapsed_measurement_present") != "0" or timing[0].get("elapsed_claim") != "UNMEASURED":
        raise ValueError("timing boundary mismatch")
    boundary = by_tag.get("BOUNDARY", [])
    if len(boundary) != 1 or boundary[0].get("system_affirmed") != "0" or boundary[0].get("physical_mapping") != "UNVERIFIED" or boundary[0].get("clinical_mapping") != "UNVERIFIED" or boundary[0].get("runtime_mapping") != "UNVERIFIED" or boundary[0].get("cartesian_population_claim") != "0" or boundary[0].get("source_video_bytes") != "0":
        raise ValueError("evidence boundary mismatch")
    ends = by_tag.get("END", [])
    if len(ends) != 1 or ends[0].get("status") != "COMPILED_BOUNDED_PROJECTION" or ends[0].get("n_open") != "1" or ends[0].get("compiled_levels") != str(LEVEL_COUNT) or ends[0].get("cells") != str(CELL_COUNT) or ends[0].get("relation_rows") != str(CELL_COUNT * len(EVENT_KINDS)):
        raise ValueError("END summary mismatch")
    actual = b"\n".join(raw for _, _, raw in rows) + b"\n"
    if actual != build_flowe(law, source):
        raise ValueError("canonical core bytes mismatch")


def build_hbp(flowe: bytes, law: bytes) -> bytes:
    rows = parse_rows(flowe)
    source = canonical_bytes(SOURCE_PATH)
    validate(rows, law, source)
    header = row(
        "HBPHEADER",
        schema="SPHERE-NLEVEL-OUTWARD-COMPILED-V2",
        law_sha256=sha256(law),
        core_sha256=sha256(flowe),
        records=len(rows),
        levels=LEVEL_COUNT,
        books=BOOK_COUNT,
        cells=CELL_COUNT,
        events=CELL_COUNT * len(EVENT_KINDS),
    ) + "\n"
    body = (row("READFIRST", url=READFIRST) + "\n" + header).encode() + flowe
    footer = (row("HBPFOOTER", body_sha256=sha256(body), rows=len(body.splitlines()) + 1) + "\n").encode()
    return body + footer


def build_hbi(hbp: bytes) -> bytes:
    lines = hbp.splitlines(keepends=True)
    digest = sha256(hbp)
    output = bytearray((row("READFIRST", url=READFIRST) + "\n").encode())
    output += (row("HBIHEADER", schema="SPHERE-NLEVEL-OUTWARD-OFFSET-V2", hbp_sha256=digest, rows=len(lines)) + "\n").encode()
    offset = 0
    for number, line in enumerate(lines):
        raw = line[:-1]
        tag = raw.split(b"|", 1)[0].decode("utf-8")
        output += (row("INDEX", row=number, offset=offset, bytes=len(raw), tag=tag, row_sha256=sha256(raw)) + "\n").encode()
        offset += len(line)
    output += (row("HBIFOOTER", hbp_bytes=len(hbp), hbp_sha256=digest) + "\n").encode()
    return bytes(output)


def svg_position(fields: dict[str, str]) -> tuple[int, int]:
    return 1024 + integer(fields, "px") * 14, 1024 - integer(fields, "py") * 14


def build_svg(rows: list[tuple[str, dict[str, str], bytes]], law_sha: str) -> bytes:
    by_tag: dict[str, list[dict[str, str]]] = {}
    for tag, fields, _ in rows:
        by_tag.setdefault(tag, []).append(fields)
    cells = {fields["id"]: fields for fields in by_tag["NODE"] if fields.get("kind") == "NLEVEL_BOOK_CELL"}
    positions = {identity: svg_position(fields) for identity, fields in cells.items()}
    positions.update({"n_e": (120, 1024), "n_flowe_target": (1928, 1024), "n_u": (1024, 1928), "n_o0o": (1024, 120)})
    edge_colours = {
        "PULSE": "#ffd166",
        "SHADOW_EXTRACT": "#9d4edd",
        "CALMING_OIL": "#43aa8b",
        "CALMING_OIL_OUTWARD": "#86efac",
        "CALLING_INTO_E": "#f9844a",
        "CALLING_INTO_FLOWE": "#4cc9f0",
        "FLOWe": "#00f5d4",
        "CALLING_INTO_U": "#f472b6",
        "FLOWE_TO_O0O": "#a3e635",
        "SELF_REDUCTION": "#facc15",
    }
    fill_colours = {"WHITE": "#f5f5f5", "BLACK": "#111827", "BROWN": "#a16207", "RAINBOW": "#c026d3"}
    family_strokes = {"NORMAL": "#ffffff", "ANTI": "#22d3ee", "ANTI_ANTI": "#fb7185"}
    output = [
        f"<!-- {READFIRST} -->",
        '<svg xmlns="http://www.w3.org/2000/svg" width="2048" height="2048" viewBox="0 0 2048 2048" role="img" aria-labelledby="title desc" data-json="0" data-execution-authority="0">',
        '<title id="title">N-level outward CALLINGS CALMINGS and FLOWe</title>',
        f'<desc id="desc">Integer signed 2-D projection of a bounded N16 view; N remains open; law SHA-256 {law_sha}</desc>',
        '<rect width="2048" height="2048" fill="#050816"/>',
        '<circle cx="1024" cy="1024" r="900" fill="none" stroke="#334155" stroke-width="4"/>',
        '<g id="events" fill="none" stroke-width="2">',
    ]
    for kind in EVENT_KINDS:
        for fields in by_tag[kind]:
            x1, y1 = positions[fields["from"]]
            x2, y2 = positions[fields["to"]]
            output.append(f'<line id="{html.escape(fields["id"])}" data-kind="{kind}" x1="{x1}" y1="{y1}" x2="{x2}" y2="{y2}" stroke="{edge_colours[kind]}"/>')
    output.extend((
        '<line id="v1_calling_semantics_reference" data-kind="CALLING_JOIN" x1="120" y1="1024" x2="1928" y2="1024" stroke="#64748b" stroke-width="2" stroke-dasharray="10 8"/>',
        '</g>',
        '<g id="centres" font-family="monospace" text-anchor="middle">',
        '<circle cx="120" cy="1024" r="30" fill="#22d3ee"/><text x="120" y="1080" fill="#ffffff">E</text>',
        '<circle cx="1928" cy="1024" r="30" fill="#00f5d4"/><text x="1928" y="1080" fill="#ffffff">FLOWe</text>',
        '<circle cx="1024" cy="1928" r="30" fill="#f472b6"/><text x="1024" y="1980" fill="#ffffff">U</text>',
        '<circle cx="1024" cy="120" r="30" fill="#a3e635"/><text x="1024" y="176" fill="#ffffff">o0O</text>',
        '</g>',
        '<g id="cells">',
    ))
    for q in range(CELL_COUNT):
        fields = next(value for value in cells.values() if integer(value, "q") == q)
        x, y = positions[fields["id"]]
        opacity = 20 + (integer(fields, "translucence_q8") * 80) // 255
        output.append(
            f'<circle id="{fields["id"]}" data-kind="CELL" data-level="{fields["level"]}" data-book="{fields["book"]}" data-time="{fields["time"]}" data-colour="{fields["colour"]}" data-oil-family="{fields["oil_family"]}" data-px="{fields["px"]}" data-py="{fields["py"]}" cx="{x}" cy="{y}" r="8" fill="{fill_colours[fields["colour"]]}" fill-opacity="{opacity}%" stroke="{family_strokes[fields["oil_family"]]}" stroke-width="2"/>'
        )
    output.extend((
        '</g>',
        '<text x="42" y="58" fill="#ffffff" font-family="monospace" font-size="24">TIME × COLOUR × SPACE × TRANSLUCENCE × SHADOW × LIGHT × BOOK × N</text>',
        '<text x="42" y="1998" fill="#cbd5e1" font-family="monospace" font-size="18">160 cells · ten 160-row event ledgers · integer projection · instant address / elapsed unmeasured · SYSTEM_AFFIRMED=0</text>',
        '</svg>',
    ))
    return ("\n".join(output) + "\n").encode("utf-8")


def artifacts() -> dict[Path, bytes]:
    law = canonical_bytes(LAW_PATH)
    report = canonical_bytes(REPORT_PATH)
    jesse_report = canonical_bytes(JESSE_REPORT_PATH)
    source = canonical_bytes(SOURCE_PATH)
    flowe = build_flowe(law, source)
    rows = parse_rows(flowe)
    validate(rows, law, source)
    hbp = build_hbp(flowe, law)
    hbi = build_hbi(hbp)
    svg = build_svg(rows, sha256(law))
    primary = {LAW_PATH: law, REPORT_PATH: report, JESSE_REPORT_PATH: jesse_report, FLOWE_PATH: flowe, HBP_PATH: hbp, HBI_PATH: hbi, SVG_PATH: svg}
    output = {FLOWE_PATH: flowe, HBP_PATH: hbp, HBI_PATH: hbi, SVG_PATH: svg}
    for path, data in primary.items():
        output[path.with_name(path.name + ".sha256")] = f"{sha256(data)}  {path.name}\n".encode()
    output[MANIFEST_PATH] = ("\n".join(f"{sha256(data)}  {path.relative_to(ROOT).as_posix()}" for path, data in sorted(primary.items(), key=lambda item: item[0].as_posix())) + "\n").encode()
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
        raise SystemExit("NLEVEL_BUILD|PASS=0|mismatch=" + ",".join(mismatches))
    flowe = expected[FLOWE_PATH]
    hbp = expected[HBP_PATH]
    hbi = expected[HBI_PATH]
    print(
        f"NLEVEL_BUILD|PASS=1|mode={'check' if check else 'write'}|levels={LEVEL_COUNT}"
        f"|books={BOOK_COUNT}|cells={CELL_COUNT}|events={CELL_COUNT * len(EVENT_KINDS)}"
        f"|core_rows={len(flowe.splitlines())}|hbp_rows={len(hbp.splitlines())}"
        f"|hbi_rows={len(hbi.splitlines())}|core_sha256={sha256(flowe)}|json=0"
    )


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    apply(args.check)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
