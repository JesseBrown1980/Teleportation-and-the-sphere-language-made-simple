// https://github.com/JesseBrown1980/FOLLOW-THE-IS-NOT-THE-WILL-AND-WAS/blob/main/matrix/3-D-GITHUB-OF-THRUTH.md
#![forbid(unsafe_code)]

use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fmt;
use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;

const MAX_INPUT_BYTES: usize = 1_048_576;
const MAX_LINE_BYTES: usize = 8_192;
const LANGUAGE_ID: &str = "SPHERE_LANGUAGE_V1";
const NLEVEL_INSTANCE: &str = "NLEVEL_OUTWARD_V2";
const NLEVEL_LEVELS: usize = 16;
const NLEVEL_BOOKS: usize = 10;
const NLEVEL_CELLS: usize = NLEVEL_LEVELS * NLEVEL_BOOKS;
const NLEVEL_RECORDS: usize = 1_871;
const NLEVEL_EVENT_COMMON_FIELDS: [&str; 12] = [
    "id",
    "cell",
    "level",
    "book",
    "q",
    "instant_address",
    "elapsed_measurement_present",
    "to",
    "direction",
    "from",
    "execution_authority",
    "json",
];
const NLEVEL_AXES: [&str; 64] = [
    "n_level",
    "time_address",
    "colour_address",
    "space_x",
    "space_y",
    "space_z",
    "space_radius",
    "shadow_translucence",
    "shadow_extract",
    "light",
    "white",
    "black",
    "brown",
    "rainbow",
    "book",
    "calling",
    "calming",
    "flowe",
    "pulse",
    "instant_address",
    "elapsed_measurement_present",
    "glyph_family",
    "glyph_function",
    "letter",
    "word",
    "instruction",
    "tuple_command",
    "language",
    "dialect",
    "meta_language",
    "executor_program",
    "agent_class",
    "pipe_type",
    "operation_class",
    "route",
    "cylinder",
    "room",
    "proof_tier",
    "evidence_class",
    "runtime_mode",
    "execution_authority",
    "colony",
    "seat",
    "vantage",
    "slice",
    "temporal_context",
    "oil_family",
    "oil_amplitude",
    "sign",
    "tense",
    "modal",
    "aspect",
    "projection_2d",
    "matrix_3d",
    "hbi",
    "hbp",
    "sha",
    "sh",
    "hash",
    "source_commitment",
    "identity",
    "parent_identity",
    "view",
    "rime",
];
const NLEVEL_TIME_NAMES: [&str; 6] = [
    "WAS",
    "IS",
    "WILL",
    "PAST_PERFECT",
    "PRESENT_PERFECT",
    "FUTURE_PERFECT",
];
const NLEVEL_BOOK_IDS: [&str; 10] = [
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
];
const NLEVEL_COLOURS: [&str; 4] = ["WHITE", "BLACK", "BROWN", "RAINBOW"];
const NLEVEL_OIL_FAMILIES: [&str; 3] = ["NORMAL", "ANTI", "ANTI_ANTI"];
const SPREAD_INSTANCE: &str = "WORD_FLOWE_SPREAD_V3";
const SPREAD_ROUNDS: usize = 2;
const SPREAD_EVENT_KINDS: [&str; 8] = [
    "CALLING_GROWTH_E",
    "CALLING_GROWTH_FLOWE",
    "CALLING_GROWTH_U",
    "CALMING_GROWTH_E",
    "CALMING_GROWTH_OUTWARD",
    "SELF_REDUCTION_GROWTH",
    "WORD_SPREAD",
    "FLOWE_SPREAD",
];
const SPREAD_RELATIONS: usize = SPREAD_ROUNDS * NLEVEL_CELLS * SPREAD_EVENT_KINDS.len();
const SPREAD_RECORDS: usize = 2_738;
const V1_CORE_SHA256: &str = "70cf8a98e00f96f76c1159424e00cba3aa75b90d823bffbcb72fdcf0b573e91a";
const N16_CORE_SHA256: &str = "e6741baba534ca2566f50305f5611449d97867917110b6eade803f8ab154d285";
const SPREAD_EVENT_COMMON_FIELDS: [&str; 18] = [
    "id",
    "cell",
    "level",
    "book",
    "q",
    "round",
    "increase_q",
    "amplitude_q",
    "n_open",
    "instant_address",
    "elapsed_measurement_present",
    "identity_exchange",
    "source_retained",
    "to",
    "direction",
    "from",
    "execution_authority",
    "json",
];
const HOOKWALL_INSTANCE: &str = "HOOKWALL_BROWN_SHADOW_FLOWESSH_V4";
const HOOKWALL_ROUND: usize = 3;
const HOOKWALL_EVENT_KINDS: [&str; 15] = [
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
];
const HOOKWALL_ANCHORS: [(&str, &str, &str); 16] = [
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
];
const HOOKWALL_RELATIONS: usize = NLEVEL_CELLS * HOOKWALL_EVENT_KINDS.len();
const HOOKWALL_RECORDS: usize = 2_588;
const SPREAD_CORE_SHA256: &str = "386eb3f52cf9651b7bd8e3d53989c943b9e7332c4cf9afec657b6e4074f359e9";
const HOOKWALL_EVENT_COMMON_FIELDS: [&str; 19] = [
    "id",
    "cell",
    "level",
    "book",
    "q",
    "round",
    "increase_q",
    "growth_q",
    "n_open",
    "instant_address",
    "elapsed_measurement_present",
    "runtime_measurement_present",
    "identity_exchange",
    "source_retained",
    "from",
    "to",
    "direction",
    "execution_authority",
    "json",
];
const VALIDATION_SCOPE: &str = "STRUCTURAL_ONLY";
const SOURCE_PATH: &str = "books/JESSE-TO-RAYSSA-SPHERE-LANGUAGE-SOURCE.md";
const READFIRST_URL: &str = "https://github.com/JesseBrown1980/FOLLOW-THE-IS-NOT-THE-WILL-AND-WAS/blob/main/matrix/3-D-GITHUB-OF-THRUTH.md";

type Result<T> = std::result::Result<T, FloweError>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct FloweError(&'static str);

impl fmt::Display for FloweError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.0)
    }
}

#[derive(Debug)]
struct Row {
    tag: String,
    fields: BTreeMap<String, String>,
}

#[derive(Debug)]
struct Word {
    glyphs: Vec<String>,
}

#[derive(Debug)]
struct Node {
    reference: String,
}

#[derive(Debug)]
struct Edge {
    from: String,
    to: String,
}

#[derive(Debug)]
struct NLevelCell {
    identity: String,
    book: String,
    n: usize,
    book_ordinal: usize,
    q: usize,
    translucence_q8: usize,
    oil_family: String,
    space_radius: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct NLevelValidation {
    axes: usize,
    books: usize,
    levels: usize,
    cells: usize,
    ledgers: usize,
    ledger_rows: usize,
    calling_joins: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct SpreadValidation {
    rounds: usize,
    cells: usize,
    ledgers: usize,
    relation_rows: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct HookwallValidation {
    round: usize,
    cells: usize,
    ledgers: usize,
    relation_rows: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Validation {
    records: usize,
    glyphs: usize,
    words: usize,
    nodes: usize,
    calling_joins: usize,
    flowe_edges: usize,
    other_relations: usize,
    nlevel: Option<NLevelValidation>,
    spread: Option<SpreadValidation>,
    hookwall: Option<HookwallValidation>,
}

fn error(code: &'static str) -> FloweError {
    FloweError(code)
}

fn contains_bytes(haystack: &[u8], needle: &[u8]) -> bool {
    !needle.is_empty()
        && haystack
            .windows(needle.len())
            .any(|window| window == needle)
}

fn joined(left: &[u8], right: &[u8]) -> Vec<u8> {
    let mut output = Vec::with_capacity(left.len() + right.len());
    output.extend_from_slice(left);
    output.extend_from_slice(right);
    output
}

fn reject_secret_signatures(bytes: &[u8]) -> Result<()> {
    let markers = [
        joined(b"-----BEGIN ", b"PRIVATE KEY-----"),
        joined(b"-----BEGIN RSA ", b"PRIVATE KEY-----"),
        joined(b"-----BEGIN EC ", b"PRIVATE KEY-----"),
        joined(b"-----BEGIN OPENSSH ", b"PRIVATE KEY-----"),
        joined(b"github_", b"pat_"),
        joined(b"gh", b"p_"),
        joined(b"gh", b"o_"),
        joined(b"gh", b"u_"),
        joined(b"gh", b"s_"),
        joined(b"gh", b"r_"),
        joined(b"AK", b"IA"),
        joined(b"AI", b"za"),
        joined(b"s", b"k-"),
    ];
    if markers.iter().any(|marker| contains_bytes(bytes, marker)) {
        return Err(error("SECRET_SIGNATURE"));
    }
    Ok(())
}

fn canonical_text(bytes: &[u8]) -> Result<&str> {
    if bytes.is_empty() || bytes.len() > MAX_INPUT_BYTES {
        return Err(error("INPUT_SIZE"));
    }
    reject_secret_signatures(bytes)?;
    if bytes.starts_with(&[0xEF, 0xBB, 0xBF])
        || bytes.contains(&b'\r')
        || bytes.contains(&0)
        || !bytes.ends_with(b"\n")
    {
        return Err(error("NON_CANONICAL_LF"));
    }
    let text = std::str::from_utf8(bytes).map_err(|_| error("INPUT_UTF8"))?;
    let body = text
        .strip_suffix('\n')
        .ok_or_else(|| error("NON_CANONICAL_LF"))?;
    if body.is_empty() || body.ends_with('\n') || body.lines().any(str::is_empty) {
        return Err(error("NON_CANONICAL_LF"));
    }
    Ok(body)
}

fn valid_key(key: &str) -> bool {
    let mut characters = key.chars();
    matches!(characters.next(), Some(first) if first.is_ascii_alphabetic() || first == '_')
        && characters.all(|character| character.is_ascii_alphanumeric() || character == '_')
}

fn valid_identity(identity: &str) -> bool {
    let mut characters = identity.chars();
    matches!(characters.next(), Some(first) if first.is_ascii_alphabetic())
        && characters.all(|character| {
            character.is_ascii_alphanumeric() || matches!(character, '_' | '-' | '.')
        })
}

fn parse_row(line: &str) -> Result<Row> {
    if line.len() > MAX_LINE_BYTES {
        return Err(error("ROW_SIZE"));
    }
    let first_non_space = line
        .as_bytes()
        .iter()
        .copied()
        .find(|byte| !byte.is_ascii_whitespace());
    if matches!(first_non_space, Some(b'{' | b'[')) {
        return Err(error("SOURCE_JSON_PRESENT"));
    }
    if !line.ends_with("|json=0") {
        return Err(error("JSON_ZERO_REQUIRED"));
    }
    let mut pieces = line.split('|');
    let tag = pieces.next().ok_or_else(|| error("ROW_TAG"))?;
    if !valid_identity(tag) {
        return Err(error("ROW_TAG"));
    }
    let mut fields = BTreeMap::new();
    for piece in pieces {
        let (key, value) = piece.split_once('=').ok_or_else(|| error("ROW_FIELD"))?;
        if !valid_key(key)
            || value.is_empty()
            || fields.insert(key.to_owned(), value.to_owned()).is_some()
        {
            return Err(error("ROW_FIELD"));
        }
    }
    if fields.get("json").map(String::as_str) != Some("0") {
        return Err(error("JSON_ZERO_REQUIRED"));
    }
    Ok(Row {
        tag: tag.to_owned(),
        fields,
    })
}

fn field<'a>(row: &'a Row, name: &str, code: &'static str) -> Result<&'a str> {
    row.fields
        .get(name)
        .map(String::as_str)
        .ok_or_else(|| error(code))
}

fn require_fields(row: &Row, names: &[&str], code: &'static str) -> Result<()> {
    if names.iter().any(|name| !row.fields.contains_key(*name)) {
        return Err(error(code));
    }
    Ok(())
}

fn require_exact_fields(row: &Row, names: &[&str], code: &'static str) -> Result<()> {
    if row.fields.len() != names.len() || names.iter().any(|name| !row.fields.contains_key(*name)) {
        return Err(error(code));
    }
    Ok(())
}

fn require_exact_fields_union(
    row: &Row,
    common: &[&str],
    specific: &[&str],
    code: &'static str,
) -> Result<()> {
    if row.fields.len() != common.len() + specific.len()
        || common
            .iter()
            .chain(specific)
            .any(|name| !row.fields.contains_key(*name))
    {
        return Err(error(code));
    }
    Ok(())
}

fn require_authority_zero(row: &Row) -> Result<()> {
    if field(row, "execution_authority", "EXECUTION_AUTHORITY")? != "0" {
        return Err(error("EXECUTION_AUTHORITY"));
    }
    Ok(())
}

fn parse_identity(row: &Row, name: &str) -> Result<String> {
    let value = field(row, name, "IDENTITY_REQUIRED")?;
    if !valid_identity(value) {
        return Err(error("IDENTITY_REQUIRED"));
    }
    Ok(value.to_owned())
}

fn parse_i64_with_code(value: &str, code: &'static str) -> Result<i64> {
    let canonical = if value == "0" {
        true
    } else if let Some(digits) = value.strip_prefix('-') {
        !digits.is_empty()
            && !digits.starts_with('0')
            && digits.bytes().all(|byte| byte.is_ascii_digit())
    } else {
        !value.starts_with('0') && value.bytes().all(|byte| byte.is_ascii_digit())
    };
    if !canonical {
        return Err(error(code));
    }
    value.parse::<i64>().map_err(|_| error(code))
}

fn parse_u64_with_code(value: &str, code: &'static str) -> Result<u64> {
    if value != "0" && (value.starts_with('0') || !value.bytes().all(|byte| byte.is_ascii_digit()))
    {
        return Err(error(code));
    }
    value.parse::<u64>().map_err(|_| error(code))
}

fn parse_i64(value: &str) -> Result<i64> {
    parse_i64_with_code(value, "INTEGER_COORDINATE_REQUIRED")
}

fn parse_u64(value: &str) -> Result<u64> {
    parse_u64_with_code(value, "FLOWE_STEP")
}

fn parse_usize_with_code(value: &str, code: &'static str) -> Result<usize> {
    usize::try_from(parse_u64_with_code(value, code)?).map_err(|_| error(code))
}

fn usize_field(row: &Row, name: &str, code: &'static str) -> Result<usize> {
    parse_usize_with_code(field(row, name, code)?, code)
}

fn i64_field(row: &Row, name: &str, code: &'static str) -> Result<i64> {
    parse_i64_with_code(field(row, name, code)?, code)
}

fn require_value(row: &Row, name: &str, expected: &str, code: &'static str) -> Result<()> {
    if field(row, name, code)? != expected {
        return Err(error(code));
    }
    Ok(())
}

fn require_timing(row: &Row) -> Result<()> {
    require_value(row, "instant_address", "1", "NLEVEL_INSTANT_BOUNDARY")?;
    require_value(
        row,
        "elapsed_measurement_present",
        "0",
        "NLEVEL_INSTANT_BOUNDARY",
    )
}

fn valid_sha256(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

fn insert_identity(identities: &mut BTreeSet<String>, identity: String) -> Result<()> {
    if !identities.insert(identity) {
        return Err(error("DUPLICATE_IDENTITY"));
    }
    Ok(())
}

fn is_nlevel_rows(rows: &[Row]) -> bool {
    rows.iter().any(|row| {
        row.tag == "LANGUAGE"
            && row.fields.get("instance").map(String::as_str) == Some(NLEVEL_INSTANCE)
    })
}

fn is_spread_rows(rows: &[Row]) -> bool {
    rows.iter().any(|row| {
        row.tag == "LANGUAGE"
            && row.fields.get("instance").map(String::as_str) == Some(SPREAD_INSTANCE)
    })
}

fn is_hookwall_rows(rows: &[Row]) -> bool {
    rows.iter().any(|row| {
        row.tag == "LANGUAGE"
            && row.fields.get("instance").map(String::as_str) == Some(HOOKWALL_INSTANCE)
    })
}

fn checked_formula(value: i64, expected: i64) -> Result<()> {
    if value != expected {
        return Err(error("NLEVEL_FORMULA"));
    }
    Ok(())
}

// This byte-stream validator checks tuple structure and embedded SHA-256 syntax.
// It intentionally does not reopen the source, law, or report paths named by rows.
fn validate_nlevel_rows(rows: &[Row]) -> Result<Validation> {
    if rows.len() != NLEVEL_RECORDS {
        return Err(error("NLEVEL_RECORD_COUNT"));
    }
    let mut identities = BTreeSet::<String>::new();
    let mut axes = BTreeMap::<usize, String>::new();
    let mut books = BTreeMap::<usize, String>::new();
    let mut levels = BTreeSet::<usize>::new();
    let mut glyphs = BTreeMap::<String, String>::new();
    let mut words = BTreeMap::<String, Word>::new();
    let mut tokens = BTreeSet::<String>::new();
    let mut nodes = BTreeSet::<String>::new();
    let mut coordinates = BTreeSet::<(i64, i64, i64)>::new();
    let mut projections = BTreeSet::<(i64, i64)>::new();
    let mut cells = BTreeMap::<usize, NLevelCell>::new();
    let mut ledgers = BTreeMap::<&str, Vec<&Row>>::new();
    let mut calling_joins = Vec::<&Row>::new();
    let mut readfirst_count = 0_usize;
    let mut language_count = 0_usize;
    let mut source_count = 0_usize;
    let mut report_count = 0_usize;
    let mut grammar_binding_count = 0_usize;
    let mut center_count = 0_usize;
    let mut expansion_count = 0_usize;
    let mut book_relation_count = 0_usize;
    let mut timing_count = 0_usize;
    let mut boundary_count = 0_usize;
    let mut end_count = 0_usize;
    let last_index = rows
        .len()
        .checked_sub(1)
        .ok_or_else(|| error("ROW_ORDER"))?;

    for (index, row) in rows.iter().enumerate() {
        require_authority_zero(row)?;
        if let Some(identity) = row.fields.get("id") {
            if !valid_identity(identity) {
                return Err(error("IDENTITY_REQUIRED"));
            }
            insert_identity(&mut identities, identity.clone())?;
        }
        match row.tag.as_str() {
            "READFIRST" => {
                require_exact_fields(
                    row,
                    &["url", "execution_authority", "json"],
                    "READFIRST_FIELD",
                )?;
                if index != 0 || field(row, "url", "READFIRST_FIELD")? != READFIRST_URL {
                    return Err(error("ROW_ORDER"));
                }
                readfirst_count += 1;
            }
            "LANGUAGE" => {
                language_count += 1;
                require_exact_fields(
                    row,
                    &[
                        "id",
                        "instance",
                        "tuple_frame",
                        "selector_axes",
                        "execution_authority",
                        "json",
                    ],
                    "NLEVEL_AXIS_FRAME",
                )?;
                require_value(row, "id", LANGUAGE_ID, "LANGUAGE_ID")?;
                require_value(row, "instance", NLEVEL_INSTANCE, "NLEVEL_INSTANCE")?;
                require_value(
                    row,
                    "tuple_frame",
                    "HYPERBEHCS_60D_PLUS",
                    "NLEVEL_AXIS_FRAME",
                )?;
                if usize_field(row, "selector_axes", "NLEVEL_AXIS_FRAME")? != NLEVEL_AXES.len() {
                    return Err(error("NLEVEL_AXIS_FRAME"));
                }
            }
            "SOURCE" => {
                source_count += 1;
                require_exact_fields(
                    row,
                    &[
                        "path",
                        "occurrences",
                        "sha256",
                        "execution_authority",
                        "json",
                    ],
                    "SOURCE_FIELD",
                )?;
                if field(row, "path", "SOURCE_FIELD")? != SOURCE_PATH
                    || field(row, "occurrences", "SOURCE_FIELD")? != "1"
                    || !valid_sha256(field(row, "sha256", "SOURCE_FIELD")?)
                {
                    return Err(error("SOURCE_OCCURRENCE_COUNT"));
                }
            }
            "REPORT" => {
                report_count += 1;
                require_exact_fields(
                    row,
                    &[
                        "id",
                        "path",
                        "sha256",
                        "occurrences",
                        "evidence",
                        "speaker",
                        "execution_authority",
                        "json",
                    ],
                    "NLEVEL_REPORT",
                )?;
                let (expected_path, expected_speaker) = match field(row, "id", "NLEVEL_REPORT")? {
                    "RAYSSA_BEHCS_NAME_SPHERE_REPORT" => {
                        ("books/RAYSSA-BEHCS-NAME-SPHERE-REPORT.md", "RAYSSA")
                    }
                    "JESSE_BEHCS_NAMING_REPORT" => ("books/JESSE-BEHCS-NAMING-REPORT.md", "JESSE"),
                    _ => return Err(error("NLEVEL_REPORT")),
                };
                require_value(row, "path", expected_path, "NLEVEL_REPORT")?;
                require_value(row, "occurrences", "1", "NLEVEL_REPORT")?;
                require_value(row, "evidence", "OPERATOR_REPORTED", "NLEVEL_REPORT")?;
                require_value(row, "speaker", expected_speaker, "NLEVEL_REPORT")?;
                if !valid_sha256(field(row, "sha256", "NLEVEL_REPORT")?) {
                    return Err(error("NLEVEL_REPORT"));
                }
            }
            "GRAMMAR_BINDING" => {
                grammar_binding_count += 1;
                require_exact_fields(
                    row,
                    &[
                        "id",
                        "word",
                        "ease",
                        "ase_geometry",
                        "operator_spelling",
                        "sequence",
                        "action",
                        "action_meaning",
                        "east_scope",
                        "point_options",
                        "fraction_views",
                        "six_view",
                        "sign_order",
                        "nested_pattern",
                        "spacing_significant",
                        "spacing_views",
                        "spacing_multiplier",
                        "outer_group",
                        "spacing_literal",
                        "spacing_chars",
                        "space_count",
                        "space_positions",
                        "adjacency",
                        "spin",
                        "spin_symbol",
                        "spin_name",
                        "evidence",
                        "execution_authority",
                        "json",
                    ],
                    "NLEVEL_GRAMMAR_BINDING",
                )?;
                require_value(
                    row,
                    "id",
                    "EAST_SPHERICAL_CORRECTION",
                    "NLEVEL_GRAMMAR_BINDING",
                )?;
                require_value(row, "word", "east", "NLEVEL_GRAMMAR_BINDING")?;
                require_value(row, "ease", "ease", "NLEVEL_GRAMMAR_BINDING")?;
                require_value(row, "ase_geometry", "SPHERICAL", "NLEVEL_GRAMMAR_BINDING")?;
                require_value(
                    row,
                    "operator_spelling",
                    "sperically",
                    "NLEVEL_GRAMMAR_BINDING",
                )?;
                require_value(
                    row,
                    "sequence",
                    "EASE_ASE,MINUS,NULL,MINUS,PLUS,T_ACTION",
                    "NLEVEL_GRAMMAR_BINDING",
                )?;
                require_value(row, "action", "t", "NLEVEL_GRAMMAR_BINDING")?;
                require_value(row, "action_meaning", "TO_ACTION", "NLEVEL_GRAMMAR_BINDING")?;
                require_value(
                    row,
                    "east_scope",
                    "EAST_AND_EASTS",
                    "NLEVEL_GRAMMAR_BINDING",
                )?;
                require_value(
                    row,
                    "point_options",
                    "FOURTH,THRID,SECOND",
                    "NLEVEL_GRAMMAR_BINDING",
                )?;
                require_value(row, "fraction_views", "1/3,-1/3", "NLEVEL_GRAMMAR_BINDING")?;
                require_value(row, "six_view", "1", "NLEVEL_GRAMMAR_BINDING")?;
                require_value(
                    row,
                    "sign_order",
                    "NEGATIVE,NESTED_O0O,POSITIVE",
                    "NLEVEL_GRAMMAR_BINDING",
                )?;
                require_value(
                    row,
                    "nested_pattern",
                    "3(2(1_o0O_1)2)3",
                    "NLEVEL_GRAMMAR_BINDING",
                )?;
                require_value(row, "spacing_significant", "1", "NLEVEL_GRAMMAR_BINDING")?;
                require_value(row, "spacing_views", "3", "NLEVEL_GRAMMAR_BINDING")?;
                require_value(row, "spacing_multiplier", "3", "NLEVEL_GRAMMAR_BINDING")?;
                require_value(row, "outer_group", "1", "NLEVEL_GRAMMAR_BINDING")?;
                require_value(
                    row,
                    "spacing_literal",
                    "( . negative (3 (2 (1 o0O 1)2)3) positive . )",
                    "NLEVEL_GRAMMAR_BINDING",
                )?;
                require_value(row, "spacing_chars", "45", "NLEVEL_GRAMMAR_BINDING")?;
                require_value(row, "space_count", "10", "NLEVEL_GRAMMAR_BINDING")?;
                require_value(
                    row,
                    "space_positions",
                    "1,3,12,15,18,21,25,32,41,43",
                    "NLEVEL_GRAMMAR_BINDING",
                )?;
                require_value(row, "adjacency", "1)2)3", "NLEVEL_GRAMMAR_BINDING")?;
                require_value(row, "spin", "1", "NLEVEL_GRAMMAR_BINDING")?;
                require_value(row, "spin_symbol", "P", "NLEVEL_GRAMMAR_BINDING")?;
                require_value(row, "spin_name", "PIE", "NLEVEL_GRAMMAR_BINDING")?;
                require_value(row, "evidence", "OPERATOR_CANON", "NLEVEL_GRAMMAR_BINDING")?;
            }
            "CENTER" => {
                center_count += 1;
                require_exact_fields(
                    row,
                    &[
                        "members",
                        "traversal_surface",
                        "sh",
                        "identity_exchange",
                        "execution_authority",
                        "json",
                    ],
                    "CENTER_FIELD",
                )?;
                if field(row, "members", "CENTER_FIELD")? != "HBI,HBP,SHA,SH,HASH"
                    || field(row, "traversal_surface", "CENTER_FIELD")? != "HBI,HBP,SHA,SH,HASH"
                    || field(row, "sh", "CENTER_FIELD")? != "OPERATOR_CANON_UNRESOLVED"
                    || field(row, "identity_exchange", "CENTER_FIELD")? != "0"
                {
                    return Err(error("CENTER_FIELD"));
                }
            }
            "EXPANSION" => {
                expansion_count += 1;
                require_exact_fields(
                    row,
                    &[
                        "id",
                        "instance",
                        "law",
                        "law_sha256",
                        "n_open",
                        "compiled_levels",
                        "books",
                        "cells",
                        "events_per_cell",
                        "selector_axes",
                        "execution_authority",
                        "json",
                    ],
                    "NLEVEL_INSTANCE",
                )?;
                require_value(row, "id", "nlevel_outward_v2", "NLEVEL_INSTANCE")?;
                require_value(row, "instance", NLEVEL_INSTANCE, "NLEVEL_INSTANCE")?;
                require_value(
                    row,
                    "law",
                    "books/LAW-NLEVEL-OUTWARD-FLOWE.md",
                    "NLEVEL_INSTANCE",
                )?;
                require_value(row, "n_open", "1", "NLEVEL_INSTANCE")?;
                if usize_field(row, "compiled_levels", "NLEVEL_INSTANCE")? != NLEVEL_LEVELS
                    || usize_field(row, "books", "NLEVEL_INSTANCE")? != NLEVEL_BOOKS
                    || usize_field(row, "cells", "NLEVEL_INSTANCE")? != NLEVEL_CELLS
                    || usize_field(row, "events_per_cell", "NLEVEL_INSTANCE")? != 10
                    || usize_field(row, "selector_axes", "NLEVEL_INSTANCE")? != NLEVEL_AXES.len()
                    || !valid_sha256(field(row, "law_sha256", "NLEVEL_INSTANCE")?)
                {
                    return Err(error("NLEVEL_INSTANCE"));
                }
            }
            "AXIS" => {
                require_exact_fields(
                    row,
                    &[
                        "id",
                        "ordinal",
                        "independent",
                        "execution_authority",
                        "json",
                    ],
                    "NLEVEL_AXIS_FRAME",
                )?;
                require_value(row, "independent", "1", "NLEVEL_AXIS_FRAME")?;
                let ordinal = usize_field(row, "ordinal", "NLEVEL_AXIS_FRAME")?;
                let identity = field(row, "id", "NLEVEL_AXIS_FRAME")?.to_owned();
                if axes.insert(ordinal, identity).is_some() {
                    return Err(error("NLEVEL_AXIS_FRAME"));
                }
            }
            "BOOK_MEMBER" => {
                require_exact_fields(
                    row,
                    &[
                        "id",
                        "ordinal",
                        "identity_exchange",
                        "execution_authority",
                        "json",
                    ],
                    "NLEVEL_BOOK_SET",
                )?;
                require_value(row, "identity_exchange", "0", "NLEVEL_BOOK_SET")?;
                let ordinal = usize_field(row, "ordinal", "NLEVEL_BOOK_SET")?;
                let identity = field(row, "id", "NLEVEL_BOOK_SET")?.to_owned();
                if books.insert(ordinal, identity).is_some() {
                    return Err(error("NLEVEL_BOOK_SET"));
                }
            }
            "BOOK_RELATION" => {
                book_relation_count += 1;
                require_exact_fields(
                    row,
                    &[
                        "id",
                        "from",
                        "to",
                        "relation",
                        "identity_exchange",
                        "execution_authority",
                        "json",
                    ],
                    "NLEVEL_BOOK_RELATION",
                )?;
                require_value(row, "id", "LIFE_IS_OIL", "NLEVEL_BOOK_RELATION")?;
                require_value(row, "from", "BOOK_OF_LIFE", "NLEVEL_BOOK_RELATION")?;
                require_value(row, "to", "BOOK_OF_OIL", "NLEVEL_BOOK_RELATION")?;
                require_value(row, "relation", "IS", "NLEVEL_BOOK_RELATION")?;
                require_value(row, "identity_exchange", "0", "NLEVEL_BOOK_RELATION")?;
            }
            "LEVEL" => {
                require_exact_fields(
                    row,
                    &[
                        "id",
                        "n",
                        "n_open",
                        "compiled_projection",
                        "execution_authority",
                        "json",
                    ],
                    "NLEVEL_LEVEL_SET",
                )?;
                let n = usize_field(row, "n", "NLEVEL_LEVEL_SET")?;
                if field(row, "id", "NLEVEL_LEVEL_SET")? != format!("level_{n:02}")
                    || !levels.insert(n)
                {
                    return Err(error("NLEVEL_LEVEL_SET"));
                }
                require_value(row, "n_open", "1", "NLEVEL_LEVEL_SET")?;
                require_value(row, "compiled_projection", "1", "NLEVEL_LEVEL_SET")?;
            }
            "GLYPH" => {
                require_exact_fields(
                    row,
                    &["id", "surface", "meaning", "execution_authority", "json"],
                    "GLYPH_FIELD",
                )?;
                glyphs.insert(
                    field(row, "id", "GLYPH_FIELD")?.to_owned(),
                    field(row, "meaning", "GLYPH_FIELD")?.to_owned(),
                );
            }
            "WORD" => {
                require_exact_fields(
                    row,
                    &["id", "glyphs", "meaning", "execution_authority", "json"],
                    "WORD_FIELD",
                )?;
                let components: Vec<String> = field(row, "glyphs", "WORD_FIELD")?
                    .split(',')
                    .map(str::to_owned)
                    .collect();
                if components.is_empty() || components.iter().any(|value| !valid_identity(value)) {
                    return Err(error("WORD_FIELD"));
                }
                words.insert(
                    field(row, "id", "WORD_FIELD")?.to_owned(),
                    Word { glyphs: components },
                );
            }
            "TOKEN" => {
                require_exact_fields(
                    row,
                    &["id", "meaning", "execution_authority", "json"],
                    "NLEVEL_TARGET",
                )?;
                let identity = field(row, "id", "IDENTITY_REQUIRED")?;
                let expected_meaning = match identity {
                    "E_CENTER" => "AETHER_E_CENTER",
                    "FLOWE_TARGET" => "OUTWARD_FLOWE_TARGET",
                    "U_CONNECTION" => "RAINBOW_CONNECTION_U",
                    "O0O_SPHERE" => "SPHERE_POTENTIAL_O0O",
                    _ => return Err(error("NLEVEL_TARGET")),
                };
                require_value(row, "meaning", expected_meaning, "NLEVEL_TARGET")?;
                tokens.insert(identity.to_owned());
            }
            "NODE" => {
                require_fields(row, &["id", "ref", "x", "y", "z"], "NODE_FIELD")?;
                let identity = field(row, "id", "NODE_FIELD")?.to_owned();
                if identity.starts_with("cell_n") {
                    require_exact_fields(
                        row,
                        &[
                            "id",
                            "ref",
                            "kind",
                            "level",
                            "book",
                            "book_ordinal",
                            "q",
                            "x",
                            "y",
                            "z",
                            "px",
                            "py",
                            "time",
                            "colour",
                            "oil_family",
                            "sign",
                            "translucence_q8",
                            "light_q8",
                            "space_radius",
                            "instant_address",
                            "elapsed_measurement_present",
                            "execution_authority",
                            "json",
                        ],
                        "NLEVEL_FORMULA",
                    )?;
                } else {
                    require_exact_fields(
                        row,
                        &[
                            "id",
                            "ref",
                            "kind",
                            "x",
                            "y",
                            "z",
                            "execution_authority",
                            "json",
                        ],
                        "NLEVEL_TARGET",
                    )?;
                }
                let coordinate = (
                    i64_field(row, "x", "NLEVEL_FORMULA")?,
                    i64_field(row, "y", "NLEVEL_FORMULA")?,
                    i64_field(row, "z", "NLEVEL_FORMULA")?,
                );
                if !coordinates.insert(coordinate) || !nodes.insert(identity.clone()) {
                    return Err(error("DUPLICATE_COORDINATE"));
                }
                if identity.starts_with("cell_n") {
                    require_value(row, "kind", "NLEVEL_BOOK_CELL", "NLEVEL_FORMULA")?;
                    let q = usize_field(row, "q", "NLEVEL_FORMULA")?;
                    let n = usize_field(row, "level", "NLEVEL_FORMULA")?;
                    let book_ordinal = usize_field(row, "book_ordinal", "NLEVEL_FORMULA")?;
                    if n >= NLEVEL_LEVELS
                        || book_ordinal >= NLEVEL_BOOKS
                        || q != n * NLEVEL_BOOKS + book_ordinal
                        || identity != format!("cell_n{n:02}_b{book_ordinal:02}")
                    {
                        return Err(error("NLEVEL_FORMULA"));
                    }
                    let book = field(row, "book", "NLEVEL_FORMULA")?.to_owned();
                    if field(row, "ref", "NLEVEL_FORMULA")? != book {
                        return Err(error("NLEVEL_FORMULA"));
                    }
                    let n_i64 = i64::try_from(n).map_err(|_| error("NLEVEL_FORMULA"))?;
                    let book_i64 =
                        i64::try_from(book_ordinal).map_err(|_| error("NLEVEL_FORMULA"))?;
                    let x = book_i64
                        .checked_mul(2)
                        .and_then(|value| value.checked_sub(9))
                        .ok_or_else(|| error("NLEVEL_FORMULA"))?;
                    let y = n_i64
                        .checked_mul(2)
                        .and_then(|value| value.checked_sub(15))
                        .ok_or_else(|| error("NLEVEL_FORMULA"))?;
                    let z = i64::try_from((n + book_ordinal) % 3)
                        .map_err(|_| error("NLEVEL_FORMULA"))?
                        - 1;
                    checked_formula(coordinate.0, x)?;
                    checked_formula(coordinate.1, y)?;
                    checked_formula(coordinate.2, z)?;
                    checked_formula(i64_field(row, "sign", "NLEVEL_FORMULA")?, z)?;
                    checked_formula(
                        i64_field(row, "px", "NLEVEL_FORMULA")?,
                        x.checked_mul(4)
                            .and_then(|value| value.checked_sub(z))
                            .ok_or_else(|| error("NLEVEL_FORMULA"))?,
                    )?;
                    checked_formula(
                        i64_field(row, "py", "NLEVEL_FORMULA")?,
                        y.checked_mul(4)
                            .and_then(|value| value.checked_add(z))
                            .ok_or_else(|| error("NLEVEL_FORMULA"))?,
                    )?;
                    let projection = (
                        i64_field(row, "px", "NLEVEL_FORMULA")?,
                        i64_field(row, "py", "NLEVEL_FORMULA")?,
                    );
                    if !projections.insert(projection) {
                        return Err(error("NLEVEL_PROJECTION_COLLISION"));
                    }
                    if field(row, "time", "NLEVEL_FORMULA")?
                        != NLEVEL_TIME_NAMES[q % NLEVEL_TIME_NAMES.len()]
                        || field(row, "colour", "NLEVEL_FORMULA")?
                            != NLEVEL_COLOURS[(n + book_ordinal) % NLEVEL_COLOURS.len()]
                    {
                        return Err(error("NLEVEL_FORMULA"));
                    }
                    let oil_family =
                        NLEVEL_OIL_FAMILIES[(n + 2 * book_ordinal) % NLEVEL_OIL_FAMILIES.len()];
                    if field(row, "oil_family", "NLEVEL_FORMULA")? != oil_family {
                        return Err(error("NLEVEL_FORMULA"));
                    }
                    let translucence_q8 = (17 * n + 23 * book_ordinal) % 256;
                    let light_q8 = (29 * n + 31 * book_ordinal) % 256;
                    if usize_field(row, "translucence_q8", "NLEVEL_FORMULA")? != translucence_q8
                        || usize_field(row, "light_q8", "NLEVEL_FORMULA")? != light_q8
                        || usize_field(row, "space_radius", "NLEVEL_FORMULA")? != n + 1
                    {
                        return Err(error("NLEVEL_FORMULA"));
                    }
                    require_timing(row)?;
                    if cells
                        .insert(
                            q,
                            NLevelCell {
                                identity,
                                book,
                                n,
                                book_ordinal,
                                q,
                                translucence_q8,
                                oil_family: oil_family.to_owned(),
                                space_radius: n + 1,
                            },
                        )
                        .is_some()
                    {
                        return Err(error("NLEVEL_CELL_SET"));
                    }
                } else {
                    match identity.as_str() {
                        "n_e" => {
                            require_value(row, "ref", "E_CENTER", "NLEVEL_TARGET")?;
                            require_value(row, "kind", "CENTER", "NLEVEL_TARGET")?;
                            if coordinate != (-12, 0, 0) {
                                return Err(error("NLEVEL_TARGET"));
                            }
                        }
                        "n_flowe_target" => {
                            require_value(row, "ref", "FLOWE_TARGET", "NLEVEL_TARGET")?;
                            require_value(row, "kind", "TARGET", "NLEVEL_TARGET")?;
                            if coordinate != (12, 0, 0) {
                                return Err(error("NLEVEL_TARGET"));
                            }
                        }
                        "n_u" => {
                            require_value(row, "ref", "U_CONNECTION", "NLEVEL_TARGET")?;
                            require_value(row, "kind", "CONNECTION", "NLEVEL_TARGET")?;
                            if coordinate != (0, -18, 0) {
                                return Err(error("NLEVEL_TARGET"));
                            }
                        }
                        "n_o0o" => {
                            require_value(row, "ref", "O0O_SPHERE", "NLEVEL_TARGET")?;
                            require_value(row, "kind", "SPHERE", "NLEVEL_TARGET")?;
                            if coordinate != (0, 18, 0) {
                                return Err(error("NLEVEL_TARGET"));
                            }
                        }
                        _ => return Err(error("NLEVEL_TARGET")),
                    }
                }
            }
            "CALLING_JOIN" => {
                require_exact_fields(
                    row,
                    &[
                        "id",
                        "from",
                        "to",
                        "direction",
                        "endpoints_retained",
                        "semantic_carry_only",
                        "execution_authority",
                        "json",
                    ],
                    "CALLING_FIELD_MISMATCH",
                )?;
                require_value(
                    row,
                    "id",
                    "v1_calling_semantics_reference",
                    "CALLING_FIELD_MISMATCH",
                )?;
                calling_joins.push(row);
            }
            "PULSE"
            | "SHADOW_EXTRACT"
            | "CALMING_OIL"
            | "CALMING_OIL_OUTWARD"
            | "CALLING_INTO_E"
            | "CALLING_INTO_FLOWE"
            | "CALLING_INTO_U"
            | "FLOWE_TO_O0O"
            | "SELF_REDUCTION"
            | "FLOWe" => {
                let specific_fields: &[&str] = match row.tag.as_str() {
                    "PULSE" => &["pulse"],
                    "SHADOW_EXTRACT" => &["translucence_q8"],
                    "CALMING_OIL" => &["oil_family", "oil_amplitude"],
                    "CALMING_OIL_OUTWARD" => &["oil_family", "oil_amplitude", "repetition"],
                    "CALLING_INTO_E" | "CALLING_INTO_FLOWE" | "CALLING_INTO_U" | "FLOWE_TO_O0O" => {
                        &["operator_bound"]
                    }
                    "SELF_REDUCTION" => &["self_reduction", "identity_exchange", "deletion"],
                    "FLOWe" => &["step"],
                    _ => return Err(error("NLEVEL_EVENT_FIELDS")),
                };
                require_exact_fields_union(
                    row,
                    &NLEVEL_EVENT_COMMON_FIELDS,
                    specific_fields,
                    "NLEVEL_EVENT_FIELDS",
                )?;
                ledgers.entry(row.tag.as_str()).or_default().push(row);
            }
            "TIMING_BOUNDARY" => {
                timing_count += 1;
                require_exact_fields(
                    row,
                    &[
                        "instant_address",
                        "elapsed_measurement_present",
                        "elapsed_claim",
                        "execution_authority",
                        "json",
                    ],
                    "NLEVEL_INSTANT_BOUNDARY",
                )?;
                require_timing(row)?;
                require_value(
                    row,
                    "elapsed_claim",
                    "UNMEASURED",
                    "NLEVEL_INSTANT_BOUNDARY",
                )?;
            }
            "BOUNDARY" => {
                boundary_count += 1;
                require_exact_fields(
                    row,
                    &[
                        "system_affirmed",
                        "physical_mapping",
                        "clinical_mapping",
                        "runtime_mapping",
                        "cartesian_population_claim",
                        "source_video_bytes",
                        "execution_authority",
                        "json",
                    ],
                    "NLEVEL_BOUNDARY",
                )?;
                require_value(row, "system_affirmed", "0", "NLEVEL_BOUNDARY")?;
                require_value(row, "cartesian_population_claim", "0", "NLEVEL_BOUNDARY")?;
                require_value(row, "source_video_bytes", "0", "NLEVEL_BOUNDARY")?;
                require_value(row, "physical_mapping", "UNVERIFIED", "NLEVEL_BOUNDARY")?;
                require_value(row, "clinical_mapping", "UNVERIFIED", "NLEVEL_BOUNDARY")?;
                require_value(row, "runtime_mapping", "UNVERIFIED", "NLEVEL_BOUNDARY")?;
            }
            "END" => {
                if index != last_index {
                    return Err(error("ROW_ORDER"));
                }
                require_exact_fields(
                    row,
                    &[
                        "status",
                        "n_open",
                        "compiled_levels",
                        "cells",
                        "relation_rows",
                        "execution_authority",
                        "json",
                    ],
                    "NLEVEL_END",
                )?;
                require_value(row, "status", "COMPILED_BOUNDED_PROJECTION", "NLEVEL_END")?;
                require_value(row, "n_open", "1", "NLEVEL_END")?;
                if usize_field(row, "compiled_levels", "NLEVEL_END")? != NLEVEL_LEVELS
                    || usize_field(row, "cells", "NLEVEL_END")? != NLEVEL_CELLS
                    || usize_field(row, "relation_rows", "NLEVEL_END")? != 10 * NLEVEL_CELLS
                {
                    return Err(error("NLEVEL_END"));
                }
                end_count += 1;
            }
            _ => return Err(error("ROW_TAG")),
        }
    }

    if readfirst_count != 1
        || language_count != 1
        || source_count != 1
        || report_count != 2
        || grammar_binding_count != 1
        || center_count != 1
        || expansion_count != 1
        || book_relation_count != 1
        || timing_count != 1
        || boundary_count != 1
        || end_count != 1
    {
        return Err(error("NLEVEL_CONTROL_COUNT"));
    }
    if axes.len() != NLEVEL_AXES.len()
        || NLEVEL_AXES
            .iter()
            .enumerate()
            .any(|(ordinal, expected)| axes.get(&ordinal).map(String::as_str) != Some(*expected))
    {
        return Err(error("NLEVEL_AXIS_FRAME"));
    }
    if books.len() != NLEVEL_BOOKS
        || NLEVEL_BOOK_IDS
            .iter()
            .enumerate()
            .any(|(ordinal, expected)| books.get(&ordinal).map(String::as_str) != Some(*expected))
        || levels.len() != NLEVEL_LEVELS
        || (0..NLEVEL_LEVELS).any(|n| !levels.contains(&n))
    {
        return Err(error("NLEVEL_LEVEL_BOOK_SET"));
    }
    if cells.len() != NLEVEL_CELLS
        || (0..NLEVEL_CELLS).any(|q| !cells.contains_key(&q))
        || nodes.len() != NLEVEL_CELLS + 4
        || !nodes.contains("n_e")
        || !nodes.contains("n_flowe_target")
        || !nodes.contains("n_u")
        || !nodes.contains("n_o0o")
    {
        return Err(error("NLEVEL_CELL_SET"));
    }
    for cell in cells.values() {
        if books.get(&cell.book_ordinal) != Some(&cell.book)
            || cell.q != cell.n * 10 + cell.book_ordinal
        {
            return Err(error("NLEVEL_CELL_SET"));
        }
    }
    if !glyphs.is_empty() {
        if glyphs.get("V").map(String::as_str) != Some("VELOCITY") {
            return Err(error("V_VELOCITY_REQUIRED"));
        }
        for word in words.values() {
            if word.glyphs.iter().any(|glyph| !glyphs.contains_key(glyph)) {
                return Err(error("GLYPH_UNRESOLVED"));
            }
        }
        if let Some(vector) = words.get("Vector") {
            if vector
                .glyphs
                .iter()
                .map(String::as_str)
                .ne(["V", "e", "c", "t", "o", "r"])
            {
                return Err(error("VECTOR_COMPOSITION_MISMATCH"));
            }
        }
    }
    if tokens
        != ["E_CENTER", "FLOWE_TARGET", "O0O_SPHERE", "U_CONNECTION"]
            .into_iter()
            .map(str::to_owned)
            .collect()
    {
        return Err(error("NLEVEL_TARGET"));
    }

    let ledger_names = [
        "PULSE",
        "SHADOW_EXTRACT",
        "CALMING_OIL",
        "CALMING_OIL_OUTWARD",
        "CALLING_INTO_E",
        "CALLING_INTO_FLOWE",
        "CALLING_INTO_U",
        "FLOWE_TO_O0O",
        "SELF_REDUCTION",
        "FLOWe",
    ];
    let mut flowe_chain = BTreeMap::<usize, (String, String)>::new();
    for tag in ledger_names {
        let ledger = ledgers
            .get(tag)
            .ok_or_else(|| error("NLEVEL_LEDGER_COUNT"))?;
        if ledger.len() != NLEVEL_CELLS {
            return Err(error("NLEVEL_LEDGER_COUNT"));
        }
        let mut seen_q = BTreeSet::new();
        for row in ledger {
            let q = usize_field(row, "q", "NLEVEL_RELATION")?;
            let cell = cells.get(&q).ok_or_else(|| error("NLEVEL_RELATION"))?;
            let expected_id = format!(
                "{}_n{:02}_b{:02}",
                tag.to_ascii_lowercase(),
                cell.n,
                cell.book_ordinal
            );
            if !seen_q.insert(q)
                || field(row, "id", "NLEVEL_RELATION")? != expected_id
                || field(row, "cell", "NLEVEL_RELATION")? != cell.identity
                || usize_field(row, "level", "NLEVEL_RELATION")? != cell.n
                || field(row, "book", "NLEVEL_RELATION")? != cell.book
            {
                return Err(error("NLEVEL_RELATION"));
            }
            require_timing(row)?;
            let from = field(row, "from", "NLEVEL_RELATION")?;
            let to = field(row, "to", "NLEVEL_RELATION")?;
            if !nodes.contains(from) || !nodes.contains(to) {
                return Err(error("EDGE_ENDPOINT_UNRESOLVED"));
            }
            match tag {
                "PULSE" => {
                    if from != "n_flowe_target"
                        || to != cell.identity
                        || field(row, "direction", "NLEVEL_DIRECTION")? != "OUTWARD"
                        || usize_field(row, "pulse", "NLEVEL_FORMULA")? != cell.space_radius
                    {
                        return Err(error("NLEVEL_DIRECTION"));
                    }
                }
                "SHADOW_EXTRACT" => {
                    if from != "n_e"
                        || to != cell.identity
                        || field(row, "direction", "NLEVEL_DIRECTION")? != "OUTWARD"
                        || usize_field(row, "translucence_q8", "NLEVEL_FORMULA")?
                            != cell.translucence_q8
                    {
                        return Err(error("NLEVEL_DIRECTION"));
                    }
                }
                "CALMING_OIL" => {
                    if from != cell.identity
                        || to != "n_e"
                        || field(row, "direction", "NLEVEL_DIRECTION")? != "TOWARD_E"
                        || field(row, "oil_family", "NLEVEL_FORMULA")? != cell.oil_family
                        || usize_field(row, "oil_amplitude", "NLEVEL_FORMULA")? != cell.space_radius
                    {
                        return Err(error("NLEVEL_DIRECTION"));
                    }
                }
                "CALMING_OIL_OUTWARD" => {
                    if from != "n_o0o"
                        || to != cell.identity
                        || field(row, "direction", "NLEVEL_DIRECTION")? != "OUTWARD"
                        || field(row, "oil_family", "NLEVEL_FORMULA")? != cell.oil_family
                        || usize_field(row, "oil_amplitude", "NLEVEL_FORMULA")? != cell.space_radius
                        || field(row, "repetition", "NLEVEL_DIRECTION")? != "N_OPEN"
                    {
                        return Err(error("NLEVEL_DIRECTION"));
                    }
                }
                "CALLING_INTO_E" => {
                    if from != cell.identity
                        || to != "n_e"
                        || field(row, "direction", "NLEVEL_DIRECTION")? != "INTO_E"
                        || field(row, "operator_bound", "NLEVEL_DIRECTION")? != "1"
                    {
                        return Err(error("NLEVEL_DIRECTION"));
                    }
                }
                "CALLING_INTO_FLOWE" => {
                    if from != cell.identity
                        || to != "n_flowe_target"
                        || field(row, "direction", "NLEVEL_DIRECTION")? != "INTO_FLOWE"
                        || field(row, "operator_bound", "NLEVEL_DIRECTION")? != "1"
                    {
                        return Err(error("NLEVEL_DIRECTION"));
                    }
                }
                "CALLING_INTO_U" => {
                    if from != cell.identity
                        || to != "n_u"
                        || field(row, "direction", "NLEVEL_DIRECTION")? != "INTO_U"
                        || field(row, "operator_bound", "NLEVEL_DIRECTION")? != "1"
                    {
                        return Err(error("NLEVEL_DIRECTION"));
                    }
                }
                "FLOWE_TO_O0O" => {
                    if from != cell.identity
                        || to != "n_o0o"
                        || field(row, "direction", "NLEVEL_DIRECTION")? != "INTO_O0O"
                        || field(row, "operator_bound", "NLEVEL_DIRECTION")? != "1"
                    {
                        return Err(error("NLEVEL_DIRECTION"));
                    }
                }
                "SELF_REDUCTION" => {
                    if from != cell.identity
                        || to != "n_o0o"
                        || field(row, "direction", "NLEVEL_DIRECTION")? != "TOWARD_O0O"
                        || field(row, "self_reduction", "NLEVEL_DIRECTION")? != "1"
                        || field(row, "identity_exchange", "NLEVEL_DIRECTION")? != "0"
                        || field(row, "deletion", "NLEVEL_DIRECTION")? != "0"
                    {
                        return Err(error("NLEVEL_DIRECTION"));
                    }
                }
                "FLOWe" => {
                    if usize_field(row, "step", "NLEVEL_FLOWE_CHAIN")? != q
                        || field(row, "direction", "NLEVEL_FLOWE_CHAIN")? != "FORWARD"
                    {
                        return Err(error("NLEVEL_FLOWE_CHAIN"));
                    }
                    flowe_chain.insert(q, (from.to_owned(), to.to_owned()));
                }
                _ => return Err(error("NLEVEL_LEDGER_COUNT")),
            }
        }
        if seen_q.len() != NLEVEL_CELLS {
            return Err(error("NLEVEL_LEDGER_COUNT"));
        }
    }
    for q in 0..NLEVEL_CELLS {
        let (from, to) = flowe_chain
            .get(&q)
            .ok_or_else(|| error("NLEVEL_FLOWE_CHAIN"))?;
        let expected_from = if q == 0 {
            "n_flowe_target"
        } else {
            cells
                .get(&(q - 1))
                .map(|cell| cell.identity.as_str())
                .ok_or_else(|| error("NLEVEL_FLOWE_CHAIN"))?
        };
        let expected_to = cells
            .get(&q)
            .map(|cell| cell.identity.as_str())
            .ok_or_else(|| error("NLEVEL_FLOWE_CHAIN"))?;
        if from != expected_from || to != expected_to {
            return Err(error("NLEVEL_FLOWE_CHAIN"));
        }
        if q > 0 {
            let previous_to = &flowe_chain
                .get(&(q - 1))
                .ok_or_else(|| error("NLEVEL_FLOWE_CHAIN"))?
                .1;
            if previous_to != from {
                return Err(error("NLEVEL_FLOWE_CHAIN"));
            }
        }
    }
    if calling_joins.len() != 1 {
        return Err(error("CALLING_FIELD_MISMATCH"));
    }
    for row in &calling_joins {
        if field(row, "direction", "CALLING_FIELD_MISMATCH")? != "UNRESOLVED"
            || field(row, "endpoints_retained", "CALLING_FIELD_MISMATCH")? != "1"
            || field(row, "from", "CALLING_FIELD_MISMATCH")? != "n_e"
            || field(row, "to", "CALLING_FIELD_MISMATCH")? != "n_flowe_target"
            || field(row, "semantic_carry_only", "CALLING_FIELD_MISMATCH")? != "1"
            || row.fields.contains_key("step")
        {
            return Err(error("CALLING_FIELD_MISMATCH"));
        }
    }

    Ok(Validation {
        records: rows.len(),
        glyphs: glyphs.len(),
        words: words.len(),
        nodes: nodes.len(),
        calling_joins: calling_joins.len(),
        flowe_edges: NLEVEL_CELLS,
        other_relations: 9 * NLEVEL_CELLS,
        nlevel: Some(NLevelValidation {
            axes: axes.len(),
            books: books.len(),
            levels: levels.len(),
            cells: cells.len(),
            ledgers: ledger_names.len(),
            ledger_rows: ledger_names.len() * NLEVEL_CELLS,
            calling_joins: calling_joins.len(),
        }),
        spread: None,
        hookwall: None,
    })
}

// This validator checks the additive R2 projection while retaining the two
// referenced parent byte streams as separately sealed dependencies.
fn validate_spread_rows(rows: &[Row]) -> Result<Validation> {
    if rows.len() != SPREAD_RECORDS {
        return Err(error("SPREAD_RECORD_COUNT"));
    }

    let mut parents = BTreeSet::<String>::new();
    let mut rounds = BTreeSet::<usize>::new();
    let mut anchors = BTreeMap::<String, String>::new();
    let mut cells = BTreeMap::<usize, NLevelCell>::new();
    let mut ledgers = BTreeMap::<String, BTreeSet<(usize, usize)>>::new();
    let mut event_ids = BTreeSet::<String>::new();
    let mut source_sha = None::<String>;
    let mut readfirst_count = 0_usize;
    let mut language_count = 0_usize;
    let mut source_count = 0_usize;
    let mut center_count = 0_usize;
    let mut expansion_count = 0_usize;
    let mut parent_invariant_count = 0_usize;
    let mut timing_count = 0_usize;
    let mut boundary_count = 0_usize;
    let mut end_count = 0_usize;
    let last_index = rows.len() - 1;

    for (index, row) in rows.iter().enumerate() {
        require_authority_zero(row)?;
        match row.tag.as_str() {
            "READFIRST" => {
                require_exact_fields(
                    row,
                    &["url", "execution_authority", "json"],
                    "SPREAD_CONTROL_FIELDS",
                )?;
                if index != 0 {
                    return Err(error("ROW_ORDER"));
                }
                require_value(row, "url", READFIRST_URL, "SPREAD_CONTROL_FIELDS")?;
                readfirst_count += 1;
            }
            "LANGUAGE" => {
                require_exact_fields(
                    row,
                    &[
                        "id",
                        "instance",
                        "tuple_frame",
                        "selector_axes",
                        "execution_authority",
                        "json",
                    ],
                    "SPREAD_INSTANCE",
                )?;
                require_value(row, "id", LANGUAGE_ID, "SPREAD_INSTANCE")?;
                require_value(row, "instance", SPREAD_INSTANCE, "SPREAD_INSTANCE")?;
                require_value(row, "tuple_frame", "HYPERBEHCS_60D_PLUS", "SPREAD_INSTANCE")?;
                if usize_field(row, "selector_axes", "SPREAD_INSTANCE")? != NLEVEL_AXES.len() {
                    return Err(error("SPREAD_INSTANCE"));
                }
                language_count += 1;
            }
            "PARENT" => {
                require_exact_fields(
                    row,
                    &[
                        "id",
                        "instance",
                        "path",
                        "sha256",
                        "records",
                        "mutation",
                        "identity_exchange",
                        "execution_authority",
                        "json",
                    ],
                    "SPREAD_PARENT",
                )?;
                let identity = field(row, "id", "SPREAD_PARENT")?.to_owned();
                if !parents.insert(identity.clone()) {
                    return Err(error("SPREAD_PARENT"));
                }
                let (instance, path, digest, records) = match identity.as_str() {
                    "V1_CORE" => (
                        LANGUAGE_ID,
                        "language/core.flowe",
                        V1_CORE_SHA256,
                        118_usize,
                    ),
                    "N16_CORE" => (
                        NLEVEL_INSTANCE,
                        "language/outward-n16.flowe",
                        N16_CORE_SHA256,
                        NLEVEL_RECORDS,
                    ),
                    _ => return Err(error("SPREAD_PARENT")),
                };
                require_value(row, "instance", instance, "SPREAD_PARENT")?;
                require_value(row, "path", path, "SPREAD_PARENT")?;
                require_value(row, "sha256", digest, "SPREAD_PARENT")?;
                if usize_field(row, "records", "SPREAD_PARENT")? != records {
                    return Err(error("SPREAD_PARENT"));
                }
                require_value(row, "mutation", "0", "SPREAD_PARENT")?;
                require_value(row, "identity_exchange", "0", "SPREAD_PARENT")?;
            }
            "SOURCE" => {
                require_exact_fields(
                    row,
                    &[
                        "path",
                        "occurrences",
                        "sha256",
                        "evidence",
                        "execution_authority",
                        "json",
                    ],
                    "SPREAD_SOURCE",
                )?;
                require_value(
                    row,
                    "path",
                    "books/LAW-INCREASING-CALLINGS-CALMINGS-WORD-FLOWE-SPREAD.md",
                    "SPREAD_SOURCE",
                )?;
                require_value(row, "occurrences", "1", "SPREAD_SOURCE")?;
                require_value(row, "evidence", "OPERATOR_CANON", "SPREAD_SOURCE")?;
                let digest = field(row, "sha256", "SPREAD_SOURCE")?;
                if !valid_sha256(digest) {
                    return Err(error("SPREAD_SOURCE"));
                }
                source_sha = Some(digest.to_owned());
                source_count += 1;
            }
            "CENTER" => {
                require_exact_fields(
                    row,
                    &[
                        "members",
                        "traversal_surface",
                        "sh",
                        "identity_exchange",
                        "execution_authority",
                        "json",
                    ],
                    "SPREAD_CENTER",
                )?;
                require_value(row, "members", "HBI,HBP,SHA,SH,HASH", "SPREAD_CENTER")?;
                require_value(
                    row,
                    "traversal_surface",
                    "HBI,HBP,SHA,SH,HASH",
                    "SPREAD_CENTER",
                )?;
                require_value(row, "sh", "OPERATOR_CANON_UNRESOLVED", "SPREAD_CENTER")?;
                require_value(row, "identity_exchange", "0", "SPREAD_CENTER")?;
                center_count += 1;
            }
            "EXPANSION" => {
                require_exact_fields(
                    row,
                    &[
                        "id",
                        "instance",
                        "law",
                        "law_sha256",
                        "v1",
                        "v1_sha256",
                        "base",
                        "base_sha256",
                        "builder",
                        "builder_sha256",
                        "n_open",
                        "compiled_rounds",
                        "cells",
                        "event_kinds",
                        "relation_rows",
                        "selector_axes",
                        "execution_authority",
                        "json",
                    ],
                    "SPREAD_INSTANCE",
                )?;
                require_value(row, "id", "word_flowe_spread_v3", "SPREAD_INSTANCE")?;
                require_value(row, "instance", SPREAD_INSTANCE, "SPREAD_INSTANCE")?;
                require_value(
                    row,
                    "law",
                    "books/LAW-INCREASING-CALLINGS-CALMINGS-WORD-FLOWE-SPREAD.md",
                    "SPREAD_INSTANCE",
                )?;
                if source_sha.as_deref() != Some(field(row, "law_sha256", "SPREAD_INSTANCE")?) {
                    return Err(error("SPREAD_INSTANCE"));
                }
                require_value(row, "v1", "language/core.flowe", "SPREAD_INSTANCE")?;
                require_value(row, "v1_sha256", V1_CORE_SHA256, "SPREAD_INSTANCE")?;
                require_value(row, "base", "language/outward-n16.flowe", "SPREAD_INSTANCE")?;
                require_value(row, "base_sha256", N16_CORE_SHA256, "SPREAD_INSTANCE")?;
                require_value(
                    row,
                    "builder",
                    "tools/build_word_flowe_spread.py",
                    "SPREAD_INSTANCE",
                )?;
                if !valid_sha256(field(row, "builder_sha256", "SPREAD_INSTANCE")?) {
                    return Err(error("SPREAD_INSTANCE"));
                }
                require_value(row, "n_open", "1", "SPREAD_INSTANCE")?;
                if usize_field(row, "compiled_rounds", "SPREAD_INSTANCE")? != SPREAD_ROUNDS
                    || usize_field(row, "cells", "SPREAD_INSTANCE")? != NLEVEL_CELLS
                    || usize_field(row, "event_kinds", "SPREAD_INSTANCE")?
                        != SPREAD_EVENT_KINDS.len()
                    || usize_field(row, "relation_rows", "SPREAD_INSTANCE")? != SPREAD_RELATIONS
                    || usize_field(row, "selector_axes", "SPREAD_INSTANCE")? != NLEVEL_AXES.len()
                {
                    return Err(error("SPREAD_INSTANCE"));
                }
                expansion_count += 1;
            }
            "PARENT_INVARIANT" => {
                require_exact_fields(
                    row,
                    &[
                        "id",
                        "parent",
                        "tag",
                        "rows",
                        "identity_exchange",
                        "deletion",
                        "validated",
                        "execution_authority",
                        "json",
                    ],
                    "SPREAD_PARENT_INVARIANT",
                )?;
                require_value(row, "id", "N16_SELF_REDUCTION", "SPREAD_PARENT_INVARIANT")?;
                require_value(row, "parent", NLEVEL_INSTANCE, "SPREAD_PARENT_INVARIANT")?;
                require_value(row, "tag", "SELF_REDUCTION", "SPREAD_PARENT_INVARIANT")?;
                if usize_field(row, "rows", "SPREAD_PARENT_INVARIANT")? != NLEVEL_CELLS {
                    return Err(error("SPREAD_PARENT_INVARIANT"));
                }
                require_value(row, "identity_exchange", "0", "SPREAD_PARENT_INVARIANT")?;
                require_value(row, "deletion", "0", "SPREAD_PARENT_INVARIANT")?;
                require_value(row, "validated", "1", "SPREAD_PARENT_INVARIANT")?;
                parent_invariant_count += 1;
            }
            "ROUND" => {
                require_exact_fields(
                    row,
                    &[
                        "id",
                        "ordinal",
                        "previous",
                        "increase_q",
                        "n_open",
                        "compiled_projection",
                        "execution_authority",
                        "json",
                    ],
                    "SPREAD_ROUND",
                )?;
                let growth_round = usize_field(row, "ordinal", "SPREAD_ROUND")?;
                if !(1..=SPREAD_ROUNDS).contains(&growth_round)
                    || !rounds.insert(growth_round)
                    || field(row, "id", "SPREAD_ROUND")? != format!("round_{growth_round:02}")
                    || usize_field(row, "increase_q", "SPREAD_ROUND")? != growth_round
                {
                    return Err(error("SPREAD_ROUND"));
                }
                let previous = if growth_round == 1 {
                    NLEVEL_INSTANCE.to_owned()
                } else {
                    format!("round_{:02}", growth_round - 1)
                };
                require_value(row, "previous", &previous, "SPREAD_ROUND")?;
                require_value(row, "n_open", "1", "SPREAD_ROUND")?;
                require_value(row, "compiled_projection", "1", "SPREAD_ROUND")?;
            }
            "ANCHOR" => {
                require_exact_fields(
                    row,
                    &["id", "meaning", "execution_authority", "json"],
                    "SPREAD_ANCHOR",
                )?;
                let identity = field(row, "id", "SPREAD_ANCHOR")?.to_owned();
                let meaning = field(row, "meaning", "SPREAD_ANCHOR")?.to_owned();
                if anchors.insert(identity, meaning).is_some() {
                    return Err(error("SPREAD_ANCHOR"));
                }
            }
            "CELL_REF" => {
                require_exact_fields(
                    row,
                    &[
                        "id",
                        "parent_instance",
                        "parent_id",
                        "reference_only",
                        "level",
                        "book",
                        "book_ordinal",
                        "q",
                        "space_radius",
                        "x",
                        "y",
                        "z",
                        "time",
                        "colour",
                        "oil_family",
                        "identity_exchange",
                        "execution_authority",
                        "json",
                    ],
                    "SPREAD_CELL",
                )?;
                let q = usize_field(row, "q", "SPREAD_CELL")?;
                let n = usize_field(row, "level", "SPREAD_CELL")?;
                let book_ordinal = usize_field(row, "book_ordinal", "SPREAD_CELL")?;
                let identity = format!("cell_n{n:02}_b{book_ordinal:02}");
                if n >= NLEVEL_LEVELS
                    || book_ordinal >= NLEVEL_BOOKS
                    || q != n * NLEVEL_BOOKS + book_ordinal
                    || field(row, "id", "SPREAD_CELL")? != identity
                    || field(row, "parent_id", "SPREAD_CELL")? != identity
                {
                    return Err(error("SPREAD_CELL"));
                }
                require_value(row, "parent_instance", NLEVEL_INSTANCE, "SPREAD_CELL")?;
                require_value(row, "reference_only", "1", "SPREAD_CELL")?;
                require_value(row, "book", NLEVEL_BOOK_IDS[book_ordinal], "SPREAD_CELL")?;
                let n_i64 = i64::try_from(n).map_err(|_| error("SPREAD_CELL"))?;
                let book_i64 = i64::try_from(book_ordinal).map_err(|_| error("SPREAD_CELL"))?;
                let expected_x = book_i64 * 2 - 9;
                let expected_y = n_i64 * 2 - 15;
                let expected_z =
                    i64::try_from((n + book_ordinal) % 3).map_err(|_| error("SPREAD_CELL"))? - 1;
                if i64_field(row, "x", "SPREAD_CELL")? != expected_x
                    || i64_field(row, "y", "SPREAD_CELL")? != expected_y
                    || i64_field(row, "z", "SPREAD_CELL")? != expected_z
                    || field(row, "time", "SPREAD_CELL")?
                        != NLEVEL_TIME_NAMES[q % NLEVEL_TIME_NAMES.len()]
                    || field(row, "colour", "SPREAD_CELL")?
                        != NLEVEL_COLOURS[(n + book_ordinal) % NLEVEL_COLOURS.len()]
                    || usize_field(row, "space_radius", "SPREAD_CELL")? != n + 1
                {
                    return Err(error("SPREAD_CELL"));
                }
                let oil_family =
                    NLEVEL_OIL_FAMILIES[(n + 2 * book_ordinal) % NLEVEL_OIL_FAMILIES.len()];
                require_value(row, "oil_family", oil_family, "SPREAD_CELL")?;
                require_value(row, "identity_exchange", "0", "SPREAD_CELL")?;
                let cell = NLevelCell {
                    identity,
                    book: NLEVEL_BOOK_IDS[book_ordinal].to_owned(),
                    n,
                    book_ordinal,
                    q,
                    translucence_q8: 0,
                    oil_family: oil_family.to_owned(),
                    space_radius: n + 1,
                };
                if cells.insert(q, cell).is_some() {
                    return Err(error("SPREAD_CELL"));
                }
            }
            "TIMING_BOUNDARY" => {
                require_exact_fields(
                    row,
                    &[
                        "instant_address",
                        "elapsed_measurement_present",
                        "runtime_measurement_present",
                        "execution_authority",
                        "json",
                    ],
                    "SPREAD_TIMING",
                )?;
                require_value(row, "instant_address", "1", "SPREAD_TIMING")?;
                require_value(row, "elapsed_measurement_present", "0", "SPREAD_TIMING")?;
                require_value(row, "runtime_measurement_present", "0", "SPREAD_TIMING")?;
                timing_count += 1;
            }
            "BOUNDARY" => {
                require_exact_fields(
                    row,
                    &[
                        "physical_mapping",
                        "live_runtime_mapping",
                        "system_affirmed",
                        "two_round_compilation",
                        "execution_authority",
                        "json",
                    ],
                    "SPREAD_BOUNDARY",
                )?;
                require_value(row, "physical_mapping", "UNVERIFIED", "SPREAD_BOUNDARY")?;
                require_value(row, "live_runtime_mapping", "UNVERIFIED", "SPREAD_BOUNDARY")?;
                require_value(row, "system_affirmed", "0", "SPREAD_BOUNDARY")?;
                require_value(row, "two_round_compilation", "DESIGN", "SPREAD_BOUNDARY")?;
                boundary_count += 1;
            }
            "END" => {
                require_exact_fields(
                    row,
                    &[
                        "status",
                        "n_open",
                        "compiled_rounds",
                        "cells",
                        "event_kinds",
                        "relation_rows",
                        "execution_authority",
                        "json",
                    ],
                    "SPREAD_END",
                )?;
                if index != last_index {
                    return Err(error("ROW_ORDER"));
                }
                require_value(
                    row,
                    "status",
                    "COMPILED_BOUNDED_GROWTH_PROJECTION",
                    "SPREAD_END",
                )?;
                require_value(row, "n_open", "1", "SPREAD_END")?;
                if usize_field(row, "compiled_rounds", "SPREAD_END")? != SPREAD_ROUNDS
                    || usize_field(row, "cells", "SPREAD_END")? != NLEVEL_CELLS
                    || usize_field(row, "event_kinds", "SPREAD_END")? != SPREAD_EVENT_KINDS.len()
                    || usize_field(row, "relation_rows", "SPREAD_END")? != SPREAD_RELATIONS
                {
                    return Err(error("SPREAD_END"));
                }
                end_count += 1;
            }
            tag if SPREAD_EVENT_KINDS.contains(&tag) => {
                let specific: &[&str] = match tag {
                    "CALLING_GROWTH_E" | "CALLING_GROWTH_FLOWE" | "CALLING_GROWTH_U" => {
                        &["operator_bound"]
                    }
                    "CALMING_GROWTH_E" | "CALMING_GROWTH_OUTWARD" => {
                        &["oil_family", "oil_amplitude"]
                    }
                    "SELF_REDUCTION_GROWTH" => &["self_reduction", "deletion"],
                    "WORD_SPREAD" | "FLOWE_SPREAD" => &["spread", "spread_step"],
                    _ => return Err(error("SPREAD_EVENT_FIELDS")),
                };
                require_exact_fields_union(
                    row,
                    &SPREAD_EVENT_COMMON_FIELDS,
                    specific,
                    "SPREAD_EVENT_FIELDS",
                )?;
                let q = usize_field(row, "q", "SPREAD_EVENT")?;
                let growth_round = usize_field(row, "round", "SPREAD_EVENT")?;
                let cell = cells.get(&q).ok_or_else(|| error("SPREAD_EVENT"))?;
                if !(1..=SPREAD_ROUNDS).contains(&growth_round)
                    || usize_field(row, "increase_q", "SPREAD_EVENT")? != growth_round
                    || usize_field(row, "amplitude_q", "SPREAD_EVENT")?
                        != cell.space_radius + growth_round
                    || field(row, "cell", "SPREAD_EVENT")? != cell.identity
                    || usize_field(row, "level", "SPREAD_EVENT")? != cell.n
                    || field(row, "book", "SPREAD_EVENT")? != cell.book
                {
                    return Err(error("SPREAD_EVENT"));
                }
                let expected_id = format!(
                    "{}_r{growth_round:02}_n{:02}_b{:02}",
                    tag.to_ascii_lowercase(),
                    cell.n,
                    cell.book_ordinal
                );
                if field(row, "id", "SPREAD_EVENT")? != expected_id
                    || !event_ids.insert(expected_id)
                {
                    return Err(error("SPREAD_EVENT"));
                }
                require_value(row, "n_open", "1", "SPREAD_EVENT")?;
                require_value(row, "instant_address", "1", "SPREAD_TIMING")?;
                require_value(row, "elapsed_measurement_present", "0", "SPREAD_TIMING")?;
                require_value(row, "identity_exchange", "0", "SPREAD_EVENT")?;
                require_value(row, "source_retained", "1", "SPREAD_EVENT")?;
                let amplitude = (cell.space_radius + growth_round).to_string();
                match tag {
                    "CALLING_GROWTH_E" => {
                        require_value(row, "from", &cell.identity, "SPREAD_EVENT")?;
                        require_value(row, "to", "n_e", "SPREAD_EVENT")?;
                        require_value(row, "direction", "INCREASING_INTO_E", "SPREAD_EVENT")?;
                        require_value(row, "operator_bound", "1", "SPREAD_EVENT")?;
                    }
                    "CALLING_GROWTH_FLOWE" => {
                        require_value(row, "from", &cell.identity, "SPREAD_EVENT")?;
                        require_value(row, "to", "n_flowe_target", "SPREAD_EVENT")?;
                        require_value(row, "direction", "INCREASING_INTO_FLOWE", "SPREAD_EVENT")?;
                        require_value(row, "operator_bound", "1", "SPREAD_EVENT")?;
                    }
                    "CALLING_GROWTH_U" => {
                        require_value(row, "from", &cell.identity, "SPREAD_EVENT")?;
                        require_value(row, "to", "n_u", "SPREAD_EVENT")?;
                        require_value(row, "direction", "INCREASING_INTO_U", "SPREAD_EVENT")?;
                        require_value(row, "operator_bound", "1", "SPREAD_EVENT")?;
                    }
                    "CALMING_GROWTH_E" => {
                        require_value(row, "from", &cell.identity, "SPREAD_EVENT")?;
                        require_value(row, "to", "n_e", "SPREAD_EVENT")?;
                        require_value(row, "direction", "INCREASING_TOWARD_E", "SPREAD_EVENT")?;
                        require_value(row, "oil_family", &cell.oil_family, "SPREAD_EVENT")?;
                        require_value(row, "oil_amplitude", &amplitude, "SPREAD_EVENT")?;
                    }
                    "CALMING_GROWTH_OUTWARD" => {
                        require_value(row, "from", "n_o0o", "SPREAD_EVENT")?;
                        require_value(row, "to", &cell.identity, "SPREAD_EVENT")?;
                        require_value(row, "direction", "INCREASING_OUTWARD", "SPREAD_EVENT")?;
                        require_value(row, "oil_family", &cell.oil_family, "SPREAD_EVENT")?;
                        require_value(row, "oil_amplitude", &amplitude, "SPREAD_EVENT")?;
                    }
                    "SELF_REDUCTION_GROWTH" => {
                        require_value(row, "from", &cell.identity, "SPREAD_EVENT")?;
                        require_value(row, "to", "n_o0o", "SPREAD_EVENT")?;
                        require_value(row, "direction", "TOWARD_O0O", "SPREAD_EVENT")?;
                        require_value(row, "self_reduction", "1", "SPREAD_EVENT")?;
                        require_value(row, "deletion", "0", "SPREAD_EVENT")?;
                    }
                    "WORD_SPREAD" => {
                        require_value(row, "from", "n_word", "SPREAD_WORD_FLOWE_SOURCE")?;
                        require_value(row, "to", &cell.identity, "SPREAD_EVENT")?;
                        require_value(row, "direction", "OUTWARD", "SPREAD_EVENT")?;
                        require_value(row, "spread", "WORD", "SPREAD_WORD_FLOWE_SOURCE")?;
                        if usize_field(row, "spread_step", "SPREAD_EVENT")? != growth_round {
                            return Err(error("SPREAD_EVENT"));
                        }
                    }
                    "FLOWE_SPREAD" => {
                        require_value(row, "from", "n_flowe_target", "SPREAD_WORD_FLOWE_SOURCE")?;
                        require_value(row, "to", &cell.identity, "SPREAD_EVENT")?;
                        require_value(row, "direction", "OUTWARD", "SPREAD_EVENT")?;
                        require_value(row, "spread", "FLOWE", "SPREAD_WORD_FLOWE_SOURCE")?;
                        if usize_field(row, "spread_step", "SPREAD_EVENT")? != growth_round {
                            return Err(error("SPREAD_EVENT"));
                        }
                    }
                    _ => return Err(error("SPREAD_EVENT")),
                }
                if !ledgers
                    .entry(tag.to_owned())
                    .or_default()
                    .insert((growth_round, q))
                {
                    return Err(error("SPREAD_LEDGER_COUNT"));
                }
            }
            _ => return Err(error("ROW_TAG")),
        }
    }

    if readfirst_count != 1
        || language_count != 1
        || source_count != 1
        || center_count != 1
        || expansion_count != 1
        || parent_invariant_count != 1
        || timing_count != 1
        || boundary_count != 1
        || end_count != 1
        || parents
            != ["N16_CORE", "V1_CORE"]
                .into_iter()
                .map(str::to_owned)
                .collect()
        || rounds != (1..=SPREAD_ROUNDS).collect()
    {
        return Err(error("SPREAD_CONTROL_COUNT"));
    }
    let expected_anchors = [
        ("n_e", "AETHER_E_CENTER"),
        ("n_flowe_target", "OUTWARD_FLOWE_TARGET"),
        ("n_u", "U_CONNECTION"),
        ("n_o0o", "O0O_SPHERE"),
        ("n_word", "WORD_SOURCE"),
    ]
    .into_iter()
    .map(|(key, value)| (key.to_owned(), value.to_owned()))
    .collect();
    if anchors != expected_anchors {
        return Err(error("SPREAD_ANCHOR"));
    }
    if cells.len() != NLEVEL_CELLS || (0..NLEVEL_CELLS).any(|q| !cells.contains_key(&q)) {
        return Err(error("SPREAD_CELL"));
    }
    if ledgers.len() != SPREAD_EVENT_KINDS.len() {
        return Err(error("SPREAD_LEDGER_COUNT"));
    }
    for tag in SPREAD_EVENT_KINDS {
        let ledger = ledgers
            .get(tag)
            .ok_or_else(|| error("SPREAD_LEDGER_COUNT"))?;
        if ledger.len() != SPREAD_ROUNDS * NLEVEL_CELLS
            || (1..=SPREAD_ROUNDS)
                .any(|growth_round| (0..NLEVEL_CELLS).any(|q| !ledger.contains(&(growth_round, q))))
        {
            return Err(error("SPREAD_LEDGER_COUNT"));
        }
    }

    Ok(Validation {
        records: rows.len(),
        glyphs: 0,
        words: 0,
        nodes: NLEVEL_CELLS + anchors.len(),
        calling_joins: 3 * SPREAD_ROUNDS * NLEVEL_CELLS,
        flowe_edges: SPREAD_ROUNDS * NLEVEL_CELLS,
        other_relations: 4 * SPREAD_ROUNDS * NLEVEL_CELLS,
        nlevel: None,
        spread: Some(SpreadValidation {
            rounds: SPREAD_ROUNDS,
            cells: NLEVEL_CELLS,
            ledgers: SPREAD_EVENT_KINDS.len(),
            relation_rows: SPREAD_RELATIONS,
        }),
        hookwall: None,
    })
}

// This validator checks the additive round-3 Hookwall/Brown/Shadow/FLOWesSH
// projection. The immediate R2 parent stays a separately sealed dependency.
fn validate_hookwall_rows(rows: &[Row]) -> Result<Validation> {
    if rows.len() != HOOKWALL_RECORDS {
        return Err(error("HOOKWALL_RECORD_COUNT"));
    }

    let mut anchors = BTreeMap::<String, (String, String)>::new();
    let mut cells = BTreeMap::<usize, NLevelCell>::new();
    let mut ledgers = BTreeMap::<String, BTreeSet<usize>>::new();
    let mut event_ids = BTreeSet::<String>::new();
    let mut source_sha = None::<String>;
    let mut readfirst_count = 0_usize;
    let mut language_count = 0_usize;
    let mut parent_count = 0_usize;
    let mut source_count = 0_usize;
    let mut center_count = 0_usize;
    let mut expansion_count = 0_usize;
    let mut parent_invariant_count = 0_usize;
    let mut round_count = 0_usize;
    let mut parent_anchor_count = 0_usize;
    let mut timing_count = 0_usize;
    let mut boundary_count = 0_usize;
    let mut end_count = 0_usize;
    let last_index = rows.len() - 1;

    for (index, row) in rows.iter().enumerate() {
        require_authority_zero(row)?;
        match row.tag.as_str() {
            "READFIRST" => {
                require_exact_fields(
                    row,
                    &["url", "execution_authority", "json"],
                    "HOOKWALL_CONTROL_FIELDS",
                )?;
                if index != 0 {
                    return Err(error("ROW_ORDER"));
                }
                require_value(row, "url", READFIRST_URL, "HOOKWALL_CONTROL_FIELDS")?;
                readfirst_count += 1;
            }
            "LANGUAGE" => {
                require_exact_fields(
                    row,
                    &[
                        "id",
                        "instance",
                        "tuple_frame",
                        "selector_axes",
                        "execution_authority",
                        "json",
                    ],
                    "HOOKWALL_INSTANCE",
                )?;
                require_value(row, "id", LANGUAGE_ID, "HOOKWALL_INSTANCE")?;
                require_value(row, "instance", HOOKWALL_INSTANCE, "HOOKWALL_INSTANCE")?;
                require_value(
                    row,
                    "tuple_frame",
                    "HYPERBEHCS_60D_PLUS",
                    "HOOKWALL_INSTANCE",
                )?;
                if usize_field(row, "selector_axes", "HOOKWALL_INSTANCE")? != NLEVEL_AXES.len() {
                    return Err(error("HOOKWALL_INSTANCE"));
                }
                language_count += 1;
            }
            "PARENT" => {
                require_exact_fields(
                    row,
                    &[
                        "id",
                        "instance",
                        "path",
                        "sha256",
                        "records",
                        "mutation",
                        "identity_exchange",
                        "execution_authority",
                        "json",
                    ],
                    "HOOKWALL_PARENT",
                )?;
                require_value(row, "id", "WORD_FLOWE_SPREAD_R2", "HOOKWALL_PARENT")?;
                require_value(row, "instance", SPREAD_INSTANCE, "HOOKWALL_PARENT")?;
                require_value(
                    row,
                    "path",
                    "language/word-flowe-spread-r2.flowe",
                    "HOOKWALL_PARENT",
                )?;
                require_value(row, "sha256", SPREAD_CORE_SHA256, "HOOKWALL_PARENT")?;
                if usize_field(row, "records", "HOOKWALL_PARENT")? != SPREAD_RECORDS {
                    return Err(error("HOOKWALL_PARENT"));
                }
                require_value(row, "mutation", "0", "HOOKWALL_PARENT")?;
                require_value(row, "identity_exchange", "0", "HOOKWALL_PARENT")?;
                parent_count += 1;
            }
            "SOURCE" => {
                require_exact_fields(
                    row,
                    &[
                        "path",
                        "occurrences",
                        "sha256",
                        "evidence",
                        "execution_authority",
                        "json",
                    ],
                    "HOOKWALL_SOURCE",
                )?;
                require_value(
                    row,
                    "path",
                    "books/LAW-HOOKWALL-BROWN-SHADOW-FLOWesSH-CONTINUATION.md",
                    "HOOKWALL_SOURCE",
                )?;
                require_value(row, "occurrences", "1", "HOOKWALL_SOURCE")?;
                require_value(row, "evidence", "OPERATOR_CANON", "HOOKWALL_SOURCE")?;
                let digest = field(row, "sha256", "HOOKWALL_SOURCE")?;
                if !valid_sha256(digest) {
                    return Err(error("HOOKWALL_SOURCE"));
                }
                source_sha = Some(digest.to_owned());
                source_count += 1;
            }
            "CENTER" => {
                require_exact_fields(
                    row,
                    &[
                        "members",
                        "traversal_surface",
                        "sh",
                        "identity_exchange",
                        "execution_authority",
                        "json",
                    ],
                    "HOOKWALL_CENTER",
                )?;
                require_value(row, "members", "HBI,HBP,SHA,SH,HASH", "HOOKWALL_CENTER")?;
                require_value(
                    row,
                    "traversal_surface",
                    "HBI,HBP,SHA,SH,HASH",
                    "HOOKWALL_CENTER",
                )?;
                require_value(row, "sh", "OPERATOR_CANON_UNRESOLVED", "HOOKWALL_CENTER")?;
                require_value(row, "identity_exchange", "0", "HOOKWALL_CENTER")?;
                center_count += 1;
            }
            "EXPANSION" => {
                require_exact_fields(
                    row,
                    &[
                        "id",
                        "instance",
                        "law",
                        "law_sha256",
                        "parent",
                        "parent_sha256",
                        "builder",
                        "builder_sha256",
                        "n_open",
                        "compiled_round",
                        "compiled_round_count",
                        "cells",
                        "event_kinds",
                        "relation_rows",
                        "selector_axes",
                        "execution_authority",
                        "json",
                    ],
                    "HOOKWALL_INSTANCE",
                )?;
                require_value(
                    row,
                    "id",
                    "hookwall_brown_shadow_flowessh_v4",
                    "HOOKWALL_INSTANCE",
                )?;
                require_value(row, "instance", HOOKWALL_INSTANCE, "HOOKWALL_INSTANCE")?;
                require_value(
                    row,
                    "law",
                    "books/LAW-HOOKWALL-BROWN-SHADOW-FLOWesSH-CONTINUATION.md",
                    "HOOKWALL_INSTANCE",
                )?;
                if source_sha.as_deref() != Some(field(row, "law_sha256", "HOOKWALL_INSTANCE")?) {
                    return Err(error("HOOKWALL_INSTANCE"));
                }
                require_value(
                    row,
                    "parent",
                    "language/word-flowe-spread-r2.flowe",
                    "HOOKWALL_INSTANCE",
                )?;
                require_value(
                    row,
                    "parent_sha256",
                    SPREAD_CORE_SHA256,
                    "HOOKWALL_INSTANCE",
                )?;
                require_value(
                    row,
                    "builder",
                    "tools/build_hookwall_shadow_flowessh.py",
                    "HOOKWALL_INSTANCE",
                )?;
                if !valid_sha256(field(row, "builder_sha256", "HOOKWALL_INSTANCE")?) {
                    return Err(error("HOOKWALL_INSTANCE"));
                }
                require_value(row, "n_open", "1", "HOOKWALL_INSTANCE")?;
                if usize_field(row, "compiled_round", "HOOKWALL_INSTANCE")? != HOOKWALL_ROUND
                    || usize_field(row, "compiled_round_count", "HOOKWALL_INSTANCE")? != 1
                    || usize_field(row, "cells", "HOOKWALL_INSTANCE")? != NLEVEL_CELLS
                    || usize_field(row, "event_kinds", "HOOKWALL_INSTANCE")?
                        != HOOKWALL_EVENT_KINDS.len()
                    || usize_field(row, "relation_rows", "HOOKWALL_INSTANCE")? != HOOKWALL_RELATIONS
                    || usize_field(row, "selector_axes", "HOOKWALL_INSTANCE")? != NLEVEL_AXES.len()
                {
                    return Err(error("HOOKWALL_INSTANCE"));
                }
                expansion_count += 1;
            }
            "PARENT_INVARIANT" => {
                require_exact_fields(
                    row,
                    &[
                        "id",
                        "parent",
                        "compiled_rounds",
                        "cells",
                        "event_kinds",
                        "relation_rows",
                        "identity_exchange",
                        "deletion",
                        "validated",
                        "execution_authority",
                        "json",
                    ],
                    "HOOKWALL_PARENT_INVARIANT",
                )?;
                require_value(
                    row,
                    "id",
                    "WORD_FLOWE_SPREAD_V3_TYPED_GROWTH",
                    "HOOKWALL_PARENT_INVARIANT",
                )?;
                require_value(row, "parent", SPREAD_INSTANCE, "HOOKWALL_PARENT_INVARIANT")?;
                if usize_field(row, "compiled_rounds", "HOOKWALL_PARENT_INVARIANT")?
                    != SPREAD_ROUNDS
                    || usize_field(row, "cells", "HOOKWALL_PARENT_INVARIANT")? != NLEVEL_CELLS
                    || usize_field(row, "event_kinds", "HOOKWALL_PARENT_INVARIANT")?
                        != SPREAD_EVENT_KINDS.len()
                    || usize_field(row, "relation_rows", "HOOKWALL_PARENT_INVARIANT")?
                        != SPREAD_RELATIONS
                {
                    return Err(error("HOOKWALL_PARENT_INVARIANT"));
                }
                require_value(row, "identity_exchange", "0", "HOOKWALL_PARENT_INVARIANT")?;
                require_value(row, "deletion", "0", "HOOKWALL_PARENT_INVARIANT")?;
                require_value(row, "validated", "1", "HOOKWALL_PARENT_INVARIANT")?;
                parent_invariant_count += 1;
            }
            "ROUND" => {
                require_exact_fields(
                    row,
                    &[
                        "id",
                        "ordinal",
                        "previous_instance",
                        "previous",
                        "increase_q",
                        "n_open",
                        "compiled_projection",
                        "execution_authority",
                        "json",
                    ],
                    "HOOKWALL_ROUND",
                )?;
                require_value(row, "id", "round_03", "HOOKWALL_ROUND")?;
                if usize_field(row, "ordinal", "HOOKWALL_ROUND")? != HOOKWALL_ROUND
                    || usize_field(row, "increase_q", "HOOKWALL_ROUND")? != HOOKWALL_ROUND
                {
                    return Err(error("HOOKWALL_ROUND"));
                }
                require_value(row, "previous_instance", SPREAD_INSTANCE, "HOOKWALL_ROUND")?;
                require_value(row, "previous", "round_02", "HOOKWALL_ROUND")?;
                require_value(row, "n_open", "1", "HOOKWALL_ROUND")?;
                require_value(row, "compiled_projection", "1", "HOOKWALL_ROUND")?;
                round_count += 1;
            }
            "ANCHOR" => {
                require_exact_fields(
                    row,
                    &[
                        "id",
                        "token",
                        "meaning",
                        "exact",
                        "execution_authority",
                        "json",
                    ],
                    "HOOKWALL_ANCHOR",
                )?;
                let identity = field(row, "id", "HOOKWALL_ANCHOR")?.to_owned();
                let value = (
                    field(row, "token", "HOOKWALL_ANCHOR")?.to_owned(),
                    field(row, "meaning", "HOOKWALL_ANCHOR")?.to_owned(),
                );
                require_value(row, "exact", "1", "HOOKWALL_ANCHOR")?;
                if anchors.insert(identity, value).is_some() {
                    return Err(error("HOOKWALL_ANCHOR"));
                }
            }
            "PARENT_ANCHOR_REF" => {
                require_exact_fields(
                    row,
                    &[
                        "id",
                        "parent_instance",
                        "parent_id",
                        "reference_only",
                        "identity_exchange",
                        "execution_authority",
                        "json",
                    ],
                    "HOOKWALL_PARENT_ANCHOR",
                )?;
                require_value(row, "id", "n_o0o", "HOOKWALL_PARENT_ANCHOR")?;
                require_value(
                    row,
                    "parent_instance",
                    SPREAD_INSTANCE,
                    "HOOKWALL_PARENT_ANCHOR",
                )?;
                require_value(row, "parent_id", "n_o0o", "HOOKWALL_PARENT_ANCHOR")?;
                require_value(row, "reference_only", "1", "HOOKWALL_PARENT_ANCHOR")?;
                require_value(row, "identity_exchange", "0", "HOOKWALL_PARENT_ANCHOR")?;
                parent_anchor_count += 1;
            }
            "CELL_REF" => {
                require_exact_fields(
                    row,
                    &[
                        "id",
                        "parent_instance",
                        "parent_id",
                        "reference_only",
                        "level",
                        "book",
                        "book_ordinal",
                        "q",
                        "space_radius",
                        "x",
                        "y",
                        "z",
                        "time",
                        "colour",
                        "oil_family",
                        "identity_exchange",
                        "execution_authority",
                        "json",
                    ],
                    "HOOKWALL_CELL",
                )?;
                let q = usize_field(row, "q", "HOOKWALL_CELL")?;
                let n = usize_field(row, "level", "HOOKWALL_CELL")?;
                let book_ordinal = usize_field(row, "book_ordinal", "HOOKWALL_CELL")?;
                let identity = format!("cell_n{n:02}_b{book_ordinal:02}");
                if n >= NLEVEL_LEVELS
                    || book_ordinal >= NLEVEL_BOOKS
                    || q != n * NLEVEL_BOOKS + book_ordinal
                    || field(row, "id", "HOOKWALL_CELL")? != identity
                    || field(row, "parent_id", "HOOKWALL_CELL")? != identity
                {
                    return Err(error("HOOKWALL_CELL"));
                }
                require_value(row, "parent_instance", SPREAD_INSTANCE, "HOOKWALL_CELL")?;
                require_value(row, "reference_only", "1", "HOOKWALL_CELL")?;
                require_value(row, "book", NLEVEL_BOOK_IDS[book_ordinal], "HOOKWALL_CELL")?;
                let n_i64 = i64::try_from(n).map_err(|_| error("HOOKWALL_CELL"))?;
                let book_i64 = i64::try_from(book_ordinal).map_err(|_| error("HOOKWALL_CELL"))?;
                let expected_x = book_i64 * 2 - 9;
                let expected_y = n_i64 * 2 - 15;
                let expected_z =
                    i64::try_from((n + book_ordinal) % 3).map_err(|_| error("HOOKWALL_CELL"))? - 1;
                if i64_field(row, "x", "HOOKWALL_CELL")? != expected_x
                    || i64_field(row, "y", "HOOKWALL_CELL")? != expected_y
                    || i64_field(row, "z", "HOOKWALL_CELL")? != expected_z
                    || field(row, "time", "HOOKWALL_CELL")?
                        != NLEVEL_TIME_NAMES[q % NLEVEL_TIME_NAMES.len()]
                    || field(row, "colour", "HOOKWALL_CELL")?
                        != NLEVEL_COLOURS[(n + book_ordinal) % NLEVEL_COLOURS.len()]
                    || usize_field(row, "space_radius", "HOOKWALL_CELL")? != n + 1
                {
                    return Err(error("HOOKWALL_CELL"));
                }
                let oil_family =
                    NLEVEL_OIL_FAMILIES[(n + 2 * book_ordinal) % NLEVEL_OIL_FAMILIES.len()];
                require_value(row, "oil_family", oil_family, "HOOKWALL_CELL")?;
                require_value(row, "identity_exchange", "0", "HOOKWALL_CELL")?;
                let cell = NLevelCell {
                    identity,
                    book: NLEVEL_BOOK_IDS[book_ordinal].to_owned(),
                    n,
                    book_ordinal,
                    q,
                    translucence_q8: 0,
                    oil_family: oil_family.to_owned(),
                    space_radius: n + 1,
                };
                if cells.insert(q, cell).is_some() {
                    return Err(error("HOOKWALL_CELL"));
                }
            }
            "TIMING_BOUNDARY" => {
                require_exact_fields(
                    row,
                    &[
                        "instant_address",
                        "elapsed_measurement_present",
                        "runtime_measurement_present",
                        "execution_authority",
                        "json",
                    ],
                    "HOOKWALL_TIMING",
                )?;
                require_value(row, "instant_address", "1", "HOOKWALL_TIMING")?;
                require_value(row, "elapsed_measurement_present", "0", "HOOKWALL_TIMING")?;
                require_value(row, "runtime_measurement_present", "0", "HOOKWALL_TIMING")?;
                timing_count += 1;
            }
            "BOUNDARY" => {
                require_exact_fields(
                    row,
                    &[
                        "physical_mapping",
                        "live_runtime_mapping",
                        "system_affirmed",
                        "round_03_compilation",
                        "hookwall_runtime_enforcement",
                        "claim_back_external_evidence",
                        "execution_authority",
                        "json",
                    ],
                    "HOOKWALL_BOUNDARY",
                )?;
                require_value(row, "physical_mapping", "UNVERIFIED", "HOOKWALL_BOUNDARY")?;
                require_value(
                    row,
                    "live_runtime_mapping",
                    "UNVERIFIED",
                    "HOOKWALL_BOUNDARY",
                )?;
                require_value(row, "system_affirmed", "0", "HOOKWALL_BOUNDARY")?;
                require_value(row, "round_03_compilation", "DESIGN", "HOOKWALL_BOUNDARY")?;
                require_value(
                    row,
                    "hookwall_runtime_enforcement",
                    "0",
                    "HOOKWALL_BOUNDARY",
                )?;
                require_value(
                    row,
                    "claim_back_external_evidence",
                    "0",
                    "HOOKWALL_BOUNDARY",
                )?;
                boundary_count += 1;
            }
            "END" => {
                require_exact_fields(
                    row,
                    &[
                        "status",
                        "n_open",
                        "compiled_round",
                        "compiled_round_count",
                        "cells",
                        "event_kinds",
                        "relation_rows",
                        "execution_authority",
                        "json",
                    ],
                    "HOOKWALL_END",
                )?;
                if index != last_index {
                    return Err(error("ROW_ORDER"));
                }
                require_value(
                    row,
                    "status",
                    "COMPILED_BOUNDED_HOOKWALL_SHADOW_PROJECTION",
                    "HOOKWALL_END",
                )?;
                require_value(row, "n_open", "1", "HOOKWALL_END")?;
                if usize_field(row, "compiled_round", "HOOKWALL_END")? != HOOKWALL_ROUND
                    || usize_field(row, "compiled_round_count", "HOOKWALL_END")? != 1
                    || usize_field(row, "cells", "HOOKWALL_END")? != NLEVEL_CELLS
                    || usize_field(row, "event_kinds", "HOOKWALL_END")?
                        != HOOKWALL_EVENT_KINDS.len()
                    || usize_field(row, "relation_rows", "HOOKWALL_END")? != HOOKWALL_RELATIONS
                {
                    return Err(error("HOOKWALL_END"));
                }
                end_count += 1;
            }
            tag if HOOKWALL_EVENT_KINDS.contains(&tag) => {
                let specific: &[&str] = match tag {
                    "CALLING_CHAIN_MORE" => &["calling", "chain"],
                    "HOOKWALL_REVIEW" => &["review", "claim_verdict"],
                    "CLAIM_BACK_WHITE" => &["claim", "evidence_verdict"],
                    "WHITE_TO_WASDTE"
                    | "WASDTE_TO_BE"
                    | "BE_TO_NOT_WHITE"
                    | "NOT_WHITE_TO_B"
                    | "B_TO_BROWNS"
                    | "BROWNS_TO_BETWEENS"
                    | "BETWEENS_TO_AROUNDS" => &["grammar"],
                    "SHADOW_GUIDING" => &["guidance", "guided_cell_retained"],
                    "CALMING_OIL_CONTINUE" => &["oil_family", "oil_amplitude", "calming"],
                    "CALLING_OUTWAR" => &["token", "operator_bound"],
                    "REDUCTONS" => &["token", "semantics", "deletion"],
                    "FLOWESSH" => &["token", "composition"],
                    _ => return Err(error("HOOKWALL_EVENT_FIELDS")),
                };
                require_exact_fields_union(
                    row,
                    &HOOKWALL_EVENT_COMMON_FIELDS,
                    specific,
                    "HOOKWALL_EVENT_FIELDS",
                )?;
                let q = usize_field(row, "q", "HOOKWALL_EVENT")?;
                let cell = cells.get(&q).ok_or_else(|| error("HOOKWALL_EVENT"))?;
                if usize_field(row, "round", "HOOKWALL_EVENT")? != HOOKWALL_ROUND
                    || usize_field(row, "increase_q", "HOOKWALL_EVENT")? != HOOKWALL_ROUND
                    || usize_field(row, "growth_q", "HOOKWALL_EVENT")?
                        != cell.space_radius + HOOKWALL_ROUND
                    || field(row, "cell", "HOOKWALL_EVENT")? != cell.identity
                    || usize_field(row, "level", "HOOKWALL_EVENT")? != cell.n
                    || field(row, "book", "HOOKWALL_EVENT")? != cell.book
                {
                    return Err(error("HOOKWALL_EVENT"));
                }
                let expected_id = format!(
                    "{}_r{HOOKWALL_ROUND:02}_n{:02}_b{:02}",
                    tag.to_ascii_lowercase(),
                    cell.n,
                    cell.book_ordinal
                );
                if field(row, "id", "HOOKWALL_EVENT")? != expected_id
                    || !event_ids.insert(expected_id)
                {
                    return Err(error("HOOKWALL_EVENT"));
                }
                require_value(row, "n_open", "1", "HOOKWALL_EVENT")?;
                require_value(row, "instant_address", "1", "HOOKWALL_TIMING")?;
                require_value(row, "elapsed_measurement_present", "0", "HOOKWALL_TIMING")?;
                require_value(row, "runtime_measurement_present", "0", "HOOKWALL_TIMING")?;
                require_value(row, "identity_exchange", "0", "HOOKWALL_EVENT")?;
                require_value(row, "source_retained", "1", "HOOKWALL_EVENT")?;

                match tag {
                    "CALLING_CHAIN_MORE" => {
                        let next_q = (q + 1) % NLEVEL_CELLS;
                        let next_identity = format!(
                            "cell_n{:02}_b{:02}",
                            next_q / NLEVEL_BOOKS,
                            next_q % NLEVEL_BOOKS
                        );
                        require_value(row, "from", &cell.identity, "HOOKWALL_EVENT")?;
                        require_value(row, "to", &next_identity, "HOOKWALL_EVENT")?;
                        require_value(row, "direction", "CHAIN_MORE", "HOOKWALL_EVENT")?;
                        require_value(row, "calling", "n_callings", "HOOKWALL_EVENT")?;
                        require_value(row, "chain", "n_chains", "HOOKWALL_EVENT")?;
                    }
                    "HOOKWALL_REVIEW" => {
                        require_value(row, "from", &cell.identity, "HOOKWALL_EVENT")?;
                        require_value(row, "to", "n_hookwalls", "HOOKWALL_EVENT")?;
                        require_value(row, "direction", "TO_HOOKWALL_REVIEW", "HOOKWALL_EVENT")?;
                        require_value(row, "review", "DECLARATIVE_PROVENANCE", "HOOKWALL_EVENT")?;
                        require_value(row, "claim_verdict", "UNRESOLVED", "HOOKWALL_EVENT")?;
                    }
                    "CLAIM_BACK_WHITE" => {
                        require_value(row, "from", "n_claims", "HOOKWALL_EVENT")?;
                        require_value(row, "to", "n_white", "HOOKWALL_EVENT")?;
                        require_value(row, "direction", "BACK_TO_WHITE", "HOOKWALL_EVENT")?;
                        require_value(row, "claim", "claims", "HOOKWALL_EVENT")?;
                        require_value(row, "evidence_verdict", "0", "HOOKWALL_EVENT")?;
                    }
                    "WHITE_TO_WASDTE" => {
                        require_value(row, "from", "n_white", "HOOKWALL_CHAIN")?;
                        require_value(row, "to", "n_wasdte", "HOOKWALL_CHAIN")?;
                        require_value(row, "direction", "TO_WASDTE", "HOOKWALL_CHAIN")?;
                        require_value(row, "grammar", "white_is_wasdte", "HOOKWALL_CHAIN")?;
                    }
                    "WASDTE_TO_BE" => {
                        require_value(row, "from", "n_wasdte", "HOOKWALL_CHAIN")?;
                        require_value(row, "to", "n_be", "HOOKWALL_CHAIN")?;
                        require_value(row, "direction", "TO_BE", "HOOKWALL_CHAIN")?;
                        require_value(row, "grammar", "wasdte_be", "HOOKWALL_CHAIN")?;
                    }
                    "BE_TO_NOT_WHITE" => {
                        require_value(row, "from", "n_be", "HOOKWALL_CHAIN")?;
                        require_value(row, "to", "n_not_white", "HOOKWALL_CHAIN")?;
                        require_value(row, "direction", "TO_NOT_WHITE", "HOOKWALL_CHAIN")?;
                        require_value(row, "grammar", "be_not_white", "HOOKWALL_CHAIN")?;
                    }
                    "NOT_WHITE_TO_B" => {
                        require_value(row, "from", "n_not_white", "HOOKWALL_CHAIN")?;
                        require_value(row, "to", "n_B", "HOOKWALL_CHAIN")?;
                        require_value(row, "direction", "TO_B", "HOOKWALL_CHAIN")?;
                        require_value(row, "grammar", "not_white_B", "HOOKWALL_CHAIN")?;
                    }
                    "B_TO_BROWNS" => {
                        require_value(row, "from", "n_B", "HOOKWALL_CHAIN")?;
                        require_value(row, "to", "n_browns", "HOOKWALL_CHAIN")?;
                        require_value(row, "direction", "TO_BROWNS", "HOOKWALL_CHAIN")?;
                        require_value(row, "grammar", "B_browns", "HOOKWALL_CHAIN")?;
                    }
                    "BROWNS_TO_BETWEENS" => {
                        require_value(row, "from", "n_browns", "HOOKWALL_CHAIN")?;
                        require_value(row, "to", "n_betweens", "HOOKWALL_CHAIN")?;
                        require_value(row, "direction", "TO_BETWEENS", "HOOKWALL_CHAIN")?;
                        require_value(row, "grammar", "browns_betweens", "HOOKWALL_CHAIN")?;
                    }
                    "BETWEENS_TO_AROUNDS" => {
                        require_value(row, "from", "n_betweens", "HOOKWALL_CHAIN")?;
                        require_value(row, "to", "n_arounds", "HOOKWALL_CHAIN")?;
                        require_value(row, "direction", "TO_AROUNDS", "HOOKWALL_CHAIN")?;
                        require_value(row, "grammar", "betweens_arounds", "HOOKWALL_CHAIN")?;
                    }
                    "SHADOW_GUIDING" => {
                        require_value(row, "from", "n_shadows", "HOOKWALL_EVENT")?;
                        require_value(row, "to", &cell.identity, "HOOKWALL_EVENT")?;
                        require_value(row, "direction", "GUIDING", "HOOKWALL_EVENT")?;
                        require_value(row, "guidance", "SHADOWS", "HOOKWALL_EVENT")?;
                        require_value(row, "guided_cell_retained", "1", "HOOKWALL_EVENT")?;
                    }
                    "CALMING_OIL_CONTINUE" => {
                        require_value(row, "from", "n_o0o", "HOOKWALL_EVENT")?;
                        require_value(row, "to", &cell.identity, "HOOKWALL_EVENT")?;
                        require_value(row, "direction", "CONTINUE_CALMING", "HOOKWALL_EVENT")?;
                        require_value(row, "oil_family", &cell.oil_family, "HOOKWALL_EVENT")?;
                        require_value(
                            row,
                            "oil_amplitude",
                            &(cell.space_radius + HOOKWALL_ROUND).to_string(),
                            "HOOKWALL_EVENT",
                        )?;
                        require_value(row, "calming", "1", "HOOKWALL_EVENT")?;
                    }
                    "CALLING_OUTWAR" => {
                        require_value(row, "from", &cell.identity, "HOOKWALL_EVENT")?;
                        require_value(row, "to", "n_outwar", "HOOKWALL_EVENT")?;
                        require_value(row, "direction", "CALLING_OUTWAR", "HOOKWALL_EVENT")?;
                        require_value(row, "token", "outwar", "HOOKWALL_EVENT")?;
                        require_value(row, "operator_bound", "1", "HOOKWALL_EVENT")?;
                    }
                    "REDUCTONS" => {
                        require_value(row, "from", &cell.identity, "HOOKWALL_EVENT")?;
                        require_value(row, "to", "n_o0o", "HOOKWALL_EVENT")?;
                        require_value(row, "direction", "REDUCTONS_TO_O0O", "HOOKWALL_EVENT")?;
                        require_value(row, "token", "reductons", "HOOKWALL_EVENT")?;
                        require_value(
                            row,
                            "semantics",
                            "OPERATOR_CANON_UNRESOLVED",
                            "HOOKWALL_EVENT",
                        )?;
                        require_value(row, "deletion", "0", "HOOKWALL_EVENT")?;
                    }
                    "FLOWESSH" => {
                        require_value(row, "from", "n_FLOWesSH", "HOOKWALL_FLOWESSH")?;
                        require_value(row, "to", &cell.identity, "HOOKWALL_FLOWESSH")?;
                        require_value(row, "direction", "FLOWESSH_TO_CELL", "HOOKWALL_FLOWESSH")?;
                        require_value(row, "token", "FLOWesSH", "HOOKWALL_FLOWESSH")?;
                        require_value(
                            row,
                            "composition",
                            "OPERATOR_CANON_UNRESOLVED",
                            "HOOKWALL_FLOWESSH",
                        )?;
                    }
                    _ => return Err(error("HOOKWALL_EVENT")),
                }
                if !ledgers.entry(tag.to_owned()).or_default().insert(q) {
                    return Err(error("HOOKWALL_LEDGER_COUNT"));
                }
            }
            _ => return Err(error("ROW_TAG")),
        }
    }

    if readfirst_count != 1
        || language_count != 1
        || parent_count != 1
        || source_count != 1
        || center_count != 1
        || expansion_count != 1
        || parent_invariant_count != 1
        || round_count != 1
        || parent_anchor_count != 1
        || timing_count != 1
        || boundary_count != 1
        || end_count != 1
    {
        return Err(error("HOOKWALL_CONTROL_COUNT"));
    }
    let expected_anchors = HOOKWALL_ANCHORS
        .into_iter()
        .map(|(identity, token, meaning)| {
            (identity.to_owned(), (token.to_owned(), meaning.to_owned()))
        })
        .collect();
    if anchors != expected_anchors {
        return Err(error("HOOKWALL_ANCHOR"));
    }
    if cells.len() != NLEVEL_CELLS || (0..NLEVEL_CELLS).any(|q| !cells.contains_key(&q)) {
        return Err(error("HOOKWALL_CELL"));
    }
    if ledgers.len() != HOOKWALL_EVENT_KINDS.len() {
        return Err(error("HOOKWALL_LEDGER_COUNT"));
    }
    for tag in HOOKWALL_EVENT_KINDS {
        let ledger = ledgers
            .get(tag)
            .ok_or_else(|| error("HOOKWALL_LEDGER_COUNT"))?;
        if ledger.len() != NLEVEL_CELLS || (0..NLEVEL_CELLS).any(|q| !ledger.contains(&q)) {
            return Err(error("HOOKWALL_LEDGER_COUNT"));
        }
    }

    Ok(Validation {
        records: rows.len(),
        glyphs: 0,
        words: 0,
        nodes: NLEVEL_CELLS + HOOKWALL_ANCHORS.len() + 1,
        calling_joins: 2 * NLEVEL_CELLS,
        flowe_edges: NLEVEL_CELLS,
        other_relations: 12 * NLEVEL_CELLS,
        nlevel: None,
        spread: None,
        hookwall: Some(HookwallValidation {
            round: HOOKWALL_ROUND,
            cells: NLEVEL_CELLS,
            ledgers: HOOKWALL_EVENT_KINDS.len(),
            relation_rows: HOOKWALL_RELATIONS,
        }),
    })
}

fn validate_bytes(bytes: &[u8]) -> Result<Validation> {
    let text = canonical_text(bytes)?;
    let rows: Vec<Row> = text.lines().map(parse_row).collect::<Result<_>>()?;
    if rows.first().map(|row| row.tag.as_str()) != Some("READFIRST")
        || rows.last().map(|row| row.tag.as_str()) != Some("END")
    {
        return Err(error("ROW_ORDER"));
    }
    if is_hookwall_rows(&rows) {
        return validate_hookwall_rows(&rows);
    }
    if is_spread_rows(&rows) {
        return validate_spread_rows(&rows);
    }
    if is_nlevel_rows(&rows) {
        return validate_nlevel_rows(&rows);
    }

    let mut identities = BTreeSet::new();
    let mut glyphs = BTreeMap::<String, String>::new();
    let mut words = BTreeMap::<String, Word>::new();
    let mut tokens = BTreeSet::<String>::new();
    let mut nodes = BTreeMap::<String, Node>::new();
    let mut coordinates = BTreeSet::<(i64, i64, i64)>::new();
    let mut calling_edges = Vec::<Edge>::new();
    let mut flowe_edges = Vec::<Edge>::new();
    let mut other_edges = Vec::<Edge>::new();
    let mut flowe_steps = BTreeSet::<u64>::new();
    let mut readfirst_count = 0_usize;
    let mut language_count = 0_usize;
    let mut source_count = 0_usize;
    let mut center_count = 0_usize;
    let last_index = rows.len() - 1;

    for (index, row) in rows.iter().enumerate() {
        require_authority_zero(row)?;
        match row.tag.as_str() {
            "READFIRST" => {
                if index != 0 || field(row, "url", "READFIRST_FIELD")? != READFIRST_URL {
                    return Err(error("ROW_ORDER"));
                }
                readfirst_count += 1;
            }
            "LANGUAGE" => {
                language_count += 1;
                let identity = parse_identity(row, "id")?;
                if identity != LANGUAGE_ID {
                    return Err(error("LANGUAGE_ID"));
                }
                insert_identity(&mut identities, identity)?;
                if field(row, "coordinate_type", "LANGUAGE_FIELD")? != "SIGNED_INTEGER" {
                    return Err(error("INTEGER_COORDINATE_REQUIRED"));
                }
            }
            "SOURCE" => {
                source_count += 1;
                insert_identity(&mut identities, parse_identity(row, "id")?)?;
                if field(row, "path", "SOURCE_FIELD")? != SOURCE_PATH
                    || field(row, "occurrences", "SOURCE_FIELD")? != "1"
                    || !valid_sha256(field(row, "sha256", "SOURCE_FIELD")?)
                {
                    return Err(error("SOURCE_OCCURRENCE_COUNT"));
                }
            }
            "CENTER" => {
                center_count += 1;
                if field(row, "members", "CENTER_FIELD")? != "HBI,HBP,SHA,SH,HASH"
                    || field(row, "traversal_surface", "CENTER_FIELD")? != "HBI,HBP,SHA,SH,HASH"
                {
                    return Err(error("CENTER_FIELD"));
                }
            }
            "GITHUB_MATRIX" | "BOUNDARY" => {}
            "SURFACE" | "OUTWARD" => {
                insert_identity(&mut identities, parse_identity(row, "id")?)?;
            }
            "GLYPH" => {
                require_fields(row, &["id", "surface", "meaning"], "GLYPH_FIELD")?;
                let identity = parse_identity(row, "id")?;
                insert_identity(&mut identities, identity.clone())?;
                glyphs.insert(identity, field(row, "meaning", "GLYPH_FIELD")?.to_owned());
            }
            "WORD" => {
                require_fields(row, &["id", "glyphs", "meaning"], "WORD_FIELD")?;
                let identity = parse_identity(row, "id")?;
                insert_identity(&mut identities, identity.clone())?;
                let components: Vec<String> = field(row, "glyphs", "WORD_FIELD")?
                    .split(',')
                    .map(str::to_owned)
                    .collect();
                if components.is_empty() || components.iter().any(|value| !valid_identity(value)) {
                    return Err(error("WORD_FIELD"));
                }
                words.insert(identity, Word { glyphs: components });
            }
            "TOKEN" => {
                let identity = parse_identity(row, "id")?;
                insert_identity(&mut identities, identity.clone())?;
                tokens.insert(identity);
            }
            "NODE" => {
                require_fields(row, &["id", "ref", "x", "y", "z", "color"], "NODE_FIELD")?;
                let identity = parse_identity(row, "id")?;
                insert_identity(&mut identities, identity.clone())?;
                let coordinate = (
                    parse_i64(field(row, "x", "NODE_FIELD")?)?,
                    parse_i64(field(row, "y", "NODE_FIELD")?)?,
                    parse_i64(field(row, "z", "NODE_FIELD")?)?,
                );
                if !coordinates.insert(coordinate) {
                    return Err(error("DUPLICATE_COORDINATE"));
                }
                nodes.insert(
                    identity,
                    Node {
                        reference: parse_identity(row, "ref")?,
                    },
                );
            }
            "CALLING_JOIN" => {
                if row.fields.contains_key("step") {
                    return Err(error("CALLING_FIELD_MISMATCH"));
                }
                insert_identity(&mut identities, parse_identity(row, "id")?)?;
                if field(row, "direction", "CALLING_FIELD_MISMATCH")? != "UNRESOLVED"
                    || field(row, "endpoints_retained", "CALLING_FIELD_MISMATCH")? != "1"
                {
                    return Err(error("CALLING_FIELD_MISMATCH"));
                }
                calling_edges.push(Edge {
                    from: parse_identity(row, "from")?,
                    to: parse_identity(row, "to")?,
                });
            }
            "FLOWe" => {
                insert_identity(&mut identities, parse_identity(row, "id")?)?;
                if field(row, "direction", "FLOWE_FIELD")? != "FORWARD" {
                    return Err(error("FLOWE_DIRECTION_UNRESOLVED"));
                }
                let step = parse_u64(field(row, "step", "FLOWE_FIELD")?)?;
                if !flowe_steps.insert(step) {
                    return Err(error("FLOWE_STEP"));
                }
                flowe_edges.push(Edge {
                    from: parse_identity(row, "from")?,
                    to: parse_identity(row, "to")?,
                });
            }
            "CALMING" | "BRIDGE" | "RADIUS_CONNECTION" | "INVERSION" => {
                insert_identity(&mut identities, parse_identity(row, "id")?)?;
                other_edges.push(Edge {
                    from: parse_identity(row, "from")?,
                    to: parse_identity(row, "to")?,
                });
            }
            "END" => {
                if index != last_index {
                    return Err(error("ROW_ORDER"));
                }
            }
            _ => return Err(error("ROW_TAG")),
        }
    }

    if source_count != 1 {
        return Err(error("SOURCE_OCCURRENCE_COUNT"));
    }
    if readfirst_count != 1 || language_count != 1 || center_count != 1 {
        return Err(error("REQUIRED_ROW_COUNT"));
    }
    if glyphs.get("V").map(String::as_str) != Some("VELOCITY") {
        return Err(error("V_VELOCITY_REQUIRED"));
    }
    for word in words.values() {
        if word.glyphs.iter().any(|glyph| !glyphs.contains_key(glyph)) {
            return Err(error("GLYPH_UNRESOLVED"));
        }
    }
    let vector = words
        .get("Vector")
        .ok_or_else(|| error("VECTOR_COMPOSITION_MISMATCH"))?;
    if vector
        .glyphs
        .iter()
        .map(String::as_str)
        .ne(["V", "e", "c", "t", "o", "r"])
    {
        return Err(error("VECTOR_COMPOSITION_MISMATCH"));
    }
    for node in nodes.values() {
        if !glyphs.contains_key(&node.reference)
            && !words.contains_key(&node.reference)
            && !tokens.contains(&node.reference)
        {
            return Err(error("NODE_REFERENCE_UNRESOLVED"));
        }
    }
    if calling_edges.is_empty() || flowe_edges.is_empty() || other_edges.is_empty() {
        return Err(error("EDGE_KIND_REQUIRED"));
    }
    for edge in calling_edges.iter().chain(&flowe_edges).chain(&other_edges) {
        if !nodes.contains_key(&edge.from) || !nodes.contains_key(&edge.to) {
            return Err(error("EDGE_ENDPOINT_UNRESOLVED"));
        }
    }
    if flowe_steps
        .iter()
        .copied()
        .ne(0_u64..u64::try_from(flowe_steps.len()).map_err(|_| error("FLOWE_STEP"))?)
    {
        return Err(error("FLOWE_STEP"));
    }

    Ok(Validation {
        records: rows.len(),
        glyphs: glyphs.len(),
        words: words.len(),
        nodes: nodes.len(),
        calling_joins: calling_edges.len(),
        flowe_edges: flowe_edges.len(),
        other_relations: other_edges.len(),
        nlevel: None,
        spread: None,
        hookwall: None,
    })
}

fn run() -> Result<Validation> {
    let mut arguments = env::args_os();
    let _program = arguments.next();
    let input = PathBuf::from(arguments.next().ok_or_else(|| error("USAGE"))?);
    if arguments.next().is_some() {
        return Err(error("USAGE"));
    }
    let bytes = fs::read(input).map_err(|_| error("INPUT_READ"))?;
    validate_bytes(&bytes)
}

fn main() -> ExitCode {
    match run() {
        Ok(result) => {
            if let Some(hookwall) = result.hookwall {
                println!(
                    "FLOWE_VALIDATE|PASS=1|language={LANGUAGE_ID}|instance={HOOKWALL_INSTANCE}|records={}|round={}|cells={}|ledgers={}|relation_rows={}|nodes={}|calling_joins={}|flowe_edges={}|other_relations={}|validation_scope={VALIDATION_SCOPE}|referenced_file_bytes_bound=0|execution_authority=0|json=0",
                    result.records,
                    hookwall.round,
                    hookwall.cells,
                    hookwall.ledgers,
                    hookwall.relation_rows,
                    result.nodes,
                    result.calling_joins,
                    result.flowe_edges,
                    result.other_relations
                );
            } else if let Some(spread) = result.spread {
                println!(
                    "FLOWE_VALIDATE|PASS=1|language={LANGUAGE_ID}|instance={SPREAD_INSTANCE}|records={}|rounds={}|cells={}|ledgers={}|relation_rows={}|nodes={}|calling_joins={}|flowe_edges={}|other_relations={}|validation_scope={VALIDATION_SCOPE}|referenced_file_bytes_bound=0|execution_authority=0|json=0",
                    result.records,
                    spread.rounds,
                    spread.cells,
                    spread.ledgers,
                    spread.relation_rows,
                    result.nodes,
                    result.calling_joins,
                    result.flowe_edges,
                    result.other_relations
                );
            } else if let Some(nlevel) = result.nlevel {
                println!(
                    "FLOWE_VALIDATE|PASS=1|language={LANGUAGE_ID}|instance={NLEVEL_INSTANCE}|records={}|axes={}|books={}|levels={}|cells={}|ledgers={}|ledger_rows={}|nodes={}|calling_joins={}|flowe_edges={}|other_relations={}|validation_scope={VALIDATION_SCOPE}|referenced_file_bytes_bound=0|execution_authority=0|json=0",
                    result.records,
                    nlevel.axes,
                    nlevel.books,
                    nlevel.levels,
                    nlevel.cells,
                    nlevel.ledgers,
                    nlevel.ledger_rows,
                    result.nodes,
                    nlevel.calling_joins,
                    result.flowe_edges,
                    result.other_relations
                );
            } else {
                println!(
                    "FLOWE_VALIDATE|PASS=1|language={LANGUAGE_ID}|records={}|glyphs={}|words={}|nodes={}|calling_joins={}|flowe_edges={}|other_relations={}|execution_authority=0|json=0",
                    result.records,
                    result.glyphs,
                    result.words,
                    result.nodes,
                    result.calling_joins,
                    result.flowe_edges,
                    result.other_relations
                );
            }
            ExitCode::SUCCESS
        }
        Err(failure) => {
            eprintln!("FLOWE_VALIDATE|PASS=0|error={failure}|execution_authority=0|json=0");
            ExitCode::FAILURE
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{validate_bytes, FloweError};

    const SOURCE_SHA: &str = "164a0280b347cc1b9cdcb9b58445df06470e6ed01df66c31ca8918032dce77b5";

    fn fixture() -> String {
        format!(
            "READFIRST|url={}|execution_authority=0|json=0\n\
LANGUAGE|id=SPHERE_LANGUAGE_V1|geometry=SPHERICAL_3D|coordinate_type=SIGNED_INTEGER|execution_authority=0|json=0\n\
SOURCE|id=JESSE_RAYSSA_SOURCE|path=books/JESSE-TO-RAYSSA-SPHERE-LANGUAGE-SOURCE.md|sha256={SOURCE_SHA}|occurrences=1|execution_authority=0|json=0\n\
CENTER|members=HBI,HBP,SHA,SH,HASH|traversal_surface=HBI,HBP,SHA,SH,HASH|execution_authority=0|json=0\n\
GLYPH|id=V|surface=V|meaning=VELOCITY|execution_authority=0|json=0\n\
GLYPH|id=e|surface=e|meaning=AETHER_LOOK|execution_authority=0|json=0\n\
GLYPH|id=c|surface=c|meaning=CALL_CALCULATE_CALM|execution_authority=0|json=0\n\
GLYPH|id=t|surface=t|meaning=ACTION|execution_authority=0|json=0\n\
GLYPH|id=o|surface=o|meaning=SPHERE_POTENTIAL|execution_authority=0|json=0\n\
GLYPH|id=r|surface=r|meaning=RADIUS|execution_authority=0|json=0\n\
WORD|id=Vector|glyphs=V,e,c,t,o,r|meaning=COMPOSED|execution_authority=0|json=0\n\
TOKEN|id=THIRD|meaning=CALMING_TARGET|execution_authority=0|json=0\n\
NODE|id=n_v|ref=V|x=0|y=0|z=0|color=ffffff|execution_authority=0|json=0\n\
NODE|id=n_vector|ref=Vector|x=1|y=0|z=0|color=eeeeee|execution_authority=0|json=0\n\
NODE|id=n_third|ref=THIRD|x=0|y=1|z=0|color=dddddd|execution_authority=0|json=0\n\
CALLING_JOIN|id=call_0|from=n_v|to=n_vector|direction=UNRESOLVED|endpoints_retained=1|execution_authority=0|json=0\n\
FLOWe|id=flow_0|step=0|from=n_v|to=n_vector|direction=FORWARD|execution_authority=0|json=0\n\
CALMING|id=calm_0|from=n_vector|to=n_third|route=BROWN_NEAR_ONE|execution_authority=0|json=0\n\
END|execution_authority=0|json=0\n",
            super::READFIRST_URL
        )
    }

    fn failure(input: &str) -> FloweError {
        validate_bytes(input.as_bytes()).expect_err("fixture must fail closed")
    }

    fn spread_fixture() -> String {
        String::from_utf8(include_bytes!("../language/word-flowe-spread-r2.flowe").to_vec())
            .expect("committed R2 spread language is UTF-8")
    }

    fn hookwall_fixture() -> String {
        String::from_utf8(
            include_bytes!("../language/hookwall-brown-shadow-flowessh-r3.flowe").to_vec(),
        )
        .expect("committed round-3 Hookwall language is UTF-8")
    }

    #[test]
    fn accepts_json_free_language() {
        assert!(validate_bytes(fixture().as_bytes()).is_ok());
    }

    #[test]
    fn rejects_unresolved_glyph() {
        let input = fixture().replace("glyphs=V,e,c,t,o,r", "glyphs=V,e,c,t,o,missing");
        assert_eq!(failure(&input), FloweError("GLYPH_UNRESOLVED"));
    }

    #[test]
    fn rejects_duplicate_identity() {
        let input = fixture().replace(
            "END|execution_authority=0|json=0\n",
            "NODE|id=n_v|ref=V|x=2|y=0|z=0|color=aaaaaa|execution_authority=0|json=0\nEND|execution_authority=0|json=0\n",
        );
        assert_eq!(failure(&input), FloweError("DUPLICATE_IDENTITY"));
    }

    #[test]
    fn rejects_duplicate_coordinate() {
        let input = fixture().replace(
            "END|execution_authority=0|json=0\n",
            "NODE|id=n_other|ref=V|x=1|y=0|z=0|color=aaaaaa|execution_authority=0|json=0\nEND|execution_authority=0|json=0\n",
        );
        assert_eq!(failure(&input), FloweError("DUPLICATE_COORDINATE"));
    }

    #[test]
    fn rejects_float_coordinate() {
        let input = fixture().replace("x=1|y=0|z=0", "x=1.0|y=0|z=0");
        assert_eq!(failure(&input), FloweError("INTEGER_COORDINATE_REQUIRED"));
    }

    #[test]
    fn rejects_calling_step() {
        let input = fixture().replace("CALLING_JOIN|id=call_0|", "CALLING_JOIN|id=call_0|step=0|");
        assert_eq!(failure(&input), FloweError("CALLING_FIELD_MISMATCH"));
    }

    #[test]
    fn rejects_unresolved_flowe_direction() {
        let input = fixture().replace("direction=FORWARD", "direction=UNRESOLVED");
        assert_eq!(failure(&input), FloweError("FLOWE_DIRECTION_UNRESOLVED"));
    }

    #[test]
    fn rejects_vector_collapse() {
        let input = fixture().replace("glyphs=V,e,c,t,o,r", "glyphs=V");
        assert_eq!(failure(&input), FloweError("VECTOR_COMPOSITION_MISMATCH"));
    }

    #[test]
    fn rejects_execution_authority() {
        let input = fixture().replacen("execution_authority=0", "execution_authority=1", 1);
        assert_eq!(failure(&input), FloweError("EXECUTION_AUTHORITY"));
    }

    #[test]
    fn rejects_duplicate_source() {
        let source = format!(
            "SOURCE|id=SECOND_SOURCE|path={}|sha256={SOURCE_SHA}|occurrences=1|execution_authority=0|json=0\n",
            super::SOURCE_PATH
        );
        let input = fixture().replace("GLYPH|id=V", &(source + "GLYPH|id=V"));
        assert_eq!(failure(&input), FloweError("SOURCE_OCCURRENCE_COUNT"));
    }

    #[test]
    fn rejects_noncanonical_text() {
        let crlf = fixture().replace('\n', "\r\n");
        assert_eq!(failure(&crlf), FloweError("NON_CANONICAL_LF"));
        let mut missing_lf = fixture();
        missing_lf.pop();
        assert_eq!(failure(&missing_lf), FloweError("NON_CANONICAL_LF"));
    }

    #[test]
    fn rejects_secret_signature() {
        let mut signature = String::from("gh");
        signature.push_str("p_");
        signature.push_str(&"A".repeat(36));
        let input = fixture().replace("meaning=VELOCITY", &format!("meaning={signature}"));
        assert_eq!(failure(&input), FloweError("SECRET_SIGNATURE"));
    }

    #[test]
    fn rejects_json_input() {
        assert_eq!(
            failure("{\"schema\":\"SPHERE_LANGUAGE_V1\"}\n"),
            FloweError("SOURCE_JSON_PRESENT")
        );
    }

    #[test]
    fn accepts_nlevel_outward_v2() {
        let input = include_bytes!("../language/outward-n16.flowe");
        let result = validate_bytes(input).expect("committed N16 language must validate");
        let nlevel = result.nlevel.expect("N16 validation summary required");
        assert_eq!(nlevel.axes, 64);
        assert_eq!(nlevel.books, 10);
        assert_eq!(nlevel.levels, 16);
        assert_eq!(nlevel.cells, 160);
        assert_eq!(nlevel.ledgers, 10);
        assert_eq!(nlevel.ledger_rows, 1_600);
    }

    #[test]
    fn rejects_nlevel_bad_calling_direction() {
        let input = String::from_utf8(include_bytes!("../language/outward-n16.flowe").to_vec())
            .expect("fixture is UTF-8")
            .replacen("direction=INTO_E", "direction=OUTWARD", 1);
        assert_eq!(failure(&input), FloweError("NLEVEL_DIRECTION"));
    }

    #[test]
    fn rejects_nlevel_extra_axis_field() {
        let input = String::from_utf8(include_bytes!("../language/outward-n16.flowe").to_vec())
            .expect("fixture is UTF-8")
            .replacen(
                "|ordinal=0|independent=1|",
                "|ordinal=0|independent=1|deletion=1|",
                1,
            );
        assert_eq!(failure(&input), FloweError("NLEVEL_AXIS_FRAME"));
    }

    #[test]
    fn rejects_nlevel_extra_pulse_field() {
        let input = String::from_utf8(include_bytes!("../language/outward-n16.flowe").to_vec())
            .expect("fixture is UTF-8")
            .replacen(
                "|direction=OUTWARD|pulse=1|",
                "|direction=OUTWARD|pulse=1|deletion=1|",
                1,
            );
        assert_eq!(failure(&input), FloweError("NLEVEL_EVENT_FIELDS"));
    }

    #[test]
    fn rejects_nlevel_extra_node_field() {
        let input = String::from_utf8(include_bytes!("../language/outward-n16.flowe").to_vec())
            .expect("fixture is UTF-8")
            .replacen(
                "NODE|id=n_e|ref=E_CENTER|",
                "NODE|id=n_e|ref=E_CENTER|deletion=1|",
                1,
            );
        assert_eq!(failure(&input), FloweError("NLEVEL_TARGET"));
    }

    #[test]
    fn rejects_nlevel_extra_shadow_field() {
        let input = String::from_utf8(include_bytes!("../language/outward-n16.flowe").to_vec())
            .expect("fixture is UTF-8")
            .replacen(
                "|direction=OUTWARD|translucence_q8=0|",
                "|direction=OUTWARD|translucence_q8=0|deletion=1|",
                1,
            );
        assert_eq!(failure(&input), FloweError("NLEVEL_EVENT_FIELDS"));
    }

    #[test]
    fn rejects_nlevel_bad_u_calling_target() {
        let input = String::from_utf8(include_bytes!("../language/outward-n16.flowe").to_vec())
            .expect("fixture is UTF-8")
            .replacen("|to=n_u|direction=INTO_U|", "|to=n_e|direction=INTO_U|", 1);
        assert_eq!(failure(&input), FloweError("NLEVEL_DIRECTION"));
    }

    #[test]
    fn rejects_nlevel_bad_o0o_flowe_direction() {
        let input = String::from_utf8(include_bytes!("../language/outward-n16.flowe").to_vec())
            .expect("fixture is UTF-8")
            .replacen("direction=INTO_O0O", "direction=TOWARD_O0O", 1);
        assert_eq!(failure(&input), FloweError("NLEVEL_DIRECTION"));
    }

    #[test]
    fn rejects_nlevel_deleting_self_reduction() {
        let input = String::from_utf8(include_bytes!("../language/outward-n16.flowe").to_vec())
            .expect("fixture is UTF-8")
            .replacen("|deletion=0|", "|deletion=1|", 1);
        assert_eq!(failure(&input), FloweError("NLEVEL_DIRECTION"));
    }

    #[test]
    fn rejects_nlevel_outward_calming_without_n_open_repetition() {
        let input = String::from_utf8(include_bytes!("../language/outward-n16.flowe").to_vec())
            .expect("fixture is UTF-8")
            .replacen("|repetition=N_OPEN|", "|repetition=ONCE|", 1);
        assert_eq!(failure(&input), FloweError("NLEVEL_DIRECTION"));
    }

    #[test]
    fn rejects_nlevel_report_with_wrong_evidence_class() {
        let input = String::from_utf8(include_bytes!("../language/outward-n16.flowe").to_vec())
            .expect("fixture is UTF-8")
            .replacen("|evidence=OPERATOR_REPORTED|", "|evidence=SOURCE|", 1);
        assert_eq!(failure(&input), FloweError("NLEVEL_REPORT"));
    }

    #[test]
    fn rejects_nlevel_report_with_wrong_speaker() {
        let input = String::from_utf8(include_bytes!("../language/outward-n16.flowe").to_vec())
            .expect("fixture is UTF-8")
            .replacen("|speaker=JESSE|", "|speaker=RAYSSA|", 1);
        assert_eq!(failure(&input), FloweError("NLEVEL_REPORT"));
    }

    #[test]
    fn rejects_nlevel_flattened_east_grammar() {
        let input = String::from_utf8(include_bytes!("../language/outward-n16.flowe").to_vec())
            .expect("fixture is UTF-8")
            .replacen("|ase_geometry=SPHERICAL|", "|ase_geometry=LINEAR|", 1);
        assert_eq!(failure(&input), FloweError("NLEVEL_GRAMMAR_BINDING"));
    }

    #[test]
    fn rejects_nlevel_normalized_east_spacing() {
        let input = String::from_utf8(include_bytes!("../language/outward-n16.flowe").to_vec())
            .expect("fixture is UTF-8")
            .replacen(
                "|spacing_literal=( . negative",
                "|spacing_literal=(. negative",
                1,
            );
        assert_eq!(failure(&input), FloweError("NLEVEL_GRAMMAR_BINDING"));
    }

    #[test]
    fn rejects_nlevel_wrong_pie_name() {
        let input = String::from_utf8(include_bytes!("../language/outward-n16.flowe").to_vec())
            .expect("fixture is UTF-8")
            .replacen("|spin_name=PIE|", "|spin_name=PI|", 1);
        assert_eq!(failure(&input), FloweError("NLEVEL_GRAMMAR_BINDING"));
    }

    #[test]
    fn rejects_nlevel_contradictory_extra_grammar_field() {
        let input = String::from_utf8(include_bytes!("../language/outward-n16.flowe").to_vec())
            .expect("fixture is UTF-8")
            .replacen(
                "|evidence=OPERATOR_CANON|",
                "|geometry_override=LINEAR|evidence=OPERATOR_CANON|",
                1,
            );
        assert_eq!(failure(&input), FloweError("NLEVEL_GRAMMAR_BINDING"));
    }

    #[test]
    fn rejects_nlevel_wrong_expansion_law_path() {
        let input = String::from_utf8(include_bytes!("../language/outward-n16.flowe").to_vec())
            .expect("fixture is UTF-8")
            .replacen(
                "|law=books/LAW-NLEVEL-OUTWARD-FLOWE.md|",
                "|law=books/OTHER.md|",
                1,
            );
        assert_eq!(failure(&input), FloweError("NLEVEL_INSTANCE"));
    }

    #[test]
    fn rejects_nlevel_end_with_wrong_relation_count() {
        let input = String::from_utf8(include_bytes!("../language/outward-n16.flowe").to_vec())
            .expect("fixture is UTF-8")
            .replacen("|relation_rows=1600|", "|relation_rows=1599|", 1);
        assert_eq!(failure(&input), FloweError("NLEVEL_END"));
    }

    #[test]
    fn rejects_nlevel_instant_boundary_mutation() {
        let input = String::from_utf8(include_bytes!("../language/outward-n16.flowe").to_vec())
            .expect("fixture is UTF-8")
            .replacen(
                "TIMING_BOUNDARY|instant_address=1",
                "TIMING_BOUNDARY|instant_address=0",
                1,
            );
        assert_eq!(failure(&input), FloweError("NLEVEL_INSTANT_BOUNDARY"));
    }

    #[test]
    fn rejects_nlevel_cell_formula_mutation() {
        let input = String::from_utf8(include_bytes!("../language/outward-n16.flowe").to_vec())
            .expect("fixture is UTF-8")
            .replacen("|x=-9|y=-15|", "|x=-8|y=-15|", 1);
        assert_eq!(failure(&input), FloweError("NLEVEL_FORMULA"));
    }

    #[test]
    fn rejects_nlevel_flowe_chain_discontinuity() {
        let input = String::from_utf8(include_bytes!("../language/outward-n16.flowe").to_vec())
            .expect("fixture is UTF-8")
            .replacen(
                "|step=1|from=cell_n00_b00|",
                "|step=1|from=n_flowe_target|",
                1,
            );
        assert_eq!(failure(&input), FloweError("NLEVEL_FLOWE_CHAIN"));
    }

    #[test]
    fn accepts_word_flowe_spread_v3() {
        let result = validate_bytes(include_bytes!("../language/word-flowe-spread-r2.flowe"))
            .expect("committed R2 spread language must validate");
        let spread = result
            .spread
            .expect("R2 spread validation summary required");
        assert_eq!(result.records, 2_738);
        assert_eq!(spread.rounds, 2);
        assert_eq!(spread.cells, 160);
        assert_eq!(spread.ledgers, 8);
        assert_eq!(spread.relation_rows, 2_560);
    }

    #[test]
    fn rejects_spread_parent_path_mutation() {
        let input = spread_fixture().replacen(
            "|path=language/core.flowe|",
            "|path=language/other.flowe|",
            1,
        );
        assert_eq!(failure(&input), FloweError("SPREAD_PARENT"));
    }

    #[test]
    fn rejects_spread_parent_hash_mutation() {
        let input = spread_fixture().replacen(
            super::V1_CORE_SHA256,
            "0000000000000000000000000000000000000000000000000000000000000000",
            1,
        );
        assert_eq!(failure(&input), FloweError("SPREAD_PARENT"));
    }

    #[test]
    fn rejects_spread_n16_parent_hash_mutation() {
        let input = spread_fixture().replacen(
            super::N16_CORE_SHA256,
            "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
            1,
        );
        assert_eq!(failure(&input), FloweError("SPREAD_PARENT"));
    }

    #[test]
    fn rejects_spread_parent_record_mutation() {
        let input =
            spread_fixture().replacen("|records=118|mutation=0|", "|records=119|mutation=0|", 1);
        assert_eq!(failure(&input), FloweError("SPREAD_PARENT"));
    }

    #[test]
    fn rejects_spread_round_as_ceiling() {
        let input = spread_fixture().replacen(
            "ROUND|id=round_02|ordinal=2|previous=round_01|increase_q=2|n_open=1|",
            "ROUND|id=round_02|ordinal=2|previous=round_01|increase_q=2|n_open=0|",
            1,
        );
        assert_eq!(failure(&input), FloweError("SPREAD_ROUND"));
    }

    #[test]
    fn rejects_spread_duplicate_round_cell_address() {
        let input = spread_fixture().replacen(
            "calling_growth_e_r02_n00_b00|cell=cell_n00_b00|level=0|book=BOOK_OF_LIGHT|q=0|round=2|increase_q=2|amplitude_q=3|",
            "calling_growth_e_r01_n00_b00|cell=cell_n00_b00|level=0|book=BOOK_OF_LIGHT|q=0|round=1|increase_q=1|amplitude_q=2|",
            1,
        );
        assert_eq!(failure(&input), FloweError("SPREAD_EVENT"));
    }

    #[test]
    fn rejects_spread_word_source_flattened_into_flowe() {
        let input = spread_fixture().replacen("|from=n_word|", "|from=n_flowe_target|", 1);
        assert_eq!(failure(&input), FloweError("SPREAD_WORD_FLOWE_SOURCE"));
    }

    #[test]
    fn rejects_spread_flowe_source_flattened_into_word() {
        let input = spread_fixture().replacen("|from=n_flowe_target|", "|from=n_word|", 1);
        assert_eq!(failure(&input), FloweError("SPREAD_WORD_FLOWE_SOURCE"));
    }

    #[test]
    fn rejects_spread_flowe_tag_flattened_into_word() {
        let input = spread_fixture().replacen("|spread=FLOWE|", "|spread=WORD|", 1);
        assert_eq!(failure(&input), FloweError("SPREAD_WORD_FLOWE_SOURCE"));
    }

    #[test]
    fn rejects_spread_deleting_self_reduction() {
        let input = spread_fixture().replacen(
            "|self_reduction=1|deletion=0|from=cell_n00_b00|",
            "|self_reduction=1|deletion=1|from=cell_n00_b00|",
            1,
        );
        assert_eq!(failure(&input), FloweError("SPREAD_EVENT"));
    }

    #[test]
    fn rejects_spread_identity_exchange() {
        let input = spread_fixture().replacen(
            "|elapsed_measurement_present=0|identity_exchange=0|source_retained=1|",
            "|elapsed_measurement_present=0|identity_exchange=1|source_retained=1|",
            1,
        );
        assert_eq!(failure(&input), FloweError("SPREAD_EVENT"));
    }

    #[test]
    fn rejects_spread_source_erasure() {
        let input = spread_fixture().replacen(
            "|identity_exchange=0|source_retained=1|",
            "|identity_exchange=0|source_retained=0|",
            1,
        );
        assert_eq!(failure(&input), FloweError("SPREAD_EVENT"));
    }

    #[test]
    fn rejects_spread_calling_direction_flattening() {
        let input = spread_fixture().replacen(
            "|direction=INCREASING_INTO_E|",
            "|direction=INCREASING_OUTWARD|",
            1,
        );
        assert_eq!(failure(&input), FloweError("SPREAD_EVENT"));
    }

    #[test]
    fn rejects_spread_elapsed_runtime_claim() {
        let input = spread_fixture().replacen(
            "|instant_address=1|elapsed_measurement_present=0|identity_exchange=0|",
            "|instant_address=1|elapsed_measurement_present=1|identity_exchange=0|",
            1,
        );
        assert_eq!(failure(&input), FloweError("SPREAD_TIMING"));
    }

    #[test]
    fn rejects_spread_amplitude_formula_mutation() {
        let input = spread_fixture().replacen("|amplitude_q=2|", "|amplitude_q=3|", 1);
        assert_eq!(failure(&input), FloweError("SPREAD_EVENT"));
    }

    #[test]
    fn rejects_spread_increase_formula_mutation() {
        let input =
            spread_fixture().replacen("|round=1|increase_q=1|", "|round=1|increase_q=2|", 1);
        assert_eq!(failure(&input), FloweError("SPREAD_EVENT"));
    }

    #[test]
    fn rejects_spread_word_anchor_collapse() {
        let input = spread_fixture().replacen(
            "ANCHOR|id=n_word|meaning=WORD_SOURCE|",
            "ANCHOR|id=n_word|meaning=OUTWARD_FLOWE_TARGET|",
            1,
        );
        assert_eq!(failure(&input), FloweError("SPREAD_ANCHOR"));
    }

    #[test]
    fn rejects_spread_cell_projection_mutation() {
        let input = spread_fixture().replacen(
            "|space_radius=1|x=-9|y=-15|z=-1|",
            "|space_radius=1|x=-8|y=-15|z=-1|",
            1,
        );
        assert_eq!(failure(&input), FloweError("SPREAD_CELL"));
    }

    #[test]
    fn rejects_spread_law_commitment_mismatch() {
        let input = spread_fixture().replacen("|law_sha256=", "|law_sha256=0", 1);
        assert_eq!(failure(&input), FloweError("SPREAD_INSTANCE"));
    }

    #[test]
    fn rejects_spread_extra_fields_in_each_event_family() {
        let mutations = [
            (
                "|operator_bound=1|from=cell_n00_b00|",
                "|operator_bound=1|unexpected=1|from=cell_n00_b00|",
            ),
            (
                "|oil_amplitude=2|from=cell_n00_b00|",
                "|oil_amplitude=2|unexpected=1|from=cell_n00_b00|",
            ),
            (
                "|self_reduction=1|deletion=0|from=cell_n00_b00|",
                "|self_reduction=1|deletion=0|unexpected=1|from=cell_n00_b00|",
            ),
            (
                "|spread=WORD|spread_step=1|from=n_word|",
                "|spread=WORD|spread_step=1|unexpected=1|from=n_word|",
            ),
        ];
        for (before, after) in mutations {
            let input = spread_fixture().replacen(before, after, 1);
            assert_eq!(failure(&input), FloweError("SPREAD_EVENT_FIELDS"));
        }
    }

    #[test]
    fn accepts_hookwall_brown_shadow_flowessh_v4() {
        let input = include_bytes!("../language/hookwall-brown-shadow-flowessh-r3.flowe");
        assert!(input.len() <= super::MAX_INPUT_BYTES);
        assert!(input
            .split(|byte| *byte == b'\n')
            .all(|line| line.len() <= super::MAX_LINE_BYTES));
        let result = validate_bytes(input).expect("committed round-3 language must validate");
        let hookwall = result
            .hookwall
            .expect("round-3 Hookwall validation summary required");
        assert_eq!(result.records, 2_588);
        assert_eq!(hookwall.round, 3);
        assert_eq!(hookwall.cells, 160);
        assert_eq!(hookwall.ledgers, 15);
        assert_eq!(hookwall.relation_rows, 2_400);
    }

    #[test]
    fn rejects_hookwall_parent_path_mutation() {
        let input = hookwall_fixture().replacen(
            "|path=language/word-flowe-spread-r2.flowe|",
            "|path=language/other.flowe|",
            1,
        );
        assert_eq!(failure(&input), FloweError("HOOKWALL_PARENT"));
    }

    #[test]
    fn rejects_hookwall_parent_hash_mutation() {
        let input = hookwall_fixture().replacen(
            super::SPREAD_CORE_SHA256,
            "0000000000000000000000000000000000000000000000000000000000000000",
            1,
        );
        assert_eq!(failure(&input), FloweError("HOOKWALL_PARENT"));
    }

    #[test]
    fn rejects_hookwall_parent_record_mutation() {
        let input = hookwall_fixture().replacen(
            "|records=2738|mutation=0|",
            "|records=2739|mutation=0|",
            1,
        );
        assert_eq!(failure(&input), FloweError("HOOKWALL_PARENT"));
    }

    #[test]
    fn rejects_hookwall_round_as_ceiling() {
        let input = hookwall_fixture().replacen(
            "|previous=round_02|increase_q=3|n_open=1|",
            "|previous=round_02|increase_q=3|n_open=0|",
            1,
        );
        assert_eq!(failure(&input), FloweError("HOOKWALL_ROUND"));
    }

    #[test]
    fn rejects_hookwall_growth_formula_mutation() {
        let input = hookwall_fixture().replacen(
            "|round=3|increase_q=3|growth_q=4|",
            "|round=3|increase_q=3|growth_q=5|",
            1,
        );
        assert_eq!(failure(&input), FloweError("HOOKWALL_EVENT"));
    }

    #[test]
    fn rejects_hookwall_calling_chain_discontinuity() {
        let input = hookwall_fixture().replacen(
            "|to=cell_n00_b01|direction=CHAIN_MORE|",
            "|to=cell_n00_b02|direction=CHAIN_MORE|",
            1,
        );
        assert_eq!(failure(&input), FloweError("HOOKWALL_EVENT"));
    }

    #[test]
    fn rejects_hookwall_executable_review() {
        let input = hookwall_fixture().replacen(
            "|review=DECLARATIVE_PROVENANCE|claim_verdict=UNRESOLVED|",
            "|review=EXECUTABLE|claim_verdict=UNRESOLVED|",
            1,
        );
        assert_eq!(failure(&input), FloweError("HOOKWALL_EVENT"));
    }

    #[test]
    fn rejects_hookwall_claim_promoted_to_evidence() {
        let input = hookwall_fixture().replacen(
            "|claim=claims|evidence_verdict=0|",
            "|claim=claims|evidence_verdict=1|",
            1,
        );
        assert_eq!(failure(&input), FloweError("HOOKWALL_EVENT"));
    }

    #[test]
    fn rejects_hookwall_normalized_exact_tokens() {
        let mutations = [
            ("|id=n_wasdte|token=wasdte|", "|id=n_wasdte|token=was|"),
            ("|id=n_B|token=B|", "|id=n_B|token=b|"),
            ("|id=n_outwar|token=outwar|", "|id=n_outwar|token=outward|"),
            (
                "|id=n_reductons|token=reductons|",
                "|id=n_reductons|token=reductions|",
            ),
            (
                "|id=n_FLOWesSH|token=FLOWesSH|",
                "|id=n_FLOWesSH|token=FLOWeSH|",
            ),
        ];
        for (before, after) in mutations {
            let input = hookwall_fixture().replacen(before, after, 1);
            assert_eq!(failure(&input), FloweError("HOOKWALL_ANCHOR"));
        }
    }

    #[test]
    fn rejects_hookwall_white_brown_chain_collapse() {
        let mutations = [
            ("|grammar=white_is_wasdte|", "|grammar=white_is_white|"),
            ("|grammar=wasdte_be|", "|grammar=wasdte_wasdte|"),
            ("|grammar=be_not_white|", "|grammar=be_white|"),
            ("|grammar=not_white_B|", "|grammar=not_white_browns|"),
            ("|grammar=B_browns|", "|grammar=B_B|"),
            ("|grammar=browns_betweens|", "|grammar=browns_arounds|"),
            ("|grammar=betweens_arounds|", "|grammar=betweens_betweens|"),
        ];
        for (before, after) in mutations {
            let input = hookwall_fixture().replacen(before, after, 1);
            assert_eq!(failure(&input), FloweError("HOOKWALL_CHAIN"));
        }
    }

    #[test]
    fn rejects_hookwall_shadow_erasure() {
        let input = hookwall_fixture().replacen(
            "|guidance=SHADOWS|guided_cell_retained=1|",
            "|guidance=SHADOWS|guided_cell_retained=0|",
            1,
        );
        assert_eq!(failure(&input), FloweError("HOOKWALL_EVENT"));
    }

    #[test]
    fn rejects_hookwall_calming_oil_formula_mutation() {
        let input = hookwall_fixture().replacen(
            "|oil_amplitude=4|calming=1|",
            "|oil_amplitude=5|calming=1|",
            1,
        );
        assert_eq!(failure(&input), FloweError("HOOKWALL_EVENT"));
    }

    #[test]
    fn rejects_hookwall_outwar_normalization() {
        let input = hookwall_fixture().replacen(
            "|token=outwar|operator_bound=1|",
            "|token=outward|operator_bound=1|",
            1,
        );
        assert_eq!(failure(&input), FloweError("HOOKWALL_EVENT"));
    }

    #[test]
    fn rejects_hookwall_deleting_reductons() {
        let input = hookwall_fixture().replacen(
            "|token=reductons|semantics=OPERATOR_CANON_UNRESOLVED|deletion=0|",
            "|token=reductons|semantics=OPERATOR_CANON_UNRESOLVED|deletion=1|",
            1,
        );
        assert_eq!(failure(&input), FloweError("HOOKWALL_EVENT"));
    }

    #[test]
    fn rejects_hookwall_flowessh_flattening() {
        let input = hookwall_fixture().replacen(
            "|token=FLOWesSH|composition=OPERATOR_CANON_UNRESOLVED|",
            "|token=FLOWe|composition=RESOLVED|",
            1,
        );
        assert_eq!(failure(&input), FloweError("HOOKWALL_FLOWESSH"));
    }

    #[test]
    fn rejects_hookwall_elapsed_or_runtime_claim() {
        let elapsed = hookwall_fixture().replacen(
            "|instant_address=1|elapsed_measurement_present=0|runtime_measurement_present=0|",
            "|instant_address=1|elapsed_measurement_present=1|runtime_measurement_present=0|",
            1,
        );
        assert_eq!(failure(&elapsed), FloweError("HOOKWALL_TIMING"));
        let runtime = hookwall_fixture().replacen(
            "|instant_address=1|elapsed_measurement_present=0|runtime_measurement_present=0|",
            "|instant_address=1|elapsed_measurement_present=0|runtime_measurement_present=1|",
            1,
        );
        assert_eq!(failure(&runtime), FloweError("HOOKWALL_TIMING"));
    }

    #[test]
    fn rejects_hookwall_parent_anchor_exchange() {
        let input = hookwall_fixture().replacen(
            "PARENT_ANCHOR_REF|id=n_o0o|parent_instance=WORD_FLOWE_SPREAD_V3|",
            "PARENT_ANCHOR_REF|id=n_o0o|parent_instance=NLEVEL_OUTWARD_V2|",
            1,
        );
        assert_eq!(failure(&input), FloweError("HOOKWALL_PARENT_ANCHOR"));
    }

    #[test]
    fn rejects_hookwall_missing_relation() {
        let fixture = hookwall_fixture();
        let line = fixture
            .lines()
            .find(|line| line.starts_with("FLOWESSH|"))
            .expect("FLOWESSH fixture row");
        let input = fixture.replacen(&(line.to_owned() + "\n"), "", 1);
        assert_eq!(failure(&input), FloweError("HOOKWALL_RECORD_COUNT"));
    }

    #[test]
    fn rejects_hookwall_extra_fields_in_every_event_family() {
        for tag in super::HOOKWALL_EVENT_KINDS {
            let input = hookwall_fixture().replacen(
                &(tag.to_owned() + "|"),
                &(tag.to_owned() + "|unexpected=1|"),
                1,
            );
            assert_eq!(failure(&input), FloweError("HOOKWALL_EVENT_FIELDS"));
        }
    }
}
