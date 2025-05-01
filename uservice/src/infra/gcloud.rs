use http_body_util::BodyExt;
use http_body_util::Empty;
use hyper::body::Bytes;
use hyper::http::Request;
use hyper_util::client::legacy::Client;
use hyper_util::client::legacy::connect::HttpConnector;
use std::convert::Infallible;

/// fetch the Google cloud project-id
/// see <https://cloud.google.com/compute/docs/metadata/predefined-metadata-keys>
pub async fn fetch_project_id() -> Result<String, Infallible> {
    let uri = "http://metadata.google.internal/computeMetadata/v1/project/project-id"
        .parse::<hyper::Uri>()
        .expect("hardcoded URI must be valid");

    let client = Client::builder(hyper_util::rt::TokioExecutor::new()).build(HttpConnector::new());

    let req = Request::builder()
        .uri(uri)
        .header("Metadata-Flavor", "Google")
        .body(Empty::<Bytes>::new())
        .unwrap();

    let response = client.request(req).await.unwrap();
    let body = response.into_body();
    let body = body.collect().await.unwrap();
    let body = body.to_bytes().to_vec();

    let project_id = String::from_utf8(body).unwrap();

    Ok(project_id)
}
