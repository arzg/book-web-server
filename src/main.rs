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
    use std::io::{Read, Write};

    let mut response = [0; 512];
    stream.read(&mut response)?;

    let response = "HTTP/1.1 200 OK\r\n\r\n";

    stream.write(response.as_bytes())?;
    stream.flush()?;

    Ok(())
}
