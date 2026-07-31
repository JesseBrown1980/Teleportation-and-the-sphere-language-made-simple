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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Validation {
    records: usize,
    glyphs: usize,
    words: usize,
    nodes: usize,
    calling_joins: usize,
    flowe_edges: usize,
    other_relations: usize,
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

fn parse_i64(value: &str) -> Result<i64> {
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
        return Err(error("INTEGER_COORDINATE_REQUIRED"));
    }
    value
        .parse::<i64>()
        .map_err(|_| error("INTEGER_COORDINATE_REQUIRED"))
}

fn parse_u64(value: &str) -> Result<u64> {
    if value != "0" && (value.starts_with('0') || !value.bytes().all(|byte| byte.is_ascii_digit()))
    {
        return Err(error("FLOWE_STEP"));
    }
    value.parse::<u64>().map_err(|_| error("FLOWE_STEP"))
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

fn validate_bytes(bytes: &[u8]) -> Result<Validation> {
    let text = canonical_text(bytes)?;
    let rows: Vec<Row> = text.lines().map(parse_row).collect::<Result<_>>()?;
    if rows.first().map(|row| row.tag.as_str()) != Some("READFIRST")
        || rows.last().map(|row| row.tag.as_str()) != Some("END")
    {
        return Err(error("ROW_ORDER"));
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
}
