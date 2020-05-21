use tokio::{net::TcpStream, prelude::*};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use tokio::{net::TcpListener, stream::StreamExt};

    let mut listener = TcpListener::bind("127.0.0.1:7878").await?;
    let mut incoming = listener.incoming();

    while let Some(stream) = incoming.next().await {
        let stream = stream?;

        tokio::spawn(handle_connection(stream));
    }

    Ok(())
}

async fn handle_connection(mut stream: TcpStream) -> anyhow::Result<()> {
    use book_web_server::{Method, Request, Response, Status, Uri, Version};
    use tokio::fs;

    let mut request = [0; 512];
    let _ = stream.read(&mut request).await?;

    let request = String::from_utf8_lossy(&request);
    let request = Request::new(&request)?;

    let (status, filename) = if request.method == Method::Get && request.uri == Uri::root() {
        (Status::Ok, "hello.html")
    } else {
        (Status::NotFound, "404.html")
    };

    let body = fs::read_to_string(filename).await?;

    let response = Response {
        version: Version::OneDotOne,
        status,
        headers: Vec::new(),
        body: &body,
    };

    stream.write_all(response.to_string().as_bytes()).await?;
    stream.flush().await?;

    Ok(())
}
