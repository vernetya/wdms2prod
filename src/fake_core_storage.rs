use actix_web::{get, http::header::ContentType, web, HttpResponse};
use serde_json::json;

#[get("api/v2/storage/records/{record_id}")]
pub async fn get_record(path: web::Path<String>) -> HttpResponse {
    let r_id = path.into_inner();
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
            "id": r_id,
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
        .content_type(ContentType::json())
        .body(value.to_string())
}
