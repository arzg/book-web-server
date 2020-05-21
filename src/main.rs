use std::net::TcpStream;

fn main() -> anyhow::Result<()> {
    use std::net::TcpListener;

    let listener = TcpListener::bind("127.0.0.1:7878")?;

    for stream in listener.incoming() {
        let stream = stream?;

        // We don’t want to stop the server if an error occurs, so just ignore it and continue.
        let _ = handle_connection(stream);
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

    let (status_line, filename) = if request.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string(filename)?;
    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes())?;
    stream.flush()?;

    Ok(())
}
