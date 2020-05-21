/// An HTTP method.
///
/// ```
/// use book_web_server::Method;
///
/// assert_eq!(Method::new("GET"), Ok(("", Method::Get)));
/// ```
#[derive(Debug, PartialEq)]
pub enum Method {
    Get,
}

impl Method {
    pub fn new(s: &str) -> nom::IResult<&str, Self> {
        use nom::{bytes::complete::tag, combinator::map};

        map(tag("GET"), |_| Self::Get)(s)
    }
}
