use http_client;

#[test]
fn instance_test() {
  http_client::http::HttpClient::get("https://simonkov.dev/test-path", http_client::http::RequestOptions {
    ssl: None,
    params: None,
    headers: None,
    timeout: None,
    content_type: None
  });
}
