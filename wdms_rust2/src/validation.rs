use jsonschema::{JSONSchema, SchemaResolver, SchemaResolverError, Draft, ValidationError};
use serde_json::{Map, Value, json};
use std::sync::Arc;
use url::Url;

struct LocalSchemaResolver;



fn osdu_schema_ref_to_local_path(url: &Url) -> Option<String> {
    const SCHEMA_LOCAL_BASE_DIR: &str = "res/schemas/";
    const BASE_OSDU_SCHEMA_URL: &str = "https://schema.osdu.opengroup.org/json/";

    let url_str = url.as_str();
    if url_str.starts_with(BASE_OSDU_SCHEMA_URL) {
        return Some(format!("{}{}", SCHEMA_LOCAL_BASE_DIR, &url_str[BASE_OSDU_SCHEMA_URL.len()..]));
    }
    return None
}

impl SchemaResolver for LocalSchemaResolver {
    fn resolve(&self, _: &Value, url: &Url, _original_reference: &str) -> Result<Arc<Value>, SchemaResolverError> {
        let path = osdu_schema_ref_to_local_path(url)
            .ok_or(std::io::Error::new(std::io::ErrorKind::InvalidData, "invalid ref"))?;
        let f = std::fs::File::open(path)?;
        let document: Value = serde_json::from_reader(f)?;
        Ok(Arc::new(document))
    }
}

pub fn load_schema_validator(filename: &str) -> Result<JSONSchema, String> {
    // let file_path = "res/schemas/AbstractContact.1.0.0.json";
    let file_path = format!("res/schemas/{}", filename);
    println!("load schema {}", file_path);

    let f = std::fs::File::open(file_path).or_else(|e| Err(e.to_string()))?;
    let schema: Value = serde_json::from_reader(f).or_else(|e| Err(e.to_string()))?;
    JSONSchema::options()
        .with_draft(Draft::Draft7)
        .with_resolver(LocalSchemaResolver{})
        .compile(&schema)
        .or_else(|e| Err(e.to_string()))
}

pub fn validation_error_descr(error: &ValidationError) -> String {
    format!("{} ({}): {}", error.instance_path, error.schema_path, error)
}


#[cfg(test)]
mod test {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_osdu_schema_ref_to_path() {
        let actual = osdu_schema_ref_to_local_path(
            &Url::parse("https://schema.osdu.opengroup.org/json/foo/bar.json").unwrap()

        ).unwrap();
        assert_eq!("res/schemas/foo/bar.json".to_string(), actual);
    }

    #[test]
    fn test_invalid_osdu_schema_ref_to_path() {
        assert!(osdu_schema_ref_to_local_path(
            &Url::parse("https://unknown/json/foo/bar.json").unwrap()
            ).is_none()
        );
    }
}
