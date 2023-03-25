use serde_json::Value;
use std::net::TcpListener;

#[tokio::test]
async fn health_check_test() {
    let app_address = spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/health_z", app_address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(
        response.status().is_success(),
        "should success, actual" // format!("should success, actual {:?}", response.status().as_u16())
    );
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn error_test() {
    let app_address = spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/error", app_address))
        .send()
        .await
        .expect("Failed to execute request.");

    // println!("RESPONSE => {:?}", response);

    assert_eq!(response.status().as_u16(), 500);
    // let text = response.text().await.unwrap();
    // println!("TEXT => {}", text);
    // assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn get_well_log_test() {
    let app_address = spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get(format!(
            "{}/welllogs/osdu:work-product-component--WellLog:blablabla",
            app_address
        ))
        .header("data-partition-id", "dp")
        .header("authorization", "Bearer T0k3n")
        .send()
        .await
        .expect("Failed to execute request.");

    let status = response.status().as_u16();
    let text = response.text().await.unwrap();
    assert_eq!(status, 200, "{}", text); //.is_success());

    let value = serde_json::from_str::<Value>(text.as_str());
    assert!(value.is_ok());
    let record_id = value
        .unwrap()
        .as_object()
        .unwrap()
        .get("id")
        .unwrap()
        .as_str()
        .unwrap()
        .to_string();

    assert_eq!("osdu:work-product-component--WellLog:blablabla", record_id);
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("fail to bond random port");
    let port = listener.local_addr().unwrap().port();
    let server_url = format!("http://127.0.0.1:{}", port);
    let server = wdms2prod::build_and_start_server(
        listener,
        Some(wdms2prod::config::Config {
            enable_logging: false,
            host_core_storage: server_url.clone(),
            ..Default::default()
        }),
    )
    .expect("fail to start server");
    let _ = tokio::spawn(server);
    server_url
}
