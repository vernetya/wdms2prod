// #![feature(test)]
// extern crate test;

mod record;
pub mod version;
mod validation;

use version::SemVer;
use serde_json::{Map, Value, json};
use record::Record;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_web::middleware::Logger;

// fn main() {
//     let t: SemVer = "1.0.6".parse().unwrap();
//
//     let f = std::fs::File::open("res/examples/wellLog_v3_120.json").unwrap();
//     let document: Value = serde_json::from_reader(f).unwrap();
//     let _record = Record::new_checked(document).unwrap();
//     println!("Now {:?} will print!", t);
// }

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}


// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     std::env::set_var("RUST_LOG", "actix_web=info");
//     env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
//
//     HttpServer::new(|| {
//         App::new()
//             .wrap(Logger::default())
//             .service(hello)
//             .service(parse_kind)
//             .service(get_well_log)
//     })
//         .bind(("127.0.0.1", 8012))?
//         .run()
//         .await
// }

#[post("/kind")]
async fn parse_kind(req_body: String) -> impl Responder {
    match req_body.parse::<SemVer>() {
        Ok(v) => HttpResponse::Ok().body(v.to_string()),
        Err(e) => HttpResponse::BadRequest().body(e)
    }
}

#[get("/welllog")]
async fn get_well_log() -> impl Responder {
    let value = json!(
        {
            "acl": {
                "viewers": [
                    "data.default.viewers@opendes.com"
                ],
                "owners": [
                    "data.default.owners@opendes.com"
                ]
            },
            "data": {
                "Curves": [
                    {
                        "CurveID": "GR_ID",
                        "Mnemonic": "GR",
                        "CurveUnit": "opendes:reference-data--UnitOfMeasure:m:",
                        "LogCurveFamilyID": "opendes:reference-data--LogCurveFamily:GammaRay:",
                        "NumberOfColumns": 1,
                        "CurveDescription": "Gamma Ray",
                        "CurveSampleTypeID": "opendes:reference-data--CurveSampleType:float:"
                    },
                    {
                        "CurveID": "POR_ID",
                        "Mnemonic": "NPOR",
                        "CurveUnit": "opendes:reference-data--UnitOfMeasure:m:",
                        "LogCurveFamilyID": "opendes:reference-data--LogCurveFamily:NeutronPorosity:",
                        "NumberOfColumns": 1,
                        "CurveDescription": "Neutron Porosity",
                        "CurveSampleTypeID": "opendes:reference-data--CurveSampleType:float:"
                    },
                    {
                        "CurveID": "Bulk Density",
                        "Mnemonic": "RHOB",
                        "CurveUnit": "opendes:reference-data--UnitOfMeasure:m:",
                        "LogCurveFamilyID": "opendes:reference-data--LogCurveFamily:BulkDensity:",
                        "CurveSampleTypeID": "opendes:reference-data--CurveSampleType:float:"
                    }
                ],
                "WellboreID": "opendes:master-data--Wellbore:123456:",
                "CreationDateTime": "2013-03-22T11:16:03Z",
                "VerticalMeasurement": {
                    "VerticalMeasurement": 2680.5,
                    "VerticalMeasurementPathID": "opendes:reference-data--VerticalMeasurementPath:MD:",
                    "VerticalMeasurementUnitOfMeasureID": "opendes:reference-data--UnitOfMeasure:ft:"
                },
                "TopMeasuredDepth": 12345.6,
                "BottomMeasuredDepth": 13856.2,
                "Name": "welllogName",

                "SamplingDomainTypeID": "opendes:reference-data--WellLogSamplingDomainType:Depth:",
                "IsRegular": true,
                "LogRemark": "example data for API documentation",
                "SamplingInterval": 0.1

            },
            "id": "opendes:work-product-component--WellLog:123456",
            "kind": "osdu:wks:work-product-component--WellLog:1.2.0",
            "legal": {
                "legaltags": [
                    "legaltags"
                ],
                "otherRelevantDataCountries": [
                    "US",
                    "FR"
                ]
            },
            "meta": [{
                    "kind": "Unit",
                    "name": "ft",
                    "persistableReference": "{\"scaleOffset\":{\"scale\":0.3048,\"offset\":0.0},\"symbol\":\"ft\",\"baseMeasurement\":{\"ancestry\":\"Length\",\"type\":\"UM\"},\"type\":\"USO\"}",
                    "propertyNames": [
                        "TopMeasuredDepth",
                        "BottomMeasuredDepth",
                        "SamplingInterval"
                    ],
                    "propertyValues": [
                        "ft"
                    ]
                }, {
                    "kind": "DateTime",
                    "name": "datetime",
                    "persistableReference": "{\"format\":\"yyyy-MM-ddTHH:mm:ssZ\",\"timeZone\":\"UTC\",\"type\":\"DTM\"}",
                    "propertyNames": [
                        "dateModified",
                        "dateCreated"
                    ]
                }
            ]
        }
    );

    HttpResponse::Ok()
        .content_type("application/json")
        .body(serde_json::to_string(&value).unwrap())
}
