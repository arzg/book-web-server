/// An HTTP request.
///
/// ```
/// use book_web_server::{Method, Request, Uri, Version};
///
/// assert_eq!(
///     Request::new("GET / HTTP/1.1\r\n\r\n"),
///     Ok(Request {
///         method: Method::Get,
///         uri: Uri::root(),
///         version: Version::OneDotOne,
///         headers: Vec::new(),
///         body: "",
///     })
/// );
/// ```
#[derive(Debug, PartialEq)]
pub struct Request<'a> {
    pub method: crate::Method,
    pub uri: crate::Uri<'a>,
    pub version: crate::Version,
    pub headers: Vec<crate::Header<'a>>,
    pub body: &'a str,
}

impl<'a> Request<'a> {
    pub fn new(s: &'a str) -> Result<Self, nom::Err<(&str, nom::error::ErrorKind)>> {
        use nom::{multi::many0, sequence::tuple};

        let (s, (method, _, uri, _, version, _, headers, _)) = tuple((
            crate::Method::new,
            crate::space,
            crate::Uri::new,
            crate::space,
            crate::Version::new,
            crate::crlf,
            many0(crate::Header::new),
            crate::crlf,
        ))(s)?;

        Ok(Self {
            method,
            uri,
            version,
            headers,
            body: s,
        })
    }
}
