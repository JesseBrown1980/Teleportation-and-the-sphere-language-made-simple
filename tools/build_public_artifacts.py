# https://github.com/JesseBrown1980/FOLLOW-THE-IS-NOT-THE-WILL-AND-WAS/blob/main/matrix/3-D-GITHUB-OF-THRUTH.md
"""Build deterministic JSON-free Sphere Language public artifacts."""

from __future__ import annotations

import argparse
import hashlib
import html
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
READFIRST = "https://github.com/JesseBrown1980/FOLLOW-THE-IS-NOT-THE-WILL-AND-WAS/blob/main/matrix/3-D-GITHUB-OF-THRUTH.md"
FLOWE_PATH = ROOT / "language" / "core.flowe"
QUOTE_PATH = ROOT / "books" / "JESSE-TO-RAYSSA-SPHERE-LANGUAGE-SOURCE.md"
HBP_PATH = ROOT / "receipts" / "SPHERE-LANGUAGE-CALLINGS-CALMINGS-FLOWE.hbp"
HBI_PATH = ROOT / "receipts" / "SPHERE-LANGUAGE-CALLINGS-CALMINGS-FLOWE.hbi"
SVG_PATH = ROOT / "matrix" / "SPHERE-LANGUAGE-CALLINGS-CALMINGS-FLOWE.svg"
MANIFEST_PATH = ROOT / "hashes" / "PUBLIC-ARTIFACTS.sha256"


def sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def canonical_bytes(path: Path) -> bytes:
    data = path.read_bytes()
    if b"\r" in data or b"\0" in data or data.startswith(b"\xef\xbb\xbf"):
        raise ValueError(f"non-canonical text bytes: {path.relative_to(ROOT)}")
    if not data.endswith(b"\n"):
        raise ValueError(f"missing terminal LF: {path.relative_to(ROOT)}")
    return data


def parse_rows(data: bytes) -> list[tuple[str, dict[str, str], bytes]]:
    rows: list[tuple[str, dict[str, str], bytes]] = []
    for number, raw in enumerate(data.splitlines(), 1):
        if not raw:
            continue
        text = raw.decode("utf-8")
        parts = text.split("|")
        tag = parts[0]
        if not tag:
            raise ValueError(f"row {number}: empty tag")
        fields: dict[str, str] = {}
        for field in parts[1:]:
            if "=" not in field:
                raise ValueError(f"row {number}: field has no equals sign")
            key, value = field.split("=", 1)
            if not key or key in fields:
                raise ValueError(f"row {number}: invalid or duplicate key {key!r}")
            fields[key] = value
        if fields.get("json") != "0":
            raise ValueError(f"row {number}: json must equal 0")
        if fields.get("execution_authority") != "0":
            raise ValueError(f"row {number}: execution_authority must equal 0")
        rows.append((tag, fields, raw))
    return rows


def validate(rows: list[tuple[str, dict[str, str], bytes]], quote: bytes) -> None:
    glyphs: dict[str, dict[str, str]] = {}
    words: dict[str, dict[str, str]] = {}
    tokens: set[str] = set()
    nodes: dict[str, dict[str, str]] = {}
    coordinates: set[tuple[int, int, int]] = set()
    relation_ids: set[str] = set()
    sources = []

    for tag, fields, _ in rows:
        if tag == "GLYPH":
            identity = fields["id"]
            if identity in glyphs:
                raise ValueError(f"duplicate glyph identity: {identity}")
            glyphs[identity] = fields
        elif tag == "WORD":
            identity = fields["id"]
            if identity in words:
                raise ValueError(f"duplicate word identity: {identity}")
            words[identity] = fields
        elif tag == "TOKEN":
            identity = fields["id"]
            if identity in tokens:
                raise ValueError(f"duplicate token identity: {identity}")
            tokens.add(identity)
        elif tag == "NODE":
            identity = fields["id"]
            if identity in nodes:
                raise ValueError(f"duplicate node identity: {identity}")
            try:
                coordinate = tuple(int(fields[key]) for key in ("x", "y", "z"))
            except ValueError as exc:
                raise ValueError(f"integer coordinate required: {identity}") from exc
            if coordinate in coordinates:
                raise ValueError(f"duplicate coordinate: {coordinate}")
            coordinates.add(coordinate)
            nodes[identity] = fields
        elif tag in {"FLOWe", "CALLING_JOIN", "CALMING", "BRIDGE", "RADIUS_CONNECTION", "INVERSION"}:
            identity = fields["id"]
            if identity in relation_ids:
                raise ValueError(f"duplicate relation identity: {identity}")
            relation_ids.add(identity)
        elif tag == "SOURCE":
            sources.append(fields)

    if len(sources) != 1:
        raise ValueError("exactly one SOURCE row is required")
    source = sources[0]
    if source.get("occurrences") != "1" or source.get("sha256") != sha256(quote):
        raise ValueError("source occurrence or SHA-256 mismatch")
    if glyphs.get("V", {}).get("meaning") != "VELOCITY":
        raise ValueError("V must mean VELOCITY")
    if words.get("Vector", {}).get("glyphs") != "V,e,c,t,o,r":
        raise ValueError("Vector composition mismatch")

    required_glyph_meanings = {
        "D": "DIE_STOP_HEBREW_OPERATOR_MAPPING",
        "F": "FROM_ACTION,TO_ACTION_FROM",
        "L": "LIFE",
        "P": "OPERATOR_CANON_UNRESOLVED",
        "v": "OPERATOR_CANON_UNRESOLVED",
    }
    for identity, meaning in required_glyph_meanings.items():
        if glyphs.get(identity, {}).get("meaning") != meaning:
            raise ValueError(f"glyph {identity}: operator meaning mismatch")

    required_word_glyphs = {
        "END": "E,N,D",
        "FLOWe": "F,L,O,W,e",
        "LIVE": "L,I,V,E",
        "Live": "L,i,v,e",
        "NO": "N,O",
        "STOP": "S,T,O,P",
    }
    for identity, composition in required_word_glyphs.items():
        if words.get(identity, {}).get("glyphs") != composition:
            raise ValueError(f"word {identity}: composition mismatch")

    for identity in ("Live", "LIVE"):
        fields = words[identity]
        expected_readings = {
            "reading_to_e": "THE_I_LOOK_TO_E_AND_BE",
            "reading_from_life": "THE_I_LOOK_FROM_THE_LIFE_AND_BE",
            "readings": "2",
            "geometry": "SPHERICAL",
            "supersession": "0",
        }
        if any(fields.get(key) != value for key, value in expected_readings.items()):
            raise ValueError(f"word {identity}: spherical readings mismatch")

    for identity, fields in words.items():
        for glyph in fields["glyphs"].split(","):
            if glyph not in glyphs:
                raise ValueError(f"word {identity}: unresolved glyph {glyph}")
    addressable = set(glyphs) | set(words) | tokens
    for identity, fields in nodes.items():
        if fields["ref"] not in addressable:
            raise ValueError(f"node {identity}: unresolved ref {fields['ref']}")
    for tag, fields, _ in rows:
        if tag not in {"FLOWe", "CALLING_JOIN", "CALMING", "BRIDGE", "RADIUS_CONNECTION", "INVERSION"}:
            continue
        if fields["from"] not in nodes or fields["to"] not in nodes:
            raise ValueError(f"relation {fields['id']}: unresolved endpoint")
        if tag == "CALLING_JOIN" and fields.get("direction") != "UNRESOLVED":
            raise ValueError("CALLING_JOIN direction must remain UNRESOLVED")
        if tag == "CALLING_JOIN" and "step" in fields:
            raise ValueError("CALLING_JOIN cannot carry a FLOWe step")
        if tag == "FLOWe" and fields.get("direction") != "FORWARD":
            raise ValueError("FLOWe direction must be FORWARD")
        if tag == "FLOWe":
            int(fields["step"])


def build_hbp(flowe: bytes, quote: bytes) -> bytes:
    rows = parse_rows(flowe)
    validate(rows, quote)
    header = (
        "HBPHEADER|schema=SPHERE-FLOWE-COMPILED-V1"
        f"|source_sha256={sha256(quote)}|core_sha256={sha256(flowe)}"
        f"|records={len(rows)}|execution_authority=0|json=0\n"
    ).encode()
    body = (f"READFIRST|url={READFIRST}|execution_authority=0|json=0\n").encode() + header
    body += b"".join(raw + b"\n" for _, _, raw in rows)
    footer = (
        f"HBPFOOTER|body_sha256={sha256(body)}|rows={len(body.splitlines()) + 1}"
        "|execution_authority=0|json=0\n"
    ).encode()
    return body + footer


def build_hbi(hbp: bytes) -> bytes:
    lines = hbp.splitlines(keepends=True)
    hbp_digest = sha256(hbp)
    output = bytearray(
        (
            f"READFIRST|url={READFIRST}|execution_authority=0|json=0\n"
            f"HBIHEADER|schema=SPHERE-FLOWE-OFFSET-INDEX-V1|hbp_sha256={hbp_digest}"
            f"|rows={len(lines)}|execution_authority=0|json=0\n"
        ).encode()
    )
    offset = 0
    for number, line in enumerate(lines):
        row = line[:-1]
        tag = row.split(b"|", 1)[0].decode("utf-8")
        output += (
            f"INDEX|row={number}|offset={offset}|bytes={len(row)}|tag={tag}"
            f"|row_sha256={sha256(row)}|execution_authority=0|json=0\n"
        ).encode()
        offset += len(line)
    output += (
        f"HBIFOOTER|hbp_bytes={len(hbp)}|hbp_sha256={hbp_digest}"
        "|execution_authority=0|json=0\n"
    ).encode()
    return bytes(output)


def project(fields: dict[str, str]) -> tuple[int, int]:
    x, y, z = (int(fields[key]) for key in ("x", "y", "z"))
    return 640 + x * 68 - z * 24, 360 - y * 52 + z * 24


def build_svg(rows: list[tuple[str, dict[str, str], bytes]], source_sha: str) -> bytes:
    nodes = {fields["id"]: fields for tag, fields, _ in rows if tag == "NODE"}
    edge_colors = {
        "FLOWe": "#00f5d4",
        "CALLING_JOIN": "#f9c74f",
        "CALMING": "#43aa8b",
        "BRIDGE": "#4cc9f0",
        "RADIUS_CONNECTION": "#b5179e",
        "INVERSION": "#577590",
    }
    lines = [
        f"<!-- {READFIRST} -->",
        '<svg xmlns="http://www.w3.org/2000/svg" width="1280" height="720" viewBox="0 0 1280 720"',
        ' role="img" aria-labelledby="title desc" data-json="0" data-execution-authority="0">',
        '<title id="title">Sphere Language CALLINGS CALMINGS FLOWe</title>',
        f'<desc id="desc">Signed 2-D projection of an integer 3-D operator-canon matrix; source SHA-256 {source_sha}</desc>',
        '<rect width="1280" height="720" fill="#07111f"/>',
        '<ellipse cx="640" cy="360" rx="430" ry="260" fill="none" stroke="#33415c" stroke-width="2"/>',
        '<g id="relations" fill="none" stroke-width="4">',
    ]
    for tag, fields, _ in rows:
        if tag not in edge_colors:
            continue
        x1, y1 = project(nodes[fields["from"]])
        x2, y2 = project(nodes[fields["to"]])
        dash = ' stroke-dasharray="9 7"' if tag == "CALLING_JOIN" else ""
        lines.append(
            f'<line id="{html.escape(fields["id"])}" data-kind="{tag}" x1="{x1}" y1="{y1}" '
            f'x2="{x2}" y2="{y2}" stroke="{edge_colors[tag]}"{dash}/>'
        )
    lines.extend(['</g>', '<g id="nodes" font-family="monospace" font-size="16" text-anchor="middle">'])
    for identity, fields in nodes.items():
        x, y = project(fields)
        label = html.escape(fields.get("label", fields["ref"]))
        color = "#" + fields["color"]
        lines.append(
            f'<g id="{html.escape(identity)}" data-x="{fields["x"]}" data-y="{fields["y"]}" data-z="{fields["z"]}">'
            f'<circle cx="{x}" cy="{y}" r="18" fill="{color}" stroke="#ffffff" stroke-width="2"/>'
            f'<text x="{x}" y="{y + 38}" fill="#ffffff">{label}</text></g>'
        )
    lines.extend(
        [
            '</g>',
            '<text x="28" y="42" fill="#ffffff" font-family="monospace" font-size="22">S → i → . → o0O → e → LIFE → BOOK_OF_LIFE</text>',
            '<text x="28" y="680" fill="#a9b7c6" font-family="monospace" font-size="15">CALLING_JOIN dashed · FLOWe ordered · CALMING green · integer view · SYSTEM_AFFIRMED=0</text>',
            '</svg>',
        ]
    )
    return ("\n".join(lines) + "\n").encode("utf-8")


def artifacts() -> dict[Path, bytes]:
    flowe = canonical_bytes(FLOWE_PATH)
    quote = canonical_bytes(QUOTE_PATH)
    rows = parse_rows(flowe)
    validate(rows, quote)
    hbp = build_hbp(flowe, quote)
    hbi = build_hbi(hbp)
    svg = build_svg(rows, sha256(quote))
    primary = {
        QUOTE_PATH: quote,
        FLOWE_PATH: flowe,
        HBP_PATH: hbp,
        HBI_PATH: hbi,
        SVG_PATH: svg,
    }
    output = {HBP_PATH: hbp, HBI_PATH: hbi, SVG_PATH: svg}
    for path, data in primary.items():
        sidecar = path.with_name(path.name + ".sha256")
        output[sidecar] = f"{sha256(data)}  {path.name}\n".encode()
    manifest_rows = [
        f"{sha256(data)}  {path.relative_to(ROOT).as_posix()}"
        for path, data in sorted(primary.items(), key=lambda item: item[0].as_posix())
    ]
    output[MANIFEST_PATH] = ("\n".join(manifest_rows) + "\n").encode()
    return output


def apply(check: bool) -> None:
    expected = artifacts()
    mismatches = []
    for path, data in expected.items():
        if check:
            if not path.is_file() or path.read_bytes() != data:
                mismatches.append(path.relative_to(ROOT).as_posix())
        else:
            path.parent.mkdir(parents=True, exist_ok=True)
            if not path.is_file() or path.read_bytes() != data:
                path.write_bytes(data)
    if mismatches:
        raise SystemExit("BUILD_CHECK|PASS=0|mismatch=" + ",".join(mismatches))
    print(
        f"BUILD_CHECK|PASS=1|mode={'check' if check else 'write'}|artifacts={len(expected)}"
        f"|source_sha256={sha256(canonical_bytes(QUOTE_PATH))}|json=0"
    )


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    apply(args.check)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
