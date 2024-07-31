#[tokio::test]
async fn health_check_test() {
    let client = reqwest::Client::new();

    let response = client
        .get("http://127.0.0.1:8000/v1/healthcheck")
        .send()
        .await
        .expect("Failed to process request");

    assert!(response.status().is_success());
    assert_eq!(Some(14), response.content_length());
}
