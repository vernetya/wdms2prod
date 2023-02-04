use crate::error::{ErrorCode, WDMSError};
use crate::model::validation::{load_schema_validator, validation_error_descr};
use crate::model::version::SemVer;
use jsonschema::JSONSchema;
use serde_json::{Map, Value};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RecordErrorToDelete {
    #[error("parsing error: {0}")]
    Parsing(#[from] serde_json::error::Error),
    #[error("record invalid due to the following errors: {0:?}")]
    Validation(Vec<String>),
    #[error("Unknown kind")]
    UnknownKind(EntityKind),
}

#[derive(Debug)]
pub enum EntityType {
    Wellbore,
    WellLog,
    Uncategorised(String),
}

#[derive(Debug)]
pub enum EntitySource {
    OSDU,
    Uncategorised(String),
}

#[derive(Debug)]
pub struct EntityKindSlice<'a> {
    pub ns1: &'a str,
    pub ns2: &'a str,
    pub component: &'a str,
    pub version: SemVer,
}

#[derive(Debug)]
pub struct EntityKind {
    pub e_type: EntityType,
    pub source: EntitySource,
    pub version: SemVer,
}

impl From<&str> for EntityType {
    fn from(value: &str) -> Self {
        if value == "work-product-component--WellLog" {
            return EntityType::WellLog;
        }
        EntityType::Uncategorised(value.to_string())
    }
}
impl From<&str> for EntitySource {
    fn from(value: &str) -> Self {
        if value == "osdu" {
            return EntitySource::OSDU;
        }
        EntitySource::Uncategorised(value.to_string())
    }
}

impl<'a> From<EntityKindSlice<'a>> for EntityKind {
    fn from(value: EntityKindSlice) -> Self {
        Self {
            e_type: value.component.into(),
            source: value.ns1.into(),
            version: value.version,
        }
    }
}

impl TryFrom<&str> for EntityKind {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(EntityKindSlice::try_from(value)?.into())
    }
}

impl<'a> TryFrom<&'a str> for EntityKindSlice<'a> {
    type Error = ();

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        match Self::parse(value) {
            Some(v) => Ok(v),
            None => Err(()),
        }
    }
}

impl<'t> EntityKindSlice<'t> {
    /// parse kind str which should match regex
    /// [\w\-\.]+:[\w\-\.]+:[\w\-\.]+:[0-9]+.[0-9]+.[0-9]+
    ///
    /// kind is decomposed into [EntityKindSlice].
    ///
    pub fn parse(v: &'t str) -> Option<Self> {
        // ugly but faster than v.splitn(4,':',).collect::<Vec<&str>>();
        if let Some(p1) = v.find(':') {
            // note: this only works since there's only ASCII char
            if let Some(p2) = v[p1 + 1..].find(':') {
                if let Some(p3) = v[p2 + p1 + 2..].find(':') {
                    return Some(EntityKindSlice {
                        ns1: &v[0..p1],
                        ns2: &v[p1 + 1..p1 + 1 + p2],
                        component: &v[p1 + p2 + 2..p1 + p2 + p3 + 2],
                        version: v[p1 + p2 + p3 + 3..].parse::<SemVer>().ok()?,
                    });
                }
            }
        }

        None
    }
}

const SUPPORTED_SCHEMAS: &[(&str, &[(&str, &str)])] = &[(
    "work-product-component--WellLog",
    &[
        ("1.1.*", "work-product-component/WellLog.1.1.0.json"),
        ("1.0.*", "work-product-component/WellLog.1.0.0.json"),
        ("1.*.*", "work-product-component/WellLog.1.2.0.json"),
    ],
)];

fn load_json_schemas() -> HashMap<&'static str, Vec<(EntityKind, JSONSchema)>> {
    let mut table: HashMap<&str, Vec<(EntityKind, JSONSchema)>> = HashMap::new();
    for (component, other) in SUPPORTED_SCHEMAS.iter() {
        table.insert(
            component,
            other
                .iter()
                .map(|(ver_str, file_path)| {
                    (
                        EntityKind {
                            e_type: EntityType::from(*component),
                            source: EntitySource::OSDU,
                            version: ver_str.parse::<SemVer>().unwrap(),
                        },
                        load_schema_validator(file_path).unwrap(),
                    )
                })
                .collect(),
        );
    }

    table
}

lazy_static::lazy_static! {
    static ref SCHEMA_VALIDATORS: HashMap<&'static str, Vec<(EntityKind, JSONSchema)>> = load_json_schemas();
}

pub fn print_loaded_schemas() {
    print!("{} schemas loaded", SCHEMA_VALIDATORS.len());
}

fn schema_from_kind(kind: &str) -> Result<&JSONSchema, WDMSError> {
    let kind_tokens = EntityKindSlice::parse(kind)
        .ok_or_else(|| WDMSError::from_validation(format!("invalid kind: {}", kind), None))?;
    if let Some(ll) = SCHEMA_VALIDATORS.get(kind_tokens.component) {
        for (kind, schema) in ll {
            if kind.version.is_match_other(&kind_tokens.version) {
                return Ok(schema);
            }
        }
    }
    Err(WDMSError::from_record(
        ErrorCode::UnknownKind,
        kind.to_string(),
        None,
    ))
}

pub struct RecordMap<'a> {
    map: &'a Map<String, Value>,
}

/// parsed and validated Record according to its kind
pub struct Record {
    pub value: Value,
    pub kind: EntityKind,
}

impl Record {
    pub fn from_string(value: &str) -> Result<Self, WDMSError> {
        let doc: Value = serde_json::from_str(value)?;
        Self::from_json(doc)
    }

    pub fn from_slice(value: &[u8]) -> Result<Self, WDMSError> {
        let doc: Value = serde_json::from_slice(value)?;
        Self::from_json(doc)
    }

    pub fn from_json(value: Value) -> Result<Self, WDMSError> {
        let kind = validate_record(&value)?;
        Ok(Record { value, kind })
    }
}

fn validate_record(value: &Value) -> Result<EntityKind, WDMSError> {
    let record_map = value
        .as_object()
        .map(|v| RecordMap { map: v })
        .ok_or_else(|| {
            WDMSError::from_validation("Invalid record, an object is expected".to_string(), None)
        })?;

    let kind = record_map
        .kind()
        .ok_or_else(|| WDMSError::from_validation("kind field is missing".to_string(), None))?;

    let schema = schema_from_kind(kind)?;

    if let Err(r) = schema.validate(value) {
        return Err(WDMSError::from_validation(
            r.map(|t| validation_error_descr(&t))
                .collect::<Vec<_>>()
                .join(", "),
            None,
        ));
    }
    EntityKind::try_from(kind)
        .map_err(|_| WDMSError::from_validation(format!("invalid kind: {}", kind), None))
}

impl<'t> RecordMap<'t> {
    fn get_str(&'t self, field: &str) -> Option<&'t str> {
        self.map.get(field)?.as_str()
    }

    pub fn kind(&'t self) -> Option<&'t str> {
        self.get_str("kind")
    }
}

#[cfg(test)]
mod test {
    #[allow(unused_imports)]
    use super::*;
    // use ::test::Bencher;

    #[test]
    fn test_parse_kind() {
        let kind =
            EntityKindSlice::parse("osdu:wks:work-product-component--WellLog:1.2.50").unwrap();

        // let kind: EntityKindSlice = "osdu:wks:work-product-component--WellLog:1.2.50".parse().unwrap();
        println!("ns1: {}", kind.ns1);
        println!("ns2: {}", kind.ns2);
        println!("component: {}", kind.component);
        println!("version: {:?}", kind.version);

        let sc = schema_from_kind("osdu:wks:work-product-component--WellLog:1.2.50");
        assert!(sc.is_ok());
    }

    #[test]
    fn test_load_record() {
        let f = std::fs::File::open("res/examples/wellLog_v3_120.json").unwrap();
        let document: Value = serde_json::from_reader(f).unwrap();
        let record = Record::from_json(document).unwrap();
        println!("kind => {:?}", record.kind);
    }
}
