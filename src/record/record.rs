use crate::record::validation::{load_schema_validator, validation_error_descr};
use crate::record::version::SemVer;
use jsonschema::JSONSchema;
use serde_json::{Map, Value};
use std::collections::HashMap;

#[derive(Debug)]
pub struct EntityKind<'a> {
    pub ns1: &'a str,
    pub ns2: &'a str,
    pub component: &'a str,
    pub version: SemVer,
}

impl<'t> EntityKind<'t> {
    /// parse kind str which should match regex
    /// ```
    /// [\w\-\.]+:[\w\-\.]+:[\w\-\.]+:[0-9]+.[0-9]+.[0-9]+
    /// ```
    ///
    /// kind is decomposed into [EntityKind].
    ///
    pub fn parse(v: &'t str) -> Option<Self> {
        // ugly but faster than v.splitn(4,':',).collect::<Vec<&str>>();
        if let Some(p1) = v.find(':') {
            // note: this only works since there's only ASCII char
            if let Some(p2) = v[p1 + 1..].find(':') {
                if let Some(p3) = v[p2 + p1 + 2..].find(':') {
                    return Some(EntityKind {
                        ns1: &v[0..p1],
                        ns2: &v[p1 + 1..p1 + 1 + p2],
                        component: &v[p1 + p2 + 2..p1 + p2 + p3 + 2],
                        version: v[p1 + p2 + p3 + 3..].parse::<SemVer>().ok()?,
                    });
                }
            }
        }

        return None;
    }
}

const SUPPORTED_SCHEMAS: &'static [(&'static str, &[(&'static str, &'static str)])] = &[(
    "work-product-component--WellLog",
    &[
        ("1.1.*", "work-product-component/WellLog.1.1.0.json"),
        ("1.0.*", "work-product-component/WellLog.1.0.0.json"),
        ("1.*.*", "work-product-component/WellLog.1.2.0.json"),
    ],
)];

fn load_json_schemas() -> HashMap<&'static str, Vec<(SemVer, JSONSchema)>> {
    let mut table: HashMap<&str, Vec<(SemVer, JSONSchema)>> = HashMap::new();
    for (component, other) in SUPPORTED_SCHEMAS.iter() {
        table.insert(
            component,
            other
                .iter()
                .map(|(ver_str, file_path)| {
                    (
                        ver_str.parse::<SemVer>().unwrap(),
                        load_schema_validator(file_path).unwrap(),
                    )
                })
                .collect(),
        );
    }

    table
}

lazy_static::lazy_static! {
    static ref SCHEMA_VALIDATORS: HashMap<&'static str, Vec<(SemVer, JSONSchema)>> = load_json_schemas();
    static ref COUNT: usize = SCHEMA_VALIDATORS.len();
}

fn schema_from_kind(kind: &str) -> Option<&JSONSchema> {
    let k = EntityKind::parse(kind)?;
    for (version, schema) in SCHEMA_VALIDATORS.get(k.component)? {
        if version.is_match_other(&k.version) {
            return Some(schema);
        }
    }
    None
}

pub struct RecordMap<'a> {
    map: &'a Map<String, Value>,
}

// struct RecordMapMut<'a> {
//     map: &'a mut Map<String, Value>,
// }

pub struct Record {
    value: Value,
}

impl Record {
    pub fn from_object_unvalidated(instance: Value) -> Result<Self, (Value, String)> {
        if !instance.is_object() {
            return Err((instance, "not an object".to_string()));
        }
        Ok(Self { value: instance })
    }

    pub fn from_json(json_str: &str) -> Result<Self, String> {
        let doc: Value = serde_json::from_str(json_str).map_err(|err| err.to_string())?;
        
    }

    pub fn from_object_validated(instance: Value) -> Result<Self, (Value, Vec<String>)> {
        if !instance.is_object() {
            return Err((instance, vec!["not an object".to_string()]));
        }

        let record = Self { value: instance };
        match record.validate() {
            Err(s) => Err((record.value, s)),
            _ => Ok(record),
        }
    }

    pub fn as_map(&self) -> Option<RecordMap> {
        self.value.as_object().map(|v| RecordMap {map: v})
    }

    // pub fn as_map_mut(& mut self) -> RecordMapMut {
    //     RecordMapMut{map: self.value.as_object_mut().unwrap()}
    // }

    pub fn validate(&self) -> Result<(), Vec<String>> {
        if let Some(record_obj) = self.value.as_object() {
            let record_map = RecordMap { map: record_obj };
            let schema = match record_map.kind() {
                None => None,
                Some(kind) => schema_from_kind(kind),
            };

            if let Some(s) = schema {
                if let Err(r) = s.validate(&self.value) {
                    return Err(r.map(|t| validation_error_descr(&t)).collect::<Vec<_>>());
                }
            } else {
                return Err(vec!["no schema found".to_string()]);
            }

            Ok(())
        } else {
            Err(vec!["An object is expected".to_string()])
        }
    }
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
        let kind = EntityKind::parse("osdu:wks:work-product-component--WellLog:1.2.50").unwrap();

        // let kind: EntityKind = "osdu:wks:work-product-component--WellLog:1.2.50".parse().unwrap();
        println!("ns1: {}", kind.ns1);
        println!("ns2: {}", kind.ns2);
        println!("component: {}", kind.component);
        println!("version: {:?}", kind.version);

        let sc = schema_from_kind("osdu:wks:work-product-component--WellLog:1.2.50");
        assert!(sc.is_some());
    }

    #[test]
    fn test_load_record() {
        let f = std::fs::File::open("res/examples/wellLog_v3_120.json").unwrap();
        let document: Value = serde_json::from_reader(f).unwrap();
        let record = Record::from_object_unvalidated(document).unwrap();
        println!("kind => {}", record.as_map().unwrap().kind().unwrap());
    }

    #[test]
    fn test_validate_record() {
        let f = std::fs::File::open("res/examples/wellLog_v3_120.json").unwrap();
        let document: Value = serde_json::from_reader(f).unwrap();
        let record = Record::from_object_validated(document);
        match record {
            Ok(_) => {}
            Err((_, err)) => {
                println!("errors {}", err.join(","));
                assert!(false);
            }
        };
        // assert!(record.is_ok())
    }
}
