use std::io::{Read, Write};

fn main() {
    // eprintln!("connecting...");
    let mut stream = std::net::TcpStream::connect("34.223.124.45:80").unwrap();
    let req = b"GET / HTTP/1.1\r\nHost: neverssl.com\r\n\r\n";

    // eprintln!("writing request to socket...");
    stream.write_all(req).expect("could not write to socket");
    // eprintln!("wrote request to socket!");

    // eprintln!("reading response...");
    let mut response = Vec::new();

    let mut buffer = [0u8; 4096];

    let mut header = String::new();
    let mut body = String::new();

    loop {
        let read = stream
            .read(&mut buffer)
            .expect("could not read from TCP stream");

        if read == 0 {
            break;
        }
        // eprintln!("read {read} response bytes");

        response.extend_from_slice(&buffer[0..read]);

        let text = String::from_utf8_lossy(&response);
        if let Some((head, rest)) = text.split_once("\r\n\r\n") {
            header = head.to_string();
            // eprintln!("got header: {head}\n----");
            body = rest.to_string();
            break;
        }
    }

    eprintln!("response retrieved:\n\nheader:\n{header}\n---------");
    eprintln!("partial body:\n{body}\n------");

    // Read until end of stream.
    loop {
        let read = match stream.read(&mut buffer) {
            Ok(r) => r,
            Err(err) => match err.kind() {
                std::io::ErrorKind::BrokenPipe => {
                    break;
                }
                _ => {
                    panic!("could not read from socket: {err}");
                }
            },
        };

        if read == 0 {
            break;
        }
        // eprintln!("read {read} response bytes");

        let part_str = String::from_utf8_lossy(&buffer[0..read]);

        body.push_str(part_str.as_ref());
        if part_str.ends_with("\r\n\r\n") {
            break;
        }
    }

    eprintln!("full body:\n{body}\n-------");

    eprintln!("DONE");
}
