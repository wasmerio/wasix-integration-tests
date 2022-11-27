fn main() {
    let url = std::env::args()
        .skip(1)
        .next()
        .unwrap_or_else(|| "http://example.com".to_string());

    println!("Fetching url: {url}");

    let res = ureq::get(&url)
        // .set("Example-Header", "header value")
        .call().unwrap();
    println!("got response");
    let body: String = res.into_string().unwrap();
    println!("BODY:\n----------\n{body}\n---------\n");
}
