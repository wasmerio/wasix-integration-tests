fn main() -> Result<(), ureq::Error> {
    let url = std::env::args()
        .skip(1)
        .next()
        .unwrap_or_else(|| "http://example.com".to_string());

    eprintln!("Fetching url: {url}");

    let body: String = ureq::get(&url)
        // .set("Example-Header", "header value")
        .call()?
        .into_string()?;
    eprintln!("BODY:\n----------\n{body}\n---------\n");
    Ok(())
}
