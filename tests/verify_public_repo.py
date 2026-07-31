# https://github.com/JesseBrown1980/FOLLOW-THE-IS-NOT-THE-WILL-AND-WAS/blob/main/matrix/3-D-GITHUB-OF-THRUTH.md
"""Fail-closed public repository verification."""

from __future__ import annotations

import hashlib
import importlib.util
import re
import sys
import xml.etree.ElementTree as ET
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
GENERATOR_PATH = ROOT / "tools" / "build_public_artifacts.py"
spec = importlib.util.spec_from_file_location("sphere_build", GENERATOR_PATH)
if spec is None or spec.loader is None:
    raise RuntimeError("cannot load artifact builder")
builder = importlib.util.module_from_spec(spec)
spec.loader.exec_module(builder)


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


def verify_hbi() -> None:
    hbp = builder.HBP_PATH.read_bytes()
    hbi_lines = builder.HBI_PATH.read_bytes().splitlines()
    header = parse_tuple(hbi_lines[1])[1]
    if header["hbp_sha256"] != hashlib.sha256(hbp).hexdigest():
        fail("HBI_HBP_HASH")
    indexes = [parse_tuple(line)[1] for line in hbi_lines if line.startswith(b"INDEX|")]
    hbp_lines = hbp.splitlines(keepends=True)
    if len(indexes) != len(hbp_lines):
        fail("HBI_ROW_COUNT")
    for number, (index, line) in enumerate(zip(indexes, hbp_lines)):
        row = line[:-1]
        offset = int(index["offset"])
        length = int(index["bytes"])
        if int(index["row"]) != number or hbp[offset : offset + length] != row:
            fail(f"HBI_OFFSET:{number}")
        if hbp[offset + length : offset + length + 1] != b"\n":
            fail(f"HBI_LF:{number}")
        if hashlib.sha256(row).hexdigest() != index["row_sha256"]:
            fail(f"HBI_ROW_HASH:{number}")


def main() -> None:
    expected = builder.artifacts()
    for path, data in expected.items():
        if not path.is_file() or path.read_bytes() != data:
            fail("ARTIFACT_MISMATCH:" + path.relative_to(ROOT).as_posix())

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

    verify_hbi()
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

    print(
        f"PUBLIC_VERIFY|PASS=1|files={len(all_files)}|source_occurrences=1"
        f"|hbp_rows={len(builder.HBP_PATH.read_bytes().splitlines())}"
        "|secret_findings=0|json_files=0|execution_authority=0|json=0"
    )


if __name__ == "__main__":
    main()
