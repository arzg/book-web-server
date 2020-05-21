use std::fmt;

/// An HTTP version.
///
/// ```
/// use book_web_server::Version;
///
/// assert_eq!(Version::new("HTTP/1.1"), Ok(("", Version::OneDotOne)));
/// assert_eq!(format!("{}", Version::OneDotOne), "HTTP/1.1".to_string());
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

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OneDotOne => f.write_str("HTTP/1.1"),
        }
    }
}
