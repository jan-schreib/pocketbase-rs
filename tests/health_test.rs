use pocketbase_rs::Client;

#[tokio::test]
async fn health_test() {
    let client = Client::new("http://127.0.0.1:8090", None).unwrap();
    let health = client.health().await.unwrap();

    assert_eq!(health.code, 200);
}
