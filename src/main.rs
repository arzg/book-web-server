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
    use std::io::Read;

    let mut buffer = [0; 512];
    stream.read(&mut buffer)?;

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    Ok(())
}
