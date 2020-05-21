use std::fmt;

/// An HTTP header.
///
/// ```
/// use book_web_server::Header;
///
/// assert_eq!(
///     Header::new("Content-Length: 1000\r\n"),
///     Ok((
///         "",
///         Header {
///             field: "Content-Length",
///             val: "1000",
///         }
///     ))
/// );
///
/// assert_eq!(
///     format!(
///         "{}",
///         Header {
///             field: "Content-Length",
///             val: "100",
///         }
///     ),
///     "Content-Length: 100".to_string()
/// );
/// ```
#[derive(Debug, PartialEq)]
pub struct Header<'a> {
    pub field: &'a str,
    pub val: &'a str,
}

impl<'a> Header<'a> {
    pub fn new(s: &'a str) -> nom::IResult<&'a str, Self> {
        use nom::bytes::complete::{tag, take_until, take_while1};

        let (s, field) = take_while1(|c: char| c.is_ascii_alphanumeric() || c == '-')(s)?;

        let (s, _) = tag(":")(s)?;
        let (s, _) = crate::space(s)?;

        let (s, val) = take_until("\r\n")(s)?;

        let (s, _) = crate::crlf(s)?;

        Ok((s, Self { field, val }))
    }
}

impl fmt::Display for Header<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.field, self.val)
    }
}
