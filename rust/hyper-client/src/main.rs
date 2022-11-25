use hyper::{body::HttpBody as _, Client, Uri};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let url = std::env::args()
        .skip(1)
        .next()
        .expect("Missing required CLI argument: URL");

    let client = Client::new();

    // Make a GET /ip to 'http://httpbin.org'
    let res = client.get(url).await.expect("could not ");

    // And then, if the request gets a response...
    println!("status: {}", res.status());

    // Concatenate the body stream into a single buffer...
    let buf = hyper::body::to_bytes(res)
        .await
        .expect("Could not read body");

    println!("body: {:?}", buf);
}
