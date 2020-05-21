/// An HTTP version.
///
/// ```
/// use book_web_server::Version;
///
/// assert_eq!(Version::new("HTTP/1.1"), Ok(("", Version::OneDotOne)));
/// ```
#[derive(Debug, PartialEq)]
pub enum Version {
    OneDotOne,
}

impl Version {
    pub fn new(s: &str) -> nom::IResult<&str, Self> {
        use nom::{bytes::complete::tag, combinator::map};

        let (s, _) = tag("HTTP/")(s)?;

        let (s, version) = map(tag("1.1"), |s| match s {
            "1.1" => Self::OneDotOne,
            _ => unreachable!(),
        })(s)?;

        Ok((s, version))
    }
}
