use std::net::TcpStream;

fn main() -> anyhow::Result<()> {
    use std::net::TcpListener;

    let listener = TcpListener::bind("127.0.0.1:7878")?;

    for stream in listener.incoming() {
        let stream = stream?;
        handle_connection(stream)?;
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream) -> anyhow::Result<()> {
    use std::{
        fs,
        io::{Read, Write},
    };

    let mut request = [0; 512];
    stream.read(&mut request)?;

    let get = b"GET / HTTP/1.1\r\n";

    if request.starts_with(get) {
        let contents = fs::read_to_string("hello.html")?;
        let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);

        stream.write(response.as_bytes())?;
        stream.flush()?;
    } else {
        let status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
        let contents = fs::read_to_string("404.html")?;

        let response = format!("{}{}", status_line, contents);

        stream.write(response.as_bytes())?;
        stream.flush()?;
    }

    Ok(())
}
