mod common;

use serde_json::{json, Value};

#[tokio::test]
async fn create_tree_with_200_status() {
    let server = common::spawn_server().await;
    let client = reqwest::Client::new();
    let url = format!("{}/v1/trees", server.addr);

    let body = json!({
    "latitude": 10.93,
    "longitude": 12.99,
    "address": "12 Main Street",
    "dbh": 9.8,
    "species": "Oak",
    "description": "Test Description"
    });

    let response = client
        .post(&url)
        .header("Content-Type", "application/json")
        .body(body.to_string())
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn create_tree_with_422_status() {
    let server = common::spawn_server().await;
    let client = reqwest::Client::new();
    let url = format!("{}/v1/trees", server.addr);

    let test_cases: Vec<(Value, &str)> = vec![
        (
            json!({
                "longitude": 12.99,
                "address": "12 Main Street",
                "dbh": 9.7,
                "species": "Oak",
                "description": "Test Description"
            }),
            "missing the latitude",
        ),
        (
            json!({
                "latitude": 10.93,
                "address": "12 Main Street",
                "dbh": 9.7,
                "species": "Oak",
                "description": "Test Description"
            }),
            "missing the longitude",
        ),
        (
            json!({
                "latitude": 10.93,
                "longitude": 12.99,
                "dbh": 9.7,
                "species": "Oak",
                "description": "Test Description"
            }),
            "missing the address",
        ),
        (
            json!({
                "latitude": 10.93,
                "longitude": 12.99,
                "address": "12 Main Street",
                "species": "Oak",
                "description": "Test Description"
            }),
            "missing the dbh",
        ),
        (
            json!({
                "latitude": 10.93,
                "longitude": 12.99,
                "address": "12 Main Street",
                "dbh": 9.7,
                "description": "Test Description"
            }),
            "missing the species",
        ),
        (
            json!({
                "latitude": 10.93,
                "longitude": 12.99,
                "address": "12 Main Street",
                "dbh": 9.7,
                "species": "Oak"
            }),
            "missing the description",
        ),
        (
            json!({
                "": ""
            }),
            "missing all data",
        ),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&url)
            .header("Content-Type", "application/json")
            .body(invalid_body.to_string())
            .send()
            .await
            .expect("Failed to execute request");

        // 422 Unprocessable Content indicates that the server understands the content type
        // and syntax of the request, but is unable to process

        // create_tree() tries to deserialize the payload based on the TreePayload struct
        assert_eq!(
            422,
            response.status().as_u16(),
            "The API did not fail with 422 status when the payload was {}",
            error_message
        );
    }
}
