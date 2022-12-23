use serde_json::{Value};
use std::sync::Arc;
use jsonschema::{JSONSchema, SchemaResolver, SchemaResolverError};
use url::Url;
use std::collections::HashMap;

use lazy_static;

struct LocalSchemaResolver;

const BASE_OSDU_SCHEMA_URL: &str = "https://schema.osdu.opengroup.org/json/";
const BASE_OSDU_SCHEMA_LOCAL: &str = "res/schemas/";

// const SUPPORTED_SCHEMAS: &'static [(EntityType, &'static str, &'static str)] = &[
//     (EntityType::WellLog, "work-product-component--WellLog:1.2.0", "work-product-component/WellLog.1.2.0.json"),
//     (EntityType::WellLog, "work-product-component--WellLog:1.1.0", "work-product-component/WellLog.1.1.0.json"),
//     (EntityType::WellLog, "work-product-component--WellLog:1.0.0", "work-product-component/WellLog.1.0.0.json")
// ];


const SUPPORTED_SCHEMAS: &'static [EntitySchemaDef] = &[
    // EntitySchemaDef(EntityType::WellLog,"work-product-component--WellLog:1.2.0", "work-product-component/WellLog.1.2.0.json"),
    // EntitySchemaDef(EntityType::WellLog,"work-product-component--WellLog:1.1.0", "work-product-component/WellLog.1.1.0.json"),
    // EntitySchemaDef(EntityType::WellLog,"work-product-component--WellLog:1.0.0", "work-product-component/WellLog.1.0.0.json"),
];

enum EntityType {
    WellLog
}

struct EntitySchemaDef (EntityType, &'static str, &'static str);

impl EntitySchemaDef {
    fn kind_prefix(&self) -> &str { self.1 }
    fn schema_file(&self) -> &str { self.2 }
}


#[inline]
fn kind_to_prefix(kind: &str) -> Option<&str> {
    kind.splitn(3,':').last()
}


fn validator_from_kind(kind: &str) -> Result<&JSONSchema, String> {
    let kind_prefix = kind_to_prefix(kind).ok_or(format!("invalid kind {}", kind))?;
    SCHEMA_VALIDATORS.get(kind_prefix).ok_or(format!("unsupported kind {}", kind))
}

#[inline]
fn osdu_schema_ref_to_path(url: &Url) -> Option<String> {
    let url_str = url.as_str();
    if url_str.starts_with(BASE_OSDU_SCHEMA_URL) {
        return Some(format!("{}{}",BASE_OSDU_SCHEMA_LOCAL,&url_str[BASE_OSDU_SCHEMA_URL.len()..]));
    }
    return None
}


impl SchemaResolver for LocalSchemaResolver {
    fn resolve(&self, _: &Value, url: &Url, _original_reference: &str) -> Result<Arc<Value>, SchemaResolverError> {
        let path = osdu_schema_ref_to_path(url).unwrap();
        let f = std::fs::File::open(path)?;
        let document: Value = serde_json::from_reader(f)?;
        Ok(Arc::new(document))
    }
}

fn load_schema_validator(filename: &str) -> JSONSchema {
    // let file_path = "res/schemas/AbstractContact.1.0.0.json";
    let file_path = format!("res/schemas/{}", filename);
    println!("load schema {}", file_path);

    let f = std::fs::File::open(file_path).unwrap();
    let schema: Value = serde_json::from_reader(f).unwrap();
    JSONSchema::compile(&schema).unwrap()
    // let file_path = "res/schemas/work-product-component/WellLog.1.2.0.json";

    // let compiled = jsonschema::CompilationOptions::default()
    //     .with_resolver(LocalSchemaResolver{})
    //     .compile(&schema).expect("A valid schema");

    // could also with_document
}

pub fn print_message() {
    println!("hello");
}

lazy_static::lazy_static! {
    static ref SCHEMA_VALIDATORS: HashMap<&'static str, JSONSchema> = {
        SUPPORTED_SCHEMAS.iter().map(|f| (f.kind_prefix(), load_schema_validator(f.schema_file()))).collect()
    };

    static ref COUNT: usize = SCHEMA_VALIDATORS.len();
}


#[cfg(test)]
mod test {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_osdu_schema_ref_to_path() {
        let actual = osdu_schema_ref_to_path(
            &Url::parse("https://schema.osdu.opengroup.org/json/foo/bar.json").unwrap()

        ).unwrap();
        assert_eq!("res/schemas/foo/bar.json".to_string(), actual);
    }

    #[test]
    fn test_invalid_osdu_schema_ref_to_path() {
        assert!(osdu_schema_ref_to_path(
            &Url::parse("https://unknown/json/foo/bar.json").unwrap()
            ).is_none()
        );
    }

    #[test]
    fn test_kind_to_prefix() {
        assert_eq!(
            kind_to_prefix("osdu:wks:work-product-component--WellLog:1.2.0").unwrap(),
            "work-product-component--WellLog:1.2.0"
        );
    }

    #[test]
    fn test_validator_from_kind() {
        // assert!(validator_from_kind("osdu:wks:work-product-component--WellLog:1.2.0").is_ok());
        // assert!(validator_from_kind("a:b:work-product-component--WellLog:1.1.0").is_ok());
        assert!(validator_from_kind("a:b:c:1.1.0").is_err());
    }


    #[test]
    fn test_foo() {
        // let tt: HashMap<_, String> = SUPPORTED_SCHEMAS.iter().map(|f| (f.kind_suffix, f.schema_file.to_string())).collect();
        // let to_validate = json!({"EmailAddress": "yaya@ff.fr","OrganisationID": "123456789","Name": "yaya"});
        // let result = compiled.validate(&to_validate);
        // if let Err(errors) = result {
        //     for error in errors {
        //         println!("Validation error: {}", error);
        //         println!("Instance path: {}", error.instance_path);
        //     }
        // }

    // let kind = v.get("kind").unwrap();
    // let kind2 = &v["kind"];

        // assert_eq!(*COUNT, 3);
        // println!("The map has {} entries.", *COUNT);
    }
}