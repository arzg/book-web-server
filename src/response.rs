use std::fmt;

/// An HTTP response.
///
/// ```
/// use book_web_server::{Header, Response, Status, Version};
///
/// assert_eq!(
///     format!(
///         "{}",
///         Response {
///             version: Version::OneDotOne,
///             status: Status::Ok,
///             headers: vec![Header {
///                 field: "Content-Length",
///                 val: "13",
///             }],
///             body: "Hello, world!",
///         }
///     ),
///     "HTTP/1.1 200 OK\r\nContent-Length: 13\r\n\r\nHello, world!".to_string(),
/// );
/// ```
#[derive(Debug)]
pub struct Response<'header, 'body> {
    pub version: crate::Version,
    pub status: crate::Status,
    pub headers: Vec<crate::Header<'header>>,
    pub body: &'body str,
}

impl fmt::Display for Response<'_, '_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}\r\n", self.version, self.status)?;

        for header in &self.headers {
            write!(f, "{}\r\n", header)?;
        }
        f.write_str("\r\n")?;

        f.write_str(self.body)?;

        Ok(())
    }
}
